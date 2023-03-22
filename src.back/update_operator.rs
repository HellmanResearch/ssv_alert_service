use std::collections::HashMap;
use std::fmt::format;
use log::{debug, error};
use tungstenite::connect;
// use tungstenite::protocol::frame::coding::CloseCode::Status;
use storage::{MysqlStorage, Storage, WriteResult};
use ssv_contract::ssv_contract::SSVContract;
use storage::operator::Status;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::count;
use tracing::info;


fn calc_performance(decided_count_option: Option<&u32>, validator_count_option: Option<&u32>, days: u32) -> f32 {
    match decided_count_option {
        Some(decided_count) => {
            match validator_count_option {
                Some(validator_count) => {
                    if *validator_count == 0 {
                        return 0.0
                    }
                    let decided_count_f32 = *decided_count as f32;
                    let validator_count_f32 = *validator_count as f32;
                    let days_f32 = days as f32;
                    let mut performance = decided_count_f32 / (validator_count_f32 * days_f32) * 100.0;
                    if performance > 100.0 {
                        performance = 100.0
                    }
                    return performance;
                }
                None => {
                    return 0.0;
                }
            }
        }
        None => {
            return 0.0;
        }
    }



}

fn calc_status(active_operator_decided_option: Option<&u32>, is_active: bool) -> Result<Status, String> {
    return if is_active == true {
        match active_operator_decided_option {
            Some(count) => {
                // let zero: u32 = 0;
                if *count > 0 {
                    Ok(Status::Active)
                } else {
                    Ok(Status::Inactive)
                }
            }
            None => {
                Ok(Status::Inactive)
            }
        }
    } else {
        Ok(Status::Removed)
    }
}


pub async fn update_operator_task(eth_http_endpoint: String, address: String, abi: String, mysql_connect_url: String) -> Result<u32, String> {
    let ssv_contract = SSVContract::new(eth_http_endpoint, address, abi);
    let mysql_storage = MysqlStorage::new(mysql_connect_url);

    let mut error_count = 0;

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let start_timestamp = timestamp - 86400;
    let active_timestamp = timestamp - (60 * 10);

    let operator_decided_count = mysql_storage.get_operator_decided_count(start_timestamp)?;
    let operator_validator_count = mysql_storage.get_operator_validator_count()?;
    let operators = mysql_storage.get_all_operators()?;
    let active_operator_decided_count = mysql_storage.get_operator_decided_count(active_timestamp)?;

    let mut operator_decided_count_map: HashMap<u32, u32> = HashMap::new();
    let mut operator_validator_count_map: HashMap<u32, u32>  = HashMap::new();
    let mut active_operator_decided_map: HashMap<u32, u32>  = HashMap::new();

    for item in operator_decided_count {
        operator_decided_count_map.insert(item.operator_id, item.count);
        // operator_decided_count_map[&item.operator_id] = item.count;
    }

    for item in operator_validator_count {
        operator_validator_count_map.insert(item.operator_id, item.count);

        // operator_validator_count_map[&item.operator_id] = item.count;
    }

    for item in active_operator_decided_count {
        active_operator_decided_map.insert(item.operator_id, item.count);

        // active_operator_decided_map[&item.operator_id] = item.count;
    }

    for operator in operators {
        debug!("update operator id: {} name: {}", operator.id, operator.name);
        match ssv_contract.getOperatorById(operator.id).await {
            Ok((name, owner_address, public_key, validator_count, fee_human, score, is_active)) => {
                let active_operator_decided_option: Option<&u32> = active_operator_decided_map.get(&operator.id);
                let status_result = calc_status(active_operator_decided_option, is_active);
                match status_result {
                    Ok(status) => {

                        let decided_count_option=  operator_decided_count_map.get(&operator.id);
                        let validator_count_option = operator_validator_count_map.get(&operator.id);
                        let performance_1day = calc_performance(decided_count_option, validator_count_option, 1);

                        let validator_count = *validator_count_option.unwrap_or(&0);

                        mysql_storage.update_operator(operator.id,
                                                      operator.name,
                                                      operator.account_public_key,
                                                      status,
                                                      validator_count,
                                                      fee_human,
                                                      performance_1day,
                                                      0.0,
                        );
                    }
                    Err(error) => {
                        error_count += 1;
                        error!("calc_status error: {}", error)
                    }
                }
            }
            Err(error) => {
                error_count += 1;
                error!("ssv_contract.getOperatorById error operator_id: {} error: {}", operator.id, error)
            }
        }
    }

    return Ok(error_count);

    // match mysql_storage.get_all_operators() {
    //     Ok(operators) => {
    //         for operator in operators {
    //             match ssv_contract.getOperatorById(operator.id) {
    //                 Ok((name, owner_address, public_key, validator_count, fee_human, score, is_active)) => {
    //                     match Status::from_string(operator.status) {
    //                         Ok(status) => {
    //                             match mysql_storage.update_operator(operator.id, owner_address, public_key, validator_count, fee_human) {
    //                                 WriteResult::Normal => {
    //
    //                                 }
    //                                 _ => {
    //                                     error_count += 1;
    //                                     error!("update_operator error")
    //                                 }
    //                             }
    //                         }
    //                         Err(error) => {
    //                             error_count += 1;
    //                             error!("Status::from_string error: {}", error)
    //                         }
    //                     }
    //                 }
    //                 Err(error) => {
    //                     error_count += 1;
    //                     error!("ssv_contract.getOperatorById error operator_id: {} error: {}", operator.id, error)
    //                 }
    //             }
    //         }
    //         return Ok(error_count);
    //     }
    //     Err(error) => {
    //         return Err(error);
    //     }
    // }
}




