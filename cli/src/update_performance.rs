use std::time::{Duration, SystemTime, UNIX_EPOCH};
use storage::{MysqlStorage, Storage, WriteResult};
use std::collections::HashMap;


pub fn update_operator_task(mysql_connect_url: String) -> Result<u32, String> {
    let mysql_storage = MysqlStorage::new(mysql_connect_url);
    let mut error_count = 0;

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let start_timestamp = timestamp - 86400;

    match mysql_storage.get_operator_decided_count(start_timestamp) {
        Ok(operator_decided_count) => {
            let operator_decided_count_map = HashMap::new();
            for item in operator_decided_count {
                operator_decided_count_map[item.operator_id] = count;
            }
            match mysql_storage.get_operator_validator_count() {
                Ok(operator_validator_count) => {
                    let operator_validator_count_map = HashMap::new();
                    for item in operator_validator_count {
                        operator_validator_count_map[item.operator_id] = count;
                    }
                    match mysql_storage.get_all_operators {
                        Ok(operators) => {


                            for operator in operators {

                            }
                        }
                    }
                }
            }
        }
        Err()
    }
}
