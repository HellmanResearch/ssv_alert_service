use std::fmt::Error;
use std::time::Duration;
use actix_web::web::to;
use env_logger::init;
use ethers_core::types::Res;
use log::{error, warn, info, debug};
use tungstenite::{connect, Message};
use url::Url;
use serde::{Serialize, Deserialize};
use tokio;
use storage::{MysqlStorage, Storage};
use storage::WriteResult;
use clap::Parser;
use toml;
use std::fs;
use std::sync::Arc;
// use serde_derive::Deserialize;



#[derive(Serialize, Deserialize)]
pub struct DecidedFilter {
    from: u32,
    to: u32,
    role: String,
    publicKey: String,
}


#[derive(Serialize, Deserialize, Clone)]
pub struct DecidedItemMessage {
    MsgType: u32,
    Height: u32,
    Round: u32,
    Identifier: String,
    Data: String,
}


#[derive(Serialize, Deserialize, Clone)]
pub struct DecidedItem {
    Signature: String,
    Signers: Vec<u32>,
    Message: DecidedItemMessage,
}


#[derive(Serialize, Deserialize)]
pub struct DecidedResponse {
    pub filter: DecidedFilter,
    pub data: Vec<DecidedItem>,
}


struct SyncDecided {
    ssv_ws_endpoint: String,
    storage: MysqlStorage,
}


impl SyncDecided {
    pub fn new(ssv_ws_endpoint: String, database_connect_url: String) -> Self {
        let storage = MysqlStorage::new(database_connect_url);
        return Self {
            ssv_ws_endpoint,
            storage,
        };
    }

    pub async fn start(&self) -> Result<(), String> {
        let (mut socket, response) =
            connect(Url::parse(self.ssv_ws_endpoint.as_str()).unwrap()).expect("Can't connect");
        info!("connected to ssv server {}", self.ssv_ws_endpoint);
        loop {
            match socket.read_message() {
                Ok(message) => {
                    debug!("received a message from ssv node: {}", message);
                    match message {
                        Message::Text(message_sting) => {
                            let result: serde_json::error::Result<DecidedResponse> = serde_json::from_str(message_sting.as_str());
                            match result {
                                Ok(decided_response) => {
                                    self.save_decided_response(decided_response );
                                }
                                Err(error) => {
                                    error!("deserialize decided message error: {}", error);
                                }
                            };
                        }
                        _ => {
                            warn!("get a unknown message: {}", message)
                        }
                    }
                }
                Err(error) => {
                    socket.close(None);
                    warn!("read_message error: {}", error);
                    return Err(format!("read_message error: {}", error))
                }
            }
        }
    }

    fn save_decided_response(&self, decided_response: DecidedResponse) {
        for decided_item in decided_response.data {

        }
    }

}

fn save_decided_response(storage: &dyn Storage, decided_response: DecidedResponse) {
    for decided_item in decided_response.data {
        save_decided_item(storage, decided_item, true, decided_response.filter.role.clone(), decided_response.filter.publicKey.clone())
        // match storage.add_decided(
        //     decided_response.filter.role.clone(),
        //     decided_response.filter.publicKey.clone(),
        //     decided_item.Signature,
        //     decided_item.Message.Height,
        //     decided_item.Message.Round,
        //     decided_item.Message.Identifier,
        //     decided_item.Message.MsgType,
        // ) {
        //     WriteResult::UniqueViolation => {
        //         if recursion == false {
        //
        //         }
        //         warn!("UniqueViolation");
        //     },
        //     WriteResult::Normal => {},
        //     WriteResult::Other(details) => {
        //         warn!("write to mysql error: {}", details)
        //     }
        // }
        // for operator_id in decided_item.Signers {
        //     match storage.add_operator_decided_record(
        //         operator_id,
        //         decided_response.filter.publicKey.clone(),
        //         decided_item.Message.Height,
        //         decided_item.Message.Round,
        //     ) {
        //         WriteResult::UniqueViolation => {
        //             warn!("UniqueViolation");
        //         },
        //         WriteResult::Normal => {},
        //         WriteResult::Other(details) => {
        //             panic!("write to mysql error: {}", details)
        //         }
        //     }
        // }
    }
}