#[cfg(test)]
mod test {
    use log::error;
    use storage::MysqlStorage;
    use super::update_operator_task;


    #[tokio::test]
    async fn test_update_operator_task() {
        env_logger::builder().filter_level(log::LevelFilter::Info).init();

        error!("starting...");

        let connect_url = "mysql://root:wonders,1@192.168.1.128:3308/ssv_alert_service_5".to_string();
        let eth1_http_endpoint = "https://goerli.infura.io/v3/155f6bd08040410da6ac828c2cf24cc7".to_string();
        let contract_address = "0xb9e155e65B5c4D66df28Da8E9a0957f06F11Bc04".to_string();
        let abi = r#"[{"inputs":[],"name":"AccountAlreadyEnabled","type":"error"},{"inputs":[],"name":"ApprovalNotWithinTimeframe","type":"error"},{"inputs":[],"name":"BelowMinimumBlockPeriod","type":"error"},{"inputs":[],"name":"BurnRatePositive","type":"error"},{"inputs":[],"name":"CallerNotOperatorOwner","type":"error"},{"inputs":[],"name":"CallerNotValidatorOwner","type":"error"},{"inputs":[],"name":"ExceedManagingOperatorsPerAccountLimit","type":"error"},{"inputs":[],"name":"FeeExceedsIncreaseLimit","type":"error"},{"inputs":[],"name":"FeeTooLow","type":"error"},{"inputs":[],"name":"NegativeBalance","type":"error"},{"inputs":[],"name":"NoPendingFeeChangeRequest","type":"error"},{"inputs":[],"name":"NotEnoughBalance","type":"error"},{"inputs":[],"name":"OperatorWithPublicKeyNotExist","type":"error"},{"inputs":[],"name":"ValidatorWithPublicKeyNotExist","type":"error"},{"anonymous":false,"inputs":[{"indexed":true,"internalType":"address","name":"ownerAddress","type":"address"}],"name":"AccountEnable","type":"event"},{"anonymous":false,"inputs":[{"indexed":true,"internalType":"address","name":"ownerAddress","type":"address"}],"name":"AccountLiquidation","type":"event"},{"anonymous":false,"inputs":[{"indexed":false,"internalType":"uint256","name":"value","type":"uint256"}],"name":"DeclareOperatorFeePeriodUpdate","type":"event"},{"anonymous":false,"inputs":[{"indexed":true,"internalType":"address","name":"ownerAddress","type":"address"},{"indexed":false,"internalType":"uint32","name":"operatorId","type":"uint32"}],"name":"DeclaredOperatorFeeCancelation","type":"event"},{"anonymous":false,"inputs":[{"indexed":false,"internalType":"uint256","name":"value","type":"uint256"}],"name":"ExecuteOperatorFeePeriodUpdate","type":"event"},{"anonymous":false,"inputs":[{"indexed":false,"internalType":"uint256","name":"value","type":"uint256"},{"indexed":true,"internalType":"address","name":"ownerAddress","type":"address"},{"indexed":true,"internalType":"address","name":"senderAddress","type":"address"}],"name":"FundsDeposit","type":"event"},{"anonymous":false,"inputs":[{"indexed":false,"internalType":"uint256","name":"value","type":"uint256"},{"indexed":true,"internalType":"address","name":"ownerAddress","type":"address"}],"name":"FundsWithdrawal","type":"event"},{"anonymous":false,"inputs":[{"indexed":false,"internalType":"uint8","name":"version","type":"uint8"}],"name":"Initialized","type":"event"},{"anonymous":false,"inputs":[{"indexed":false,"internalType":"uint256","name":"value","type":"uint256"}],"name":"LiquidationThresholdPeriodUpdate","type":"event"},{"anonymous":false,"inputs":[{"indexed":false,"internalType":"uint256","name":"value","type":"uint256"}],"name":"MinimumBlocksBeforeLiquidationUpdate","type":"event"},{"anonymous":false,"inputs":[{"indexed":false,"internalType":"uint256","name":"oldFee","type":"uint256"},{"indexed":false,"internalType":"uint256","name":"newFee","type":"uint256"}],"name":"NetworkFeeUpdate","type":"event"},{"anonymous":false,"inputs":[{"indexed":false,"internalType":"uint256","name":"value","type":"uint256"},{"indexed":false,"internalType":"address","name":"recipient","type":"address"}],"name":"NetworkFeesWithdrawal","type":"event"},{"anonymous":false,"inputs":[{"indexed":true,"internalType":"address","name":"ownerAddress","type":"address"},{"indexed":false,"internalType":"uint32","name":"operatorId","type":"uint32"},{"indexed":false,"internalType":"uint256","name":"blockNumber","type":"uint256"},{"indexed":false,"internalType":"uint256","name":"fee","type":"uint256"}],"name":"OperatorFeeDeclaration","type":"event"},{"anonymous":false,"inputs":[{"indexed":true,"internalType":"address","name":"ownerAddress","type":"address"},{"indexed":false,"internalType":"uint32","name":"operatorId","type":"uint32"},{"indexed":false,"internalType":"uint256","name":"blockNumber","type":"uint256"},{"indexed":false,"internalType":"uint256","name":"fee","type":"uint256"}],"name":"OperatorFeeExecution","type":"event"},{"anonymous":false,"inputs":[{"indexed":false,"internalType":"uint256","name":"value","type":"uint256"}],"name":"OperatorFeeIncreaseLimitUpdate","type":"event"},{"anonymous":false,"inputs":[{"indexed":false,"internalType":"uint256","name":"value","type":"uint256"}],"name":"OperatorMaxFeeIncreaseUpdate","type":"event"},{"anonymous":false,"inputs":[{"indexed":true,"internalType":"uint32","name":"id","type":"uint32"},{"indexed":false,"internalType":"string","name":"name","type":"string"},{"indexed":true,"internalType":"address","name":"ownerAddress","type":"address"},{"indexed":false,"internalType":"bytes","name":"publicKey","type":"bytes"},{"indexed":false,"internalType":"uint256","name":"fee","type":"uint256"}],"name":"OperatorRegistration","type":"event"},{"anonymous":false,"inputs":[{"indexed":false,"internalType":"uint32","name":"operatorId","type":"uint32"},{"indexed":true,"internalType":"address","name":"ownerAddress","type":"address"}],"name":"OperatorRemoval","type":"event"},{"anonymous":false,"inputs":[{"indexed":false,"internalType":"uint32","name":"operatorId","type":"uint32"},{"indexed":true,"internalType":"address","name":"ownerAddress","type":"address"},{"indexed":false,"internalType":"uint256","name":"blockNumber","type":"uint256"},{"indexed":false,"internalType":"uint256","name":"score","type":"uint256"}],"name":"OperatorScoreUpdate","type":"event"},{"anonymous":false,"inputs":[{"indexed":true,"internalType":"address","name":"previousOwner","type":"address"},{"indexed":true,"internalType":"address","name":"newOwner","type":"address"}],"name":"OwnershipTransferred","type":"event"},{"anonymous":false,"inputs":[{"indexed":false,"internalType":"uint256","name":"value","type":"uint256"}],"name":"RegisteredOperatorsPerAccountLimitUpdate","type":"event"},{"anonymous":false,"inputs":[{"indexed":true,"internalType":"address","name":"ownerAddress","type":"address"},{"indexed":false,"internalType":"bytes","name":"publicKey","type":"bytes"},{"indexed":false,"internalType":"uint32[]","name":"operatorIds","type":"uint32[]"},{"indexed":false,"internalType":"bytes[]","name":"sharesPublicKeys","type":"bytes[]"},{"indexed":false,"internalType":"bytes[]","name":"encryptedKeys","type":"bytes[]"}],"name":"ValidatorRegistration","type":"event"},{"anonymous":false,"inputs":[{"indexed":true,"internalType":"address","name":"ownerAddress","type":"address"},{"indexed":false,"internalType":"bytes","name":"publicKey","type":"bytes"}],"name":"ValidatorRemoval","type":"event"},{"anonymous":false,"inputs":[{"indexed":false,"internalType":"uint256","name":"value","type":"uint256"}],"name":"ValidatorsPerOperatorLimitUpdate","type":"event"},{"inputs":[{"internalType":"address","name":"ownerAddress","type":"address"}],"name":"addressNetworkFee","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"uint32","name":"operatorId","type":"uint32"}],"name":"cancelDeclaredOperatorFee","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"uint32","name":"operatorId","type":"uint32"},{"internalType":"uint256","name":"operatorFee","type":"uint256"}],"name":"declareOperatorFee","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"address","name":"ownerAddress","type":"address"},{"internalType":"uint256","name":"amount","type":"uint256"}],"name":"deposit","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"uint32","name":"operatorId","type":"uint32"}],"name":"executeOperatorFee","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"address","name":"ownerAddress","type":"address"}],"name":"getAddressBalance","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"address","name":"ownerAddress","type":"address"}],"name":"getAddressBurnRate","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"getDeclaredOperatorFeePeriod","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"getExecuteOperatorFeePeriod","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"getLiquidationThresholdPeriod","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"getNetworkEarnings","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"getNetworkFee","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"uint32","name":"operatorId","type":"uint32"}],"name":"getOperatorById","outputs":[{"internalType":"string","name":"","type":"string"},{"internalType":"address","name":"","type":"address"},{"internalType":"bytes","name":"","type":"bytes"},{"internalType":"uint256","name":"","type":"uint256"},{"internalType":"uint256","name":"","type":"uint256"},{"internalType":"uint256","name":"","type":"uint256"},{"internalType":"bool","name":"","type":"bool"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"bytes","name":"publicKey","type":"bytes"}],"name":"getOperatorByPublicKey","outputs":[{"internalType":"string","name":"","type":"string"},{"internalType":"address","name":"","type":"address"},{"internalType":"bytes","name":"","type":"bytes"},{"internalType":"uint256","name":"","type":"uint256"},{"internalType":"uint256","name":"","type":"uint256"},{"internalType":"uint256","name":"","type":"uint256"},{"internalType":"bool","name":"","type":"bool"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"uint32","name":"operatorId","type":"uint32"}],"name":"getOperatorDeclaredFee","outputs":[{"internalType":"uint256","name":"","type":"uint256"},{"internalType":"uint256","name":"","type":"uint256"},{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"uint32","name":"operatorId","type":"uint32"}],"name":"getOperatorFee","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"getOperatorFeeIncreaseLimit","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"bytes","name":"publicKey","type":"bytes"}],"name":"getOperatorsByValidator","outputs":[{"internalType":"uint32[]","name":"","type":"uint32[]"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"address","name":"ownerAddress","type":"address"}],"name":"getValidatorsByOwnerAddress","outputs":[{"internalType":"bytes[]","name":"","type":"bytes[]"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"contract ISSVRegistry","name":"registryAddress_","type":"address"},{"internalType":"contract IERC20","name":"token_","type":"address"},{"internalType":"uint64","name":"minimumBlocksBeforeLiquidation_","type":"uint64"},{"internalType":"uint64","name":"operatorMaxFeeIncrease_","type":"uint64"},{"internalType":"uint64","name":"declareOperatorFeePeriod_","type":"uint64"},{"internalType":"uint64","name":"executeOperatorFeePeriod_","type":"uint64"}],"name":"initialize","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"address","name":"ownerAddress","type":"address"}],"name":"isLiquidatable","outputs":[{"internalType":"bool","name":"","type":"bool"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"address","name":"ownerAddress","type":"address"}],"name":"isLiquidated","outputs":[{"internalType":"bool","name":"","type":"bool"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"address[]","name":"ownerAddresses","type":"address[]"}],"name":"liquidate","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[],"name":"owner","outputs":[{"internalType":"address","name":"","type":"address"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"uint256","name":"amount","type":"uint256"}],"name":"reactivateAccount","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"string","name":"name","type":"string"},{"internalType":"bytes","name":"publicKey","type":"bytes"},{"internalType":"uint256","name":"fee","type":"uint256"}],"name":"registerOperator","outputs":[{"internalType":"uint32","name":"operatorId","type":"uint32"}],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"bytes","name":"publicKey","type":"bytes"},{"internalType":"uint32[]","name":"operatorIds","type":"uint32[]"},{"internalType":"bytes[]","name":"sharesPublicKeys","type":"bytes[]"},{"internalType":"bytes[]","name":"sharesEncrypted","type":"bytes[]"},{"internalType":"uint256","name":"amount","type":"uint256"}],"name":"registerValidator","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"uint32","name":"operatorId","type":"uint32"}],"name":"removeOperator","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"bytes","name":"publicKey","type":"bytes"}],"name":"removeValidator","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[],"name":"renounceOwnership","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"address","name":"newOwner","type":"address"}],"name":"transferOwnership","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"uint64","name":"newDeclareOperatorFeePeriod","type":"uint64"}],"name":"updateDeclareOperatorFeePeriod","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"uint64","name":"newExecuteOperatorFeePeriod","type":"uint64"}],"name":"updateExecuteOperatorFeePeriod","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"uint64","name":"blocks","type":"uint64"}],"name":"updateLiquidationThresholdPeriod","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"uint256","name":"fee","type":"uint256"}],"name":"updateNetworkFee","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"uint64","name":"newOperatorMaxFeeIncrease","type":"uint64"}],"name":"updateOperatorFeeIncreaseLimit","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"uint32","name":"operatorId","type":"uint32"},{"internalType":"uint32","name":"score","type":"uint32"}],"name":"updateOperatorScore","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"bytes","name":"publicKey","type":"bytes"},{"internalType":"uint32[]","name":"operatorIds","type":"uint32[]"},{"internalType":"bytes[]","name":"sharesPublicKeys","type":"bytes[]"},{"internalType":"bytes[]","name":"sharesEncrypted","type":"bytes[]"},{"internalType":"uint256","name":"amount","type":"uint256"}],"name":"updateValidator","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"uint32","name":"operatorId","type":"uint32"}],"name":"validatorsPerOperatorCount","outputs":[{"internalType":"uint32","name":"","type":"uint32"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"version","outputs":[{"internalType":"uint32","name":"","type":"uint32"}],"stateMutability":"pure","type":"function"},{"inputs":[{"internalType":"uint256","name":"amount","type":"uint256"}],"name":"withdraw","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[],"name":"withdrawAll","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"uint256","name":"amount","type":"uint256"}],"name":"withdrawNetworkEarnings","outputs":[],"stateMutability":"nonpayable","type":"function"}]"#.to_string();

        let r = update_operator_task(eth1_http_endpoint, contract_address, abi, connect_url);
        println!("r: ")
    }
}