use std::fmt::Error;
use env_logger::init;
use ethers_core::types::Res;
use log::{error, warn, info, debug};
use tungstenite::{connect, Message};
use url::Url;
use serde::{Serialize, Deserialize};
use storage::Storage;
use storage::WriteResult;


#[derive(Serialize, Deserialize)]
pub struct DecidedFilter {
    from: u32,
    to: u32,
    role: String,
    publicKey: String,
}


#[derive(Serialize, Deserialize)]
pub struct DecidedItemMessage {
    MsgType: u32,
    Height: u32,
    Round: u32,
    Identifier: String,
    Data: String,
}


#[derive(Serialize, Deserialize)]
pub struct DecidedItem {
    Signature: String,
    Signers: Vec<u32>,
    Message: DecidedItemMessage,
}


#[derive(Serialize, Deserialize)]
pub struct DecidedResponse {
    filter: DecidedFilter,
    data: Vec<DecidedItem>,
}


struct SyncDecided {
    ssv_ws_endpoint: String,
    storage: Box<dyn Storage>,
}


impl SyncDecided {
    pub fn new(ssv_ws_endpoint: String, storage: Box<dyn Storage>) -> Self {
        return Self {
            ssv_ws_endpoint,
            storage,
        };
    }

    pub async fn start(&self) -> Result<(), String> {
        let (mut socket, response) =
            connect(Url::parse("ws://106.14.249.226:15000/stream").unwrap()).expect("Can't connect");
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
                                    self.save_decided_response(decided_response);
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
            match self.storage.add_decided(
                decided_response.filter.role.clone(),
                decided_response.filter.publicKey.clone(),
                decided_item.Signature,
                decided_item.Message.Height,
                decided_item.Message.Round,
                decided_item.Message.Identifier,
                decided_item.Message.MsgType,
            ) {
                WriteResult::UniqueViolation => {
                    warn!("UniqueViolation");
                },
                WriteResult::Normal => {},
                WriteResult::Other(details) => {
                    warn!("write to mysql error: {}", details)
                }
            }
            for operator_id in decided_item.Signers {
                match self.storage.add_operator_decided_record(
                    operator_id,
                    decided_response.filter.publicKey.clone(),
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
    }
}


#[cfg(test)]
mod test {
    use log::error;
    use storage::MysqlStorage;
    use crate::sync_decided::SyncDecided;

    #[tokio::test]
    async fn test_sync_decided() {
        env_logger::builder().filter_level(log::LevelFilter::Info).init();
        error!("starting...");
        let ws = "ws://106.14.249.226:15000/stream".to_string();
        let connect_url = "mysql://root:wonders,1@192.168.1.128:3308/ssv_alert_service_5".to_string();
        let storage = MysqlStorage::new(connect_url);
        let sync_decided = SyncDecided::new(ws, Box::new(storage));
        loop {
            let result = sync_decided.start().await;
        }
    }
}