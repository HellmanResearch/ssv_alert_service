extern crate core;

use std::fs;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::Arc;
use std::time::Duration;
use clap::Parser;

mod update_operator;
mod update_account;
mod update_performance;
mod job;
pub mod update_event;
mod lib;
mod clear_decided;

use rand;
use job::Job;
use env_logger::Env;
use env_logger;
use log::{debug, info, warn, error};
use rand::Rng;
use serde::{Deserialize, Deserializer};

use ssv_contract;
use ssv_contract::ssv_contract::SSVContract;
use storage;
use storage::{MysqlStorage, Storage};
use update_event::update_event_task;
use toml;
use crate::clear_decided::clear_decided_task;

#[derive(Clone, Deserialize)]
pub struct ConfigDatabase {
    connect_url: String,
}

#[derive(Clone, Deserialize)]
pub struct ConfigEth {
    eth1_http_endpoint: String,
    eth1_ws_endpoint: String,
}

#[derive(Clone, Deserialize)]
pub struct ConfigSSV {
    contract_abi: String,
    contract_address: String,
    explorer_ws_endpoint: String,
    start_block_sync: u64,
}

#[derive(Clone, Deserialize)]
pub struct ConfigLog {
    level: String
}

#[derive(Clone, Deserialize)]
pub struct Config {
    database: ConfigDatabase,
    ssv: ConfigSSV,
    eth: ConfigEth,
    log: ConfigLog,
    tasks_interval: ConfigTasksInterval
}


#[derive(Clone, Deserialize)]
pub struct ConfigTasksInterval {
    update_event: u64,
    update_account: u64,
    update_operator: u64,
    update_validator: u64,
    clear_decided: u64
}


#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    config: String,
}



#[tokio::main]
async fn main() {



    let args: Args = Args::parse();
    let config_content = fs::read_to_string(args.config.clone()).expect(format!("Unable to read file {}", args.config).as_str());
    let config: Config = toml::from_str(config_content.as_str()).unwrap();

    // let config = config;
    let mut log_level = log::LevelFilter::Info;
    if config.log.level == "DEBUG" {
        log_level = log::LevelFilter::Debug;
    }else if config.log.level == "INFO" {
        log_level = log::LevelFilter::Info;
    }else if config.log.level == "WARN" {
        log_level = log::LevelFilter::Warn;
    }else if config.log.level == "ERROR" {
        log_level = log::LevelFilter::Error;
    }else{
        panic!("unknown log level: {}", config.log.level)
    }

    env_logger::builder().filter_level(log_level).init();
    info!("starting...");
    debug!("debug starting...");

    let config_event = config.clone();

    tokio::spawn(async move {
        loop {
            let config_loop = config_event.clone();
            let thread = tokio::spawn(async move {
                let connect_url = config_loop.database.connect_url;
                let eth1_http_endpoint = config_loop.eth.eth1_http_endpoint;
                let eth1_ws_endpoint = config_loop.eth.eth1_ws_endpoint;
                let contract_address = config_loop.ssv.contract_address;
                let abi = config_loop.ssv.contract_abi;
                let start_block_sync = config_loop.ssv.start_block_sync;

                let result = update_event_task(eth1_ws_endpoint,
                                               eth1_http_endpoint, contract_address,
                                               abi, connect_url,
                                               start_block_sync).await;
                match result {
                    Ok(error_count) => {
                        info!("update_event_task error count: {}", error_count)
                    }
                    Err(error_info) => {
                        error!("Failed update_event_task error_info: {}", error_info)
                    }
                }
            });
            match thread.await {
                Ok(()) => {
                },
                Err(error) => {
                    warn!("thread return error: {}", error.to_string());
                }
            };
            tokio::time::sleep(Duration::from_secs(config.tasks_interval.update_event)).await;
        }
    });

    let config_account = config.clone();

    tokio::spawn(async move {
        loop {
            let config_loop = config_account.clone();
            let thread = tokio::spawn(async {
                let connect_url = config_loop.database.connect_url;
                let eth1_http_endpoint = config_loop.eth.eth1_http_endpoint;
                let eth1_ws_endpoint = config_loop.eth.eth1_ws_endpoint;
                let contract_address = config_loop.ssv.contract_address;
                let abi = config_loop.ssv.contract_abi;

                let result = update_account::update_account_task(
                    eth1_http_endpoint.clone(), contract_address.clone(),
                    abi.clone(), connect_url.clone()).await;
                match result {
                    Ok(error_count) => {
                        info!("update_account_task error count: {}", error_count)
                    }
                    Err(error_info) => {
                        error!("Failed update_account_task error_info: {}", error_info)
                    }
                }
            });
            match thread.await {
                Ok(()) => {
                },
                Err(error) => {
                    warn!("thread return error: {}", error.to_string());
                }
            };
            tokio::time::sleep(Duration::from_secs(config.tasks_interval.update_account)).await;
        }
    });

    let config_operator = config.clone();

    tokio::spawn(async move {
        loop {
            let config_loop = config_operator.clone();
            let thread = tokio::spawn(async {
                let connect_url = config_loop.database.connect_url;
                let eth1_http_endpoint = config_loop.eth.eth1_http_endpoint;
                let eth1_ws_endpoint = config_loop.eth.eth1_ws_endpoint;
                let contract_address = config_loop.ssv.contract_address;
                let abi = config_loop.ssv.contract_abi;
                let result = update_operator::update_operator_task(
                    eth1_http_endpoint.clone(), contract_address.clone(),
                    abi.clone(), connect_url.clone()).await;
                match result {
                    Ok(error_count) => {
                        info!("Done update_operator_task error count: {}", error_count)
                    }
                    Err(error_info) => {
                        error!("Failed update_operator_task error_info: {}", error_info)
                    }
                }
            });
            match thread.await {
                Ok(()) => {
                },
                Err(error) => {
                    warn!("thread return error: {}", error.to_string());
                }
            };
            tokio::time::sleep(Duration::from_secs(config.tasks_interval.update_operator)).await;
        }
    });

    let config_clear = config.clone();

    tokio::spawn(async move {
        loop {
            let config_loop = config_clear.clone();
            let thread = tokio::spawn(async move {
                match clear_decided_task(config_loop.database.connect_url).await {
                    Ok(size) => {},
                    Err(error_info) => {
                        error!("Failed execute clear_decided_task error_info: {}", error_info)
                    }
                }
            });
            match thread.await {
                Ok(()) => {
                },
                Err(error) => {
                    warn!("thread return error: {}", error.to_string());
                }
            };
            tokio::time::sleep(Duration::from_secs(config.tasks_interval.clear_decided)).await;
        }
    });


    loop {
        tokio::time::sleep(Duration::from_secs(100));
    }
}