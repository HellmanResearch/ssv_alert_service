use ethers_contract::Contract;
use ethers_core::types::{Address, Bytes, Log, U256, U64, PathOrString};
use ethers_core::utils::hex;
use ethers_providers::{Provider, Ws};
// use crate::event;
// use crate::event::{Content, Event, OperatorRegistration};



pub struct OperatorRegistration {
    name: String,
    owner_address: String,
    public_key: String,
    id: u32,
    fee: u64,
}


pub struct OperatorRemoval {
    id: u32,
    owner_address: String,
}


pub enum Content {
    OperatorRegistration(OperatorRegistration),
    OperatorRemoval(OperatorRemoval)
}


pub struct Event {
    pub block_number: u64,
    pub content: Content,
}

pub struct Decoder {
    contract: Contract<Provider<Ws>>
}


impl Decoder {
    pub fn new(contract: Contract<Provider<Ws>>) -> Self {
        return Self {
            contract
        }
    }

    pub fn decode_log(&self, log: Log) -> Option<Event>{
        let block_number = log.block_number.unwrap().as_u64();
        let topic_0 = log.topics.get(0).unwrap();
        let topic_0_bytes = topic_0.as_bytes();
        let topic_0_str = hex::encode(topic_0_bytes);
        let topic_operator_registration = "26a77904793977b23eb8b2d412c486276510e0dc1966a4a2936d4bea0ff86e9d".to_string();
        let funds_deposit = "0x1b38057439950cdd5928b7bb240693801a08bf1eb5157fa97b7f9873f4e470ef".to_string();
        // if (topic_0_str == "26a77904793977b23eb8b2d412c486276510e0dc1966a4a2936d4bea0ff86e9d".to_string()) {
        //
        // }
        // let operator_registration = self.decode_operator_registration(log);


        match topic_0_str {
            topic_operator_registration => {
                let operator_registration = self.decode_operator_registration(log);
                let event = Event {
                    block_number,
                    content: Content::OperatorRegistration(operator_registration),
                };
                return Some(event)
            }
            funds_deposit => {
                return None
            }
            _ => {
                return None
            }
        }
    }

    pub fn decode_operator_registration(&self, log: Log) -> OperatorRegistration {
        let (id, name, owner_address, public_key_bytes, fee): (U64, String, Address, Bytes, U256) = self.contract
            .decode_event("operatorRegistration", log.topics, log.data).unwrap();
        return OperatorRegistration {
            id: id.as_u32(),
            name,
            owner_address: owner_address.to_string(),
            public_key: public_key_bytes.to_string(),
            fee: fee.as_u64()
        }
    }

}

