use std::ops::Add;
use ethers_core::{
    abi::Abi,
    types::{Address, H256, U256},
};
use ethers_contract::Contract;
use ethers_core::k256::elliptic_curve::weierstrass::add;
use ethers_core::types::{Bytes, Filter, Log};
use ethers_providers::{Provider, Http, Middleware};
use ethers_core::types::BlockNumber;
use crate::decode;


pub struct SSVContract {
    contract: Contract<Provider<Http>>,
    address: Address,
}


impl SSVContract {
    pub fn new(eth_http_endpoint: String, address: String, abi: String) -> Self {
        let client = Provider::<Http>::try_from(eth_http_endpoint).unwrap();
        // let address = "0xb9e155e65B5c4D66df28Da8E9a0957f06F11Bc04".parse::<Address>().unwrap();
        let address = address.parse::<Address>().unwrap();
        let abi: Abi = serde_json::from_str(abi.as_str()).unwrap();
        let contract = Contract::new(address, abi, client);
        return Self {
            contract,
            address,
        };
    }

    pub async fn getOperatorById(&self, id: u32) -> Result<(String, String, String, u32, f32, u32, bool), String> {
        match self.contract
            .method::<(u32, ), (String, Address, Bytes, U256, U256, U256, bool)>("getOperatorById", (id, )).unwrap()
            .call()
            .await {
            Ok((name, owner_address, public_key, validator_count, fee, score, is_active)) => {
                let fee_human = decode::fee_to_human(fee);
                // let base: U256 = U256([4170000000 as u64, 0, 0, 0]);
                // let balance_divided_base = fee / base;
                // let fee_human_f64 = balance_devided_base.as_u64();
                // let fee_human = fee_human_f64 as f32;
                // let fee_human = fee_human / 100.0;
                return Ok((name, owner_address.to_string(), public_key.to_string(), validator_count.as_u32(), fee_human, score.as_u32(), is_active));
            }
            Err(error) => {
                Err("call contract error".to_string())
            }
        }
    }


    pub async fn getAddressBalance(&self, address_str: String) -> Result<(f32), String> {
        match address_str.parse::<Address>() {
            Ok(address) => {
                match self.contract
                    .method::<(Address, ), U256>("getAddressBalance", (address, )).unwrap()
                    .call()
                    .await {
                    Ok(balance) => {
                        let e16: U256 = U256([1e16 as u64, 0, 0, 0]);
                        let balance_devided_16 = balance / e16;
                        let balance_human = balance_devided_16.as_u32() as f32 / 1e2;

                        // let balance_128 = balance_devided_16.as_u64();
                        // let balance_human = (balance_128 / 1e16).as_u32().as_f32 / 1e2;
                        return Ok(balance_human)
                    }
                    Err(error) => {
                        Err("call contract error".to_string())
                    }
                }
            }
            Err(error) => {
                return Err(format!("parse address error: {}", error.to_string()));
            }
        }
    }

    pub async fn getIsLiquidated(&self, address_str: String) -> Result<(bool), String> {
        match address_str.parse::<Address>() {
            Ok(address) => {
                match self.contract
                    .method::<(Address, ), bool>("isLiquidated", (address, )).unwrap()
                    .call()
                    .await {
                    Ok(is_liquidated) => {
                        return Ok(is_liquidated)
                    }
                    Err(error) => {
                        Err("call contract error".to_string())
                    }
                }
            }
            Err(error) => {
                return Err(format!("parse address error: {}", error.to_string()));
            }
        }
    }

    pub async fn get_latest_block(&self, ) -> Result<u64, String> {
        // let r = self.contract.client().get_block(BlockNumber::Latest).await;
        return match self.contract.client().get_block(BlockNumber::Latest).await {
            Ok(option_block) => {
                // let r = option_block.ok_or("option is None".to_string())
                match option_block {
                    Some(block) => {
                        // let r = block.number.ok_or("block.number is None".to_string())
                        let r = block.number.unwrap();
                        match block.number {
                            Some(block_number) => {
                                Ok(block_number.as_u64())
                            }
                            None => Err("block.number is None".to_string())
                        }
                    }
                    None => Err("block is None".to_string())
                }
            }
            Err(error) => Err("ProviderError".to_string())
        };
    }

    pub async fn get_logs(&self, from_block: u64, to_block: u64) -> Result<Vec<Log>, String> {
        let filter = Filter::new()
            .address(self.address)
            .from_block(from_block)
            .to_block(to_block);
        match self.contract.client().get_logs(&filter).await {
            Ok(logs) => {
                return Ok(logs)
            }
            Err(error_info) => {
                return Err("Failed get logs".to_string())
            }
        };
    }
}