fn save_decided_item(storage: &dyn Storage, decided_item: DecidedItem, exist_delete_restore: bool, role: String, validator_public_key: String) {
    match storage.add_decided(
        role.clone(),
        validator_public_key.clone(),
        decided_item.Signature.clone(),
        decided_item.Message.Height,
        decided_item.Message.Round,
        decided_item.Message.Identifier.clone(),
        decided_item.Message.MsgType,
    ) {
        WriteResult::UniqueViolation => {
            if exist_delete_restore == true {
                storage.delete_decided_by_validator_height(validator_public_key.clone(),decided_item.Message.Height);
                storage.delete_operator_decided_record_by_validator_height(validator_public_key.clone(), decided_item.Message.Height);
                save_decided_item(storage, decided_item.clone(), false, role.clone(), validator_public_key.clone());
                return;
            }
            // warn!("UniqueViolation");
            return;
        },
        WriteResult::Normal => {},
        WriteResult::Other(details) => {
            warn!("write to mysql error: {}", details)
        }
    }
    for operator_id in decided_item.Signers {
        match storage.add_operator_decided_record(
            operator_id,
            validator_public_key.clone(),
            decided_item.Message.Height,
            decided_item.Message.Round,
        ) {
            WriteResult::UniqueViolation => {
                warn!("UniqueViolation");
            },
            WriteResult::Normal => {},
            WriteResult::Other(details) => {
                panic!("write to mysql error: {}", details)
            }
        }
    }
}

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    config: String,
}

#[derive(Clone, Deserialize)]
struct ConfigDatabase {
    connect_url: String
}

#[derive(Clone, Deserialize)]
struct ConfigEth {
    eth1_http_endpoint: String,
    eth1_ws_endpoint: String
}

#[derive(Clone, Deserialize)]
struct ConfigSSV {
    contract_abi: String,
    contract_address: String,
    explorer_ws_endpoint: String
}

#[derive(Clone, Deserialize)]
struct Config {
    database: ConfigDatabase,
    ssv: ConfigSSV,
    eth: ConfigEth,
}

#[tokio::main]
async fn main() {
    env_logger::builder().filter_level(log::LevelFilter::Info).init();
    info!("starting...");

    let args: Args = Args::parse();
    let config_content = fs::read_to_string(args.config.clone()).expect(format!("Unable to read file {}", args.config).as_str());
    let config: Config = toml::from_str(config_content.as_str()).unwrap();

    // let c = Arc::new(config);

    loop {

        let config = config.clone();

        let thread = tokio::spawn(async move {

            // let cc = Arc::clone(&c);

            let ssv_ws_endpoint = config.ssv.explorer_ws_endpoint.clone();
            let connect_url = config.database.connect_url.clone();
            let storage = MysqlStorage::new(connect_url);
            // let sync_decided = SyncDecided::new(ws, connect_url);
            // let result = sync_decided.start().await;
            let (mut socket, response) =
                connect(Url::parse(ssv_ws_endpoint.as_str()).unwrap()).expect("Can't connect");
            info!("connected to ssv server {}", ssv_ws_endpoint);
            let mut count = 0;
            loop {
                count += 1;
                if count % 10000 == 0 {
                    info!("reset count: {}", count);
                    count = 0;
                }
                match socket.read_message() {
                    Ok(message) => {
                        debug!("received a message from ssv node: {}", message);
                        match message {
                            Message::Text(message_sting) => {
                                let result: serde_json::error::Result<DecidedResponse> = serde_json::from_str(message_sting.as_str());
                                match result {
                                    Ok(decided_response) => {
                                        save_decided_response(&storage, decided_response);
                                    }
                                    Err(error) => {
                                        error!("deserialize decided message error: {}", error);
                                    }
                                };
                            }
                            _ => {
                                warn!("get a unknown message: {}", message)
                            }
                        }
                    }
                    Err(error) => {
                        socket.close(None);
                        warn!("read_message error: {}", error);
                        return;
                    }
                }
            }
        });
        match thread.await {
            Ok(()) => {
                info!("thread returned OK")
            }
            Err(error) => {
                warn!("thread returned a error: {}", error)
            }
        }
        info!("interrupts sync_decided and restart");

        tokio::time::sleep(Duration::from_secs(10)).await;
    }
}