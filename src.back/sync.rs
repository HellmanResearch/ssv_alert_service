// use crate::ssv_contract;

use log::{info, warn};
use ssv_contract::contract::{ContractClient, get_contract};
use storage::operator::Status;

use ethers_core::types::{Filter, Log, Address};
use ethers_providers::{Provider, Http, Ws};
use ssv_contract::decode::Decoder;
use storage::{Storage, MysqlStorage, WriteResult};
use ethers_core::utils::hex;
use ethers_core::abi::Abi;

struct Sync {
    eth1_ws_endpoint: String,
    contract_address: String,
    contract_abi: String,
    decoder: Decoder,
    storage: Box<dyn Storage>,
}


impl Sync {
    pub async fn new(storage: Box<dyn Storage>, eth1_ws_endpoint: String, contract_address: String, contract_abi: String) -> Self {
        let contract = get_contract(eth1_ws_endpoint.clone(),
                                    contract_address.clone(), contract_abi.clone()).await;

        let decoder = Decoder::new(contract);

        return Self {
            storage,
            eth1_ws_endpoint,
            contract_address,
            contract_abi,
            decoder,
        };
    }

    pub async fn start(&mut self, start_block: u64) {
        // self.contract_client.connect().await;


        let mut contract_client = ContractClient::new(self.eth1_ws_endpoint.clone(), self.contract_address.clone());
        contract_client.connect().await;

        let mut receiver = contract_client.subscribe_events(start_block).await;

        loop {
            let option_log = receiver.recv().await;
            match option_log {
                Some(log) => {
                    match self.log_handler(log) {
                        Ok(()) => {

                        }
                        Err(error_info) => {

                        }
                    }
                }
                None => {
                    warn!("log is None of received")
                }
            }
        }
    }

    fn log_handler(&self, log: Log) -> Result<(), String> {
        let block_number = log.block_number.unwrap().as_u64();
        let topic_0 = log.topics.get(0).unwrap();
        let topic_0_bytes = topic_0.as_bytes();
        let topic_0_str = hex::encode(topic_0_bytes);

        let topic_operator_registration = "26a77904793977b23eb8b2d412c486276510e0dc1966a4a2936d4bea0ff86e9d".to_string();
        // let funds_deposit = "0x1b38057439950cdd5928b7bb240693801a08bf1eb5157fa97b7f9873f4e470ef".to_string();
        let topic_validator_registration = "888b4bb563730efc1c420fb22b503c3551134948a3a3dce4ffab6380e9ce5025".to_string();

        // let mut connection = establish_connection();

        if topic_0_str == topic_operator_registration {
            let event = self.decoder.decode_operator_registration(log)?;

            let fee_human = event.fee as f64 / 1e18;
            let fee_human = fee_human as f32;
            match self.storage.add_account(event.owner_address.clone(), 0.0) {
                WriteResult::Normal => {
                    return Ok(());
                }
                WriteResult::UniqueViolation => {
                    info!("UniqueViolation account public key: {}", event.public_key);
                    return Ok(());
                }
                WriteResult::Other(error_info) => {
                    return Err(error_info);
                }
            }
            match self.storage.add_operator(event.id, event.name, event.owner_address, Status::Active, 0, fee_human) {
                WriteResult::Normal => {
                    return Ok(());
                }
                WriteResult::UniqueViolation => {
                    info!("UniqueViolation operator public key: {}", event.public_key);
                    return Ok(());
                }
                WriteResult::Other(error_info) => {
                    return Err(error_info);
                }
            }
        } else if topic_0_str == topic_validator_registration {
            let event = self.decoder.decode_validator_registration(log)?;
            self.storage.add_validator(event.owner_address, event.public_key.clone());
            for operator_id in event.operator_ids {
                match self.storage.add_validator_operator(event.public_key.clone(), operator_id) {
                    WriteResult::Normal => {
                        return Ok(());
                    }
                    WriteResult::UniqueViolation => {
                        info!("UniqueViolation validator public key: {}", event.public_key);
                        return Ok(());
                    }
                    WriteResult::Other(error_info) => {
                        return Err(error_info);
                    }
                };
            }
        }

        // self.storage.add_account(event.owner_address.clone(), 0.0);
        // match self.storage.add_operator(event.id, event.name, event.owner_address, Status::Active, 0, fee_human) {
        //     Ok(()) => {}
        //     Err(error) => {
        //         panic!("add_operator error: {}", error)
        //     }
        // }
        return Ok(());
    }


