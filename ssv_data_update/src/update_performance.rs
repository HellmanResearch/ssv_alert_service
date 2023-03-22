// use std::time::{Duration, SystemTime, UNIX_EPOCH};
// use storage::{MysqlStorage, Storage, WriteResult};
// use std::collections::HashMap;
//
//
// fn calc_performance(decided_count_option: Option<&u32>, validator_count_option: Option<&u32>, days: u32) -> f32 {
//     match decided_count_option {
//         Some(decided_count) => {
//             match validator_count_option {
//                 Some(validator_count) => {
//                     if validator_count == 0 {
//                         return 0.0
//                     }
//                     decided_count_f32 = decided_count.as_f32();
//                     validator_count_f32 = validator_count.as_f32();
//                     days_f32 = days.as_f32();
//                     let mut performance = decided_count_f32 / (validator_count_f32 * days_f32) * 100;
//                     if performance > 100.0 {
//                         performance = 100.0
//                     }
//                     return performance;
//                 }
//                 None => {
//                     return 0.0;
//                 }
//             }
//         }
//         None => {
//             return 0.0;
//         }
//     }
// }
//
// pub fn update_operator_task(mysql_connect_url: String) -> Result<u32, String> {
//     let mysql_storage = MysqlStorage::new(mysql_connect_url);
//     let mut error_count = 0;
//
//     let timestamp = SystemTime::now()
//         .duration_since(UNIX_EPOCH)
//         .unwrap()
//         .as_secs();
//
//     let start_timestamp = timestamp - 86400;
//
//     match mysql_storage.get_operator_decided_count(start_timestamp) {
//         Ok(operator_decided_count) => {
//             let operator_decided_count_map: HashMap<u32, u32> = HashMap::new();
//             for item in operator_decided_count {
//                 operator_decided_count_map[item.operator_id] = item.count;
//             }
//             match mysql_storage.get_operator_validator_count() {
//                 Ok(operator_validator_count) => {
//                     let operator_validator_count_map = HashMap::new();
//                     for item in operator_validator_count {
//                         operator_validator_count_map[item.operator_id] = item.count;
//                     }
//                     match mysql_storage.get_all_operators() {
//                         Ok(operators) => {
//                             for operator in operators {
//                                 let decided_count_option=  operator_decided_count_map.get(&operator.id);
//                                 let validator_count_option = operator_validator_count_map.get(&operator.id)
//                                 let performance_1day = calc_performance(decided_count_option, validator_count_option, 1)
//
//                                 mysql_storage.update_operator(operator.id, operator.account_public_key, operator.status)
//                             }
//                         },
//                         Err(error) => {
//
//                         }
//                     }
//                 },
//                 Err(error) => {
//
//                 }
//             }
//         },
//         Err(error) => {
//
//         }
//     }
//     return Ok(error_count);
// }
