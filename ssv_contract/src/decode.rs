use std::ops::Add;
use ethers_contract::Contract;
use ethers_core::abi::AbiEncode;
use ethers_core::types::{Address, Bytes, Log, U256, U64, PathOrString};
use ethers_core::utils::hex;
use ethers_providers::{Provider, Ws};
use log::{error, warn, info};
// use crate::event;
// use crate::event::{Content, Event, OperatorRegistration};


#[derive(Debug)]
pub struct OperatorRegistration {
    pub name: String,
    pub owner_address: String,
    pub public_key: String,
    pub id: u32,
    pub fee_human: f32,
}


pub struct OperatorRemoval {
    id: u32,
    owner_address: String,
    block_number: u64,
    score: u32,
}

#[derive(Debug)]
pub struct ValidatorRegistration {
    pub owner_address: String,
    pub public_key: String,
    pub operator_ids: Vec<u32>,
}

pub struct ValidatorRemoval {
    owner_address: String,
    public_key: String,
}

pub enum Content {
    OperatorRegistration(OperatorRegistration),
    OperatorRemoval(OperatorRemoval),
}


pub struct Event {
    pub block_number: u64,
    pub content: Content,
}

pub struct Operator {
    pub name: String,
    pub owner_address: String,
    pub public_key: String,
    pub validator_count: u32,
    pub fee: f32,
    pub score: f32,
    pub is_active: bool,
}


pub struct Decoder {
    contract: Contract<Provider<Ws>>,
}

pub fn encode_address_to_string(address: Address) -> String {
    let buf = address.encode_hex();
    let encoded = format!("0x{}", &buf[26..]);
    return encoded;
}

impl Decoder {
    pub fn new(contract: Contract<Provider<Ws>>) -> Self {
        return Self {
            contract
        };
    }

    pub fn decode_operator_registration(&self, log: Log) -> Result<OperatorRegistration, String> {
        match self.contract.decode_event::<(U256, String, Address, Bytes, U256)>("OperatorRegistration", log.topics, log.data) {
            Ok((id, name, owner_address, public_key_bytes, fee)) => {
                let fee_human = fee_to_human(fee);
                return Ok(OperatorRegistration {
                    id: id.as_u32(),
                    name,
                    owner_address: crate::decode::encode_address_to_string(owner_address),
                    public_key: public_key_bytes.to_string(),
                    fee_human,
                });
            }
            Err(error) => {
                return Err("contract.decode_event error".to_string());
            }
        }
    }


    pub fn decode_validator_registration(&self, log: Log) -> Result<ValidatorRegistration, String> {
        match self.contract.decode_event::<(Address, Bytes, Vec<U256>, Vec<Bytes>, Vec<Bytes>)>("ValidatorRegistration", log.topics, log.data) {
            Ok((owner_address, public_key_bytes, operator_ids, _, _)) => {
                return Ok(ValidatorRegistration {
                    owner_address: encode_address_to_string(owner_address),
                    public_key: public_key_bytes.to_string(),
                    operator_ids: operator_ids.into_iter().map(|x| x.as_u32()).collect(),
                });
            }
            Err(error) => {
                return Err("contract.decode_event ValidatorRegistration error".to_string());
            }
        }
    }
}


pub fn fee_to_human(native: U256) -> f32 {
    let base: U256 = U256([4170000000 as u64, 0, 0, 0]);
    let balance_divided_base = native / base;
    let fee_human_f64 = balance_divided_base.as_u64();
    let fee_human_100 = fee_human_f64 as f32;
    let fee_human = fee_human_100 / 100.0;
    return fee_human
}