    // match topic_0_str {
    //     topic_operator_registration(_) => {
    //         let event = self.decoder.decode_operator_registration(log);
    //         let fee_human = event.fee / 1e18;
    //         self.storage.add_account(event.public_key.clone());
    //         self.storage.add_operator(event.id, event.name, event.public_key, Status::Active, 0, fee_human)
    //     }
    // }

    // if (topic_0_str == "26a77904793977b23eb8b2d412c486276510e0dc1966a4a2936d4bea0ff86e9d".to_string()) {
    //
    // }
}


#[cfg(test)]
mod test {
    use log::error;
    use storage::MysqlStorage;
    use crate::sync::Sync;

    #[tokio::test]
    async fn test_sync() {
        env_logger::builder().filter_level(log::LevelFilter::Info).init();

        error!("starting...");

        let connect_url = "mysql://root:wonders,1@192.168.1.128:3308/ssv_alert_service_5".to_string();
        let storage = MysqlStorage::new(connect_url);
        let eth1_ws_endpoint = "wss://goerli.infura.io/ws/v3/3be9f3e5cd574b989f96b281f4179a06".to_string();
        let contract_address = "0xb9e155e65B5c4D66df28Da8E9a0957f06F11Bc04".to_string();
        let abi = r#"[{"inputs":[],"name":"AccountAlreadyEnabled","type":"error"},{"inputs":[],"name":"ApprovalNotWithinTimeframe","type":"error"},{"inputs":[],"name":"BelowMinimumBlockPeriod","type":"error"},{"inputs":[],"name":"BurnRatePositive","type":"error"},{"inputs":[],"name":"CallerNotOperatorOwner","type":"error"},{"inputs":[],"name":"CallerNotValidatorOwner","type":"error"},{"inputs":[],"name":"ExceedManagingOperatorsPerAccountLimit","type":"error"},{"inputs":[],"name":"FeeExceedsIncreaseLimit","type":"error"},{"inputs":[],"name":"FeeTooLow","type":"error"},{"inputs":[],"name":"NegativeBalance","type":"error"},{"inputs":[],"name":"NoPendingFeeChangeRequest","type":"error"},{"inputs":[],"name":"NotEnoughBalance","type":"error"},{"inputs":[],"name":"OperatorWithPublicKeyNotExist","type":"error"},{"inputs":[],"name":"ValidatorWithPublicKeyNotExist","type":"error"},{"anonymous":false,"inputs":[{"indexed":true,"internalType":"address","name":"ownerAddress","type":"address"}],"name":"AccountEnable","type":"event"},{"anonymous":false,"inputs":[{"indexed":true,"internalType":"address","name":"ownerAddress","type":"address"}],"name":"AccountLiquidation","type":"event"},{"anonymous":false,"inputs":[{"indexed":false,"internalType":"uint256","name":"value","type":"uint256"}],"name":"DeclareOperatorFeePeriodUpdate","type":"event"},{"anonymous":false,"inputs":[{"indexed":true,"internalType":"address","name":"ownerAddress","type":"address"},{"indexed":false,"internalType":"uint32","name":"operatorId","type":"uint32"}],"name":"DeclaredOperatorFeeCancelation","type":"event"},{"anonymous":false,"inputs":[{"indexed":false,"internalType":"uint256","name":"value","type":"uint256"}],"name":"ExecuteOperatorFeePeriodUpdate","type":"event"},{"anonymous":false,"inputs":[{"indexed":false,"internalType":"uint256","name":"value","type":"uint256"},{"indexed":true,"internalType":"address","name":"ownerAddress","type":"address"},{"indexed":true,"internalType":"address","name":"senderAddress","type":"address"}],"name":"FundsDeposit","type":"event"},{"anonymous":false,"inputs":[{"indexed":false,"internalType":"uint256","name":"value","type":"uint256"},{"indexed":true,"internalType":"address","name":"ownerAddress","type":"address"}],"name":"FundsWithdrawal","type":"event"},{"anonymous":false,"inputs":[{"indexed":false,"internalType":"uint8","name":"version","type":"uint8"}],"name":"Initialized","type":"event"},{"anonymous":false,"inputs":[{"indexed":false,"internalType":"uint256","name":"value","type":"uint256"}],"name":"LiquidationThresholdPeriodUpdate","type":"event"},{"anonymous":false,"inputs":[{"indexed":false,"internalType":"uint256","name":"value","type":"uint256"}],"name":"MinimumBlocksBeforeLiquidationUpdate","type":"event"},{"anonymous":false,"inputs":[{"indexed":false,"internalType":"uint256","name":"oldFee","type":"uint256"},{"indexed":false,"internalType":"uint256","name":"newFee","type":"uint256"}],"name":"NetworkFeeUpdate","type":"event"},{"anonymous":false,"inputs":[{"indexed":false,"internalType":"uint256","name":"value","type":"uint256"},{"indexed":false,"internalType":"address","name":"recipient","type":"address"}],"name":"NetworkFeesWithdrawal","type":"event"},{"anonymous":false,"inputs":[{"indexed":true,"internalType":"address","name":"ownerAddress","type":"address"},{"indexed":false,"internalType":"uint32","name":"operatorId","type":"uint32"},{"indexed":false,"internalType":"uint256","name":"blockNumber","type":"uint256"},{"indexed":false,"internalType":"uint256","name":"fee","type":"uint256"}],"name":"OperatorFeeDeclaration","type":"event"},{"anonymous":false,"inputs":[{"indexed":true,"internalType":"address","name":"ownerAddress","type":"address"},{"indexed":false,"internalType":"uint32","name":"operatorId","type":"uint32"},{"indexed":false,"internalType":"uint256","name":"blockNumber","type":"uint256"},{"indexed":false,"internalType":"uint256","name":"fee","type":"uint256"}],"name":"OperatorFeeExecution","type":"event"},{"anonymous":false,"inputs":[{"indexed":false,"internalType":"uint256","name":"value","type":"uint256"}],"name":"OperatorFeeIncreaseLimitUpdate","type":"event"},{"anonymous":false,"inputs":[{"indexed":false,"internalType":"uint256","name":"value","type":"uint256"}],"name":"OperatorMaxFeeIncreaseUpdate","type":"event"},{"anonymous":false,"inputs":[{"indexed":true,"internalType":"uint32","name":"id","type":"uint32"},{"indexed":false,"internalType":"string","name":"name","type":"string"},{"indexed":true,"internalType":"address","name":"ownerAddress","type":"address"},{"indexed":false,"internalType":"bytes","name":"publicKey","type":"bytes"},{"indexed":false,"internalType":"uint256","name":"fee","type":"uint256"}],"name":"OperatorRegistration","type":"event"},{"anonymous":false,"inputs":[{"indexed":false,"internalType":"uint32","name":"operatorId","type":"uint32"},{"indexed":true,"internalType":"address","name":"ownerAddress","type":"address"}],"name":"OperatorRemoval","type":"event"},{"anonymous":false,"inputs":[{"indexed":false,"internalType":"uint32","name":"operatorId","type":"uint32"},{"indexed":true,"internalType":"address","name":"ownerAddress","type":"address"},{"indexed":false,"internalType":"uint256","name":"blockNumber","type":"uint256"},{"indexed":false,"internalType":"uint256","name":"score","type":"uint256"}],"name":"OperatorScoreUpdate","type":"event"},{"anonymous":false,"inputs":[{"indexed":true,"internalType":"address","name":"previousOwner","type":"address"},{"indexed":true,"internalType":"address","name":"newOwner","type":"address"}],"name":"OwnershipTransferred","type":"event"},{"anonymous":false,"inputs":[{"indexed":false,"internalType":"uint256","name":"value","type":"uint256"}],"name":"RegisteredOperatorsPerAccountLimitUpdate","type":"event"},{"anonymous":false,"inputs":[{"indexed":true,"internalType":"address","name":"ownerAddress","type":"address"},{"indexed":false,"internalType":"bytes","name":"publicKey","type":"bytes"},{"indexed":false,"internalType":"uint32[]","name":"operatorIds","type":"uint32[]"},{"indexed":false,"internalType":"bytes[]","name":"sharesPublicKeys","type":"bytes[]"},{"indexed":false,"internalType":"bytes[]","name":"encryptedKeys","type":"bytes[]"}],"name":"ValidatorRegistration","type":"event"},{"anonymous":false,"inputs":[{"indexed":true,"internalType":"address","name":"ownerAddress","type":"address"},{"indexed":false,"internalType":"bytes","name":"publicKey","type":"bytes"}],"name":"ValidatorRemoval","type":"event"},{"anonymous":false,"inputs":[{"indexed":false,"internalType":"uint256","name":"value","type":"uint256"}],"name":"ValidatorsPerOperatorLimitUpdate","type":"event"},{"inputs":[{"internalType":"address","name":"ownerAddress","type":"address"}],"name":"addressNetworkFee","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"uint32","name":"operatorId","type":"uint32"}],"name":"cancelDeclaredOperatorFee","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"uint32","name":"operatorId","type":"uint32"},{"internalType":"uint256","name":"operatorFee","type":"uint256"}],"name":"declareOperatorFee","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"address","name":"ownerAddress","type":"address"},{"internalType":"uint256","name":"amount","type":"uint256"}],"name":"deposit","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"uint32","name":"operatorId","type":"uint32"}],"name":"executeOperatorFee","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"address","name":"ownerAddress","type":"address"}],"name":"getAddressBalance","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"address","name":"ownerAddress","type":"address"}],"name":"getAddressBurnRate","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"getDeclaredOperatorFeePeriod","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"getExecuteOperatorFeePeriod","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"getLiquidationThresholdPeriod","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"getNetworkEarnings","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"getNetworkFee","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"uint32","name":"operatorId","type":"uint32"}],"name":"getOperatorById","outputs":[{"internalType":"string","name":"","type":"string"},{"internalType":"address","name":"","type":"address"},{"internalType":"bytes","name":"","type":"bytes"},{"internalType":"uint256","name":"","type":"uint256"},{"internalType":"uint256","name":"","type":"uint256"},{"internalType":"uint256","name":"","type":"uint256"},{"internalType":"bool","name":"","type":"bool"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"bytes","name":"publicKey","type":"bytes"}],"name":"getOperatorByPublicKey","outputs":[{"internalType":"string","name":"","type":"string"},{"internalType":"address","name":"","type":"address"},{"internalType":"bytes","name":"","type":"bytes"},{"internalType":"uint256","name":"","type":"uint256"},{"internalType":"uint256","name":"","type":"uint256"},{"internalType":"uint256","name":"","type":"uint256"},{"internalType":"bool","name":"","type":"bool"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"uint32","name":"operatorId","type":"uint32"}],"name":"getOperatorDeclaredFee","outputs":[{"internalType":"uint256","name":"","type":"uint256"},{"internalType":"uint256","name":"","type":"uint256"},{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"uint32","name":"operatorId","type":"uint32"}],"name":"getOperatorFee","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"getOperatorFeeIncreaseLimit","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"bytes","name":"publicKey","type":"bytes"}],"name":"getOperatorsByValidator","outputs":[{"internalType":"uint32[]","name":"","type":"uint32[]"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"address","name":"ownerAddress","type":"address"}],"name":"getValidatorsByOwnerAddress","outputs":[{"internalType":"bytes[]","name":"","type":"bytes[]"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"contract ISSVRegistry","name":"registryAddress_","type":"address"},{"internalType":"contract IERC20","name":"token_","type":"address"},{"internalType":"uint64","name":"minimumBlocksBeforeLiquidation_","type":"uint64"},{"internalType":"uint64","name":"operatorMaxFeeIncrease_","type":"uint64"},{"internalType":"uint64","name":"declareOperatorFeePeriod_","type":"uint64"},{"internalType":"uint64","name":"executeOperatorFeePeriod_","type":"uint64"}],"name":"initialize","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"address","name":"ownerAddress","type":"address"}],"name":"isLiquidatable","outputs":[{"internalType":"bool","name":"","type":"bool"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"address","name":"ownerAddress","type":"address"}],"name":"isLiquidated","outputs":[{"internalType":"bool","name":"","type":"bool"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"address[]","name":"ownerAddresses","type":"address[]"}],"name":"liquidate","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[],"name":"owner","outputs":[{"internalType":"address","name":"","type":"address"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"uint256","name":"amount","type":"uint256"}],"name":"reactivateAccount","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"string","name":"name","type":"string"},{"internalType":"bytes","name":"publicKey","type":"bytes"},{"internalType":"uint256","name":"fee","type":"uint256"}],"name":"registerOperator","outputs":[{"internalType":"uint32","name":"operatorId","type":"uint32"}],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"bytes","name":"publicKey","type":"bytes"},{"internalType":"uint32[]","name":"operatorIds","type":"uint32[]"},{"internalType":"bytes[]","name":"sharesPublicKeys","type":"bytes[]"},{"internalType":"bytes[]","name":"sharesEncrypted","type":"bytes[]"},{"internalType":"uint256","name":"amount","type":"uint256"}],"name":"registerValidator","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"uint32","name":"operatorId","type":"uint32"}],"name":"removeOperator","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"bytes","name":"publicKey","type":"bytes"}],"name":"removeValidator","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[],"name":"renounceOwnership","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"address","name":"newOwner","type":"address"}],"name":"transferOwnership","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"uint64","name":"newDeclareOperatorFeePeriod","type":"uint64"}],"name":"updateDeclareOperatorFeePeriod","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"uint64","name":"newExecuteOperatorFeePeriod","type":"uint64"}],"name":"updateExecuteOperatorFeePeriod","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"uint64","name":"blocks","type":"uint64"}],"name":"updateLiquidationThresholdPeriod","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"uint256","name":"fee","type":"uint256"}],"name":"updateNetworkFee","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"uint64","name":"newOperatorMaxFeeIncrease","type":"uint64"}],"name":"updateOperatorFeeIncreaseLimit","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"uint32","name":"operatorId","type":"uint32"},{"internalType":"uint32","name":"score","type":"uint32"}],"name":"updateOperatorScore","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"bytes","name":"publicKey","type":"bytes"},{"internalType":"uint32[]","name":"operatorIds","type":"uint32[]"},{"internalType":"bytes[]","name":"sharesPublicKeys","type":"bytes[]"},{"internalType":"bytes[]","name":"sharesEncrypted","type":"bytes[]"},{"internalType":"uint256","name":"amount","type":"uint256"}],"name":"updateValidator","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"uint32","name":"operatorId","type":"uint32"}],"name":"validatorsPerOperatorCount","outputs":[{"internalType":"uint32","name":"","type":"uint32"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"version","outputs":[{"internalType":"uint32","name":"","type":"uint32"}],"stateMutability":"pure","type":"function"},{"inputs":[{"internalType":"uint256","name":"amount","type":"uint256"}],"name":"withdraw","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[],"name":"withdrawAll","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"uint256","name":"amount","type":"uint256"}],"name":"withdrawNetworkEarnings","outputs":[],"stateMutability":"nonpayable","type":"function"}]"#.to_string();
        let box_storage = Box::new(storage);
        let mut sync = Sync::new(box_storage, eth1_ws_endpoint, contract_address, abi).await;
        sync.start(7742574).await;
    }
}