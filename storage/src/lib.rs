extern crate core;

mod models;
mod schema;
pub mod operator;
pub mod account;
pub mod decided;
pub mod validator;
mod performance;
pub mod tag;

use crate::validator::{NewValidator, NewValidatorOperator};


use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use std::f32::consts::E;
use std::fmt::format;
use std::ptr::NonNull;
use diesel::query_builder::IncompleteInsertStatement;
use crate::account::{Account, NewAccount, UpdateAccount};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use decided::{NewDecided, NewOperatorDecidedRecord};
use operator::Operator;
use operator::Status;
use validator::Validator;
use performance::PerformanceRecord;
use diesel::sql_types::Integer;
use crate::decided::OperatorDecidedCount;
use crate::operator::OperatorValidatorCount;
use crate::performance::NewPerformanceRecord;
use tag::Tag;
use crate::tag::{NewTag, UpdateTag};


#[derive(Debug)]
pub enum WriteResult {
    Normal,
    UniqueViolation,
    Other(String),
}

// pub fn establish_connection() -> MysqlConnection{
//     dotenv().ok();
//
//     let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
//     MysqlConnection::establish(&database_url)
//         .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
// }


pub trait Storage {
    fn add_operator(&self, id: u32, name: String, account_public_key: String, status: Status, validator_count: u32, fee_human: f32) -> WriteResult;
    // fn update_operator(&self, id: u32, name: String, account_public_key: String, status: Status, validator_count: u32, fee_human: f32) -> WriteResult;
    fn update_operator(&self, id: u32, name: String, account_public_key: String, status: Status, validator_count: u32, fee_human: f32, performance_1day: f32, performance_1month: f32) -> WriteResult;
    fn get_all_operators(&self) -> Result<Vec<Operator>, String>;
    fn get_all_accounts(&self) -> Result<Vec<Account>, String>;
    fn add_account(&self, public_key: String, ssv_balance_human: f32) -> WriteResult;
    fn get_account(&self, public_key: String) -> Option<Account>;
    fn update_account(&self, public_key: String, ssv_balance_human: f32, is_liquidation: bool) -> WriteResult;
    fn add_decided(&self, role: String, validator_public_key: String, signature: String, height: u32, round: u32, identifier: String, message_type: u32) -> WriteResult;
    fn add_operator_decided_record(&self, operator_id: u32, validator_public_key: String, height: u32, round: u32) -> WriteResult;
    fn add_validator(&self, account_public_key: String, public_key: String) -> WriteResult;
    fn add_validator_operator(&self, validator_public_key: String, operator_id: u32) -> WriteResult;
    fn transfer_query_qesult(&self, result: QueryResult<usize>) -> WriteResult;
    fn add_performance_record(&self, operator_id: u32, performance: f32, timestamp: u64) -> WriteResult;

    fn get_operator_decided_count(&self, start_timestamp: u64) -> Result<Vec<OperatorDecidedCount>, String>;
    fn delete_decided_before_timestamp(&self, timestamp: u64) -> Result<usize, String>;
    fn delete_decided_by_validator_height(&self, validator_public_key: String, height: u32) -> Result<usize, String>;
    fn delete_operator_decided_before_timestamp(&self, timestamp: u64) -> Result<usize, String>;
    fn delete_operator_decided_record_by_validator_height(&self, validator_public_key: String, height: u32) -> Result<usize, String>;
    fn get_operator_validator_count(&self) -> Result<Vec<OperatorValidatorCount>, String>;

    fn add_tag(&self, key: String, value: String) -> WriteResult;
    fn get_tag(&self, key: String) -> Option<Tag>;
    fn update_tag(&self, key: String, value: String) -> WriteResult;
}

pub struct MysqlStorage {
    connection: Option<MysqlConnection>,
    connect_url: String,
}

impl MysqlStorage {
    pub fn new(connect_url: String) -> Self {
        return Self {
            connection: None,
            connect_url,
        };
    }

    fn get_connection(&self) -> MysqlConnection {
        // dotenv().ok();
        // let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        return MysqlConnection::establish(self.connect_url.as_str())
            .unwrap_or_else(|_| panic!("Error connecting to {}", self.connect_url));
        // self.connection = Some(connection);

        // match self.connection.clone() {
        //     Some(connection) =>{
        //         return connection
        //     },
        //     None => {
        //         dotenv().ok();
        //         let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        //         let connection = MysqlConnection::establish(&database_url)
        //             .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
        //         self.connection = Some(connection);
        //         return connection;
        //     }
        // }
    }

    // fn save<T: Table, U: Insertable<T>>(&self, target: T, records: U) {
    //
    //     let connection = &mut self.get_connection();
    //
    //     let r = diesel::insert_into(target)
    //         .values::<U>(records)
    //         .execute(connection);
    //         // .expect("error saving new post");
    // }
}


impl Storage for MysqlStorage {
    fn add_operator(&self, id: u32, name: String, account_public_key: String, status: Status, validator_count: u32, fee_human: f32) -> WriteResult {
        let operator = operator::NewOperator {
            id,
            name,
            status: status.to_string(),
            account_public_key,
            validator_count,
            fee_human,
            performance_1day: 0.0,
            performance_1month: 0.0,
        };
        let connection = &mut self.get_connection();
        let result = diesel::insert_into(schema::operator::table)
            .values(&operator)
            .execute(connection);

        return self.transfer_query_qesult(result);
    }

    fn update_operator(&self, id: u32, name: String, account_public_key: String, status: Status, validator_count: u32, fee_human: f32, performance_1day: f32, performance_1month: f32) -> WriteResult {
        let update_operator = operator::UpdateOperator {
            name,
            status: status.to_string(),
            account_public_key,
            validator_count,
            fee_human,
            performance_1day,
            performance_1month,
        };
        let connection = &mut self.get_connection();
        let result = diesel::update(schema::operator::table)
            .filter(schema::operator::dsl::id.eq(id))
            .set(update_operator)
            .execute(connection);

        return self.transfer_query_qesult(result);
    }

    fn get_all_operators(&self) -> Result<Vec<Operator>, String> {
        let connection = &mut self.get_connection();

        let result = schema::operator::dsl::operator
            .load::<Operator>(connection);
        match result {
            Ok(operators) => {
                return Ok(operators);
            }
            Err(error) => {
                return Err(format!("load data from mysql error: {}", error.to_string()));
            }
        }
    }

    fn get_all_accounts(&self) -> Result<Vec<Account>, String> {
        let connection = &mut self.get_connection();

        let result = schema::account::dsl::account
            .load::<Account>(connection);
        match result {
            Ok(accounts) => {
                return Ok(accounts);
            }
            Err(error) => {
                return Err(format!("load data from mysql error: {}", error.to_string()));
            }
        }
    }


    fn add_account(&self, public_key: String, ssv_balance_human: f32) -> WriteResult {
        let account = NewAccount {
            public_key,
            ssv_balance_human,
            is_liquidation: false
        };

        let connection = &mut self.get_connection();

        let result = diesel::insert_into(schema::account::table)
            .values(&account)
            .execute(connection);

        return self.transfer_query_qesult(result);
    }


    fn get_account(&self, public_key: String) -> Option<Account> {
        let connection = &mut self.get_connection();
        return schema::account::dsl::account
            .filter(schema::account::dsl::public_key.eq(public_key.clone()))
            .first(connection)
            .optional()
            .expect("get account error");
    }

    fn update_account(&self, public_key: String, ssv_balance_human: f32, is_liquidation: bool) -> WriteResult {
        let connection = &mut self.get_connection();
        let account = UpdateAccount {
            ssv_balance_human,
            is_liquidation,
        };

        let result = diesel::update(schema::account::table)
            .filter(schema::account::dsl::public_key.eq(public_key))
            .set(account)
            .execute(connection);

        return self.transfer_query_qesult(result);
    }

    fn add_decided(&self, role: String, validator_public_key: String, signature: String, height: u32, round: u32, identifier: String, message_type: u32) -> WriteResult {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let decided = NewDecided {
            role,
            validator_public_key,
            signature,
            height,
            round,
            identifier,
            message_type,
            timestamp,
        };
        let connection = &mut self.get_connection();

        // match diesel::insert_into(schema::decided::table)
        //     .values(&decided)
        //     .execute(connection) {
        //     Ok(size) => {
        //         return Ok(());
        //     }
        //     Err(error) => {
        //         return Err(format!("write decided to mysql error: {}", error.to_string()));
        //     }
        // }


        let result = diesel::insert_into(schema::decided::table)
            .values(&decided)
            .execute(connection);

        return self.transfer_query_qesult(result);
    }

    fn delete_decided_by_validator_height(&self, validator_public_key: String, height: u32) -> Result<usize, String> {
        let connection = &mut self.get_connection();
        return match diesel::delete(schema::decided::table
            .filter(schema::decided::validator_public_key.eq(validator_public_key))
            .filter(schema::decided::height.eq(height)))
            .execute(connection) {
            Ok(number) => {
                Ok(number)
            }
            Err(error) => Err(error.to_string())
        };
    }

    fn delete_operator_decided_record_by_validator_height(&self, validator_public_key: String, height: u32) -> Result<usize, String> {
        let connection = &mut self.get_connection();
        return match diesel::delete(schema::operator_decided_record::table
            .filter(schema::operator_decided_record::validator_public_key.eq(validator_public_key))
            .filter(schema::operator_decided_record::height.eq(height)))
            .execute(connection) {
            Ok(number) => {
                Ok(number)
            }
            Err(error) => Err(error.to_string())
        };
    }

    fn add_operator_decided_record(&self, operator_id: u32, validator_public_key: String, height: u32, round: u32) -> WriteResult {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let operator_decided_record = NewOperatorDecidedRecord {
            operator_id,
            validator_public_key,
            height,
            round,
            timestamp,
        };
        let connection = &mut self.get_connection();

        // let a = diesel::insert_into(schema::operator_decided_record::table)
        // .values(&operator_decided_record)
        //     .execute(connection);

        let result = diesel::insert_into(schema::operator_decided_record::table)
            .values(&operator_decided_record)
            .execute(connection);

        return self.transfer_query_qesult(result);
    }

    fn add_validator(&self, account_public_key: String, public_key: String) -> WriteResult {
        let validator = NewValidator {
            account_public_key,
            public_key,
        };
        let connection = &mut self.get_connection();
        // match diesel::insert_into(schema::validator::table)
        //     .values(&validator)
        //     .execute(connection) {
        //     Ok(size) => {
        //         return Ok(());
        //     }
        //     Err(error) => {
        //         return Err(format!("write validator to mysql error: {}", error.to_string()));
        //     }
        // }


        let result = diesel::insert_into(schema::validator::table)
            .values(&validator)
            .execute(connection);

        return self.transfer_query_qesult(result);
    }

    fn add_validator_operator(&self, validator_public_key: String, operator_id: u32) -> WriteResult {
        let validator_operator = NewValidatorOperator {
            validator_public_key,
            operator_id,
        };
        let connection = &mut self.get_connection();

        let result = diesel::insert_into(schema::validator_operator::table)
            .values(&validator_operator)
            .execute(connection);

        return self.transfer_query_qesult(result);
    }

    fn add_performance_record(&self, operator_id: u32, performance: f32, timestamp: u64) -> WriteResult {
        let performance = NewPerformanceRecord {
            operator_id,
            performance,
            timestamp,
        };
        let connection = &mut self.get_connection();

        let result = diesel::insert_into(schema::performance_record::table)
            .values(&performance)
            .execute(connection);

        return self.transfer_query_qesult(result);
    }

    fn get_operator_validator_count(&self) -> Result<Vec<OperatorValidatorCount>, String> {
        let connection = &mut self.get_connection();
        match schema::operator_decided_record::table
            .group_by(schema::operator_decided_record::operator_id)
            .select((schema::operator_decided_record::operator_id, diesel::dsl::count(schema::operator_decided_record::operator_id)))
            .load::<(u32, i64)>(connection) {
            Ok(items) => {
                let mut results: Vec<OperatorValidatorCount> = vec![];
                for (operator_id, count) in items {
                    let ovc = OperatorValidatorCount {
                        operator_id,
                        count: count as u32,
                    };
                    results.push(ovc);
                }
                return Ok(results);
            }
            Err(error) => {
                return Err("read from mysql error".to_string());
            }
        }
    }

    fn get_operator_decided_count(&self, start_timestamp: u64) -> Result<Vec<OperatorDecidedCount>, String> {
        let connection = &mut self.get_connection();
        match schema::operator_decided_record::table
            .filter(schema::operator_decided_record::timestamp.ge(start_timestamp))
            .group_by(schema::operator_decided_record::operator_id)
            .select((schema::operator_decided_record::operator_id, diesel::dsl::count(schema::operator_decided_record::operator_id)))
            .load::<(u32, i64)>(connection) {
            Ok(items) => {
                let mut results: Vec<OperatorDecidedCount> = vec![];
                for (operator_id, count) in items {
                    let ovc = OperatorDecidedCount {
                        operator_id,
                        count: count as u32,
                    };
                    results.push(ovc);
                }
                return Ok(results);
            }
            Err(error) => {
                return Err("read from mysql error".to_string());
            }
        }
    }


    fn delete_operator_decided_before_timestamp(&self, timestamp: u64) -> Result<usize, String> {
        let connection = &mut self.get_connection();
        return match diesel::delete(schema::operator_decided_record::table
            .filter(schema::operator_decided_record::timestamp.le(timestamp)))
            .execute(connection) {
            Ok(number) => {
                Ok(number)
            }
            Err(error) => Err(error.to_string())
        };
    }

    fn delete_decided_before_timestamp(&self, timestamp: u64) -> Result<usize, String> {
        let connection = &mut self.get_connection();
        return match diesel::delete(schema::decided::table
            .filter(schema::decided::timestamp.le(timestamp)))
            .execute(connection) {
            Ok(number) => {
                Ok(number)
            }
            Err(error) => Err(error.to_string())
        };
    }

    fn transfer_query_qesult(&self, result: QueryResult<usize>) -> WriteResult {
        match result {
            Ok(size) => {
                return WriteResult::Normal;
            }
            Err(error) => {
                match error {
                    diesel::result::Error::DatabaseError(kind, info) => {
                        match kind {
                            UniqueViolation => {
                                return WriteResult::UniqueViolation;
                            }
                            _ => {
                                return WriteResult::Other("DatabaseError".to_string());
                            }
                        }
                    }
                    _ => {
                        return WriteResult::Other(error.to_string());
                    }
                }
            }
        }
    }


    fn add_tag(&self, key: String, value: String) -> WriteResult {
        let tag = NewTag {
            key,
            value,
        };

        let connection = &mut self.get_connection();

        let result = diesel::insert_into(schema::tag::table)
            .values(&tag)
            .execute(connection);

        return self.transfer_query_qesult(result);
    }

    fn get_tag(&self, key: String) -> Option<Tag> {
        let connection = &mut self.get_connection();
        return schema::tag::dsl::tag
            .filter(schema::tag::dsl::key.eq(key))
            .first(connection)
            .optional()
            .expect("get tag error");
    }


    fn update_tag(&self, key: String, value: String) -> WriteResult {
        let tag = UpdateTag {
            value,
        };

        let connection = &mut self.get_connection();

        let result = diesel::update(schema::tag::table)
            .filter(schema::tag::dsl::key.eq(key))
            .set(tag)
            .execute(connection);

        return self.transfer_query_qesult(result);
    }
}


// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

#[cfg(test)]
mod tests {
    use diesel::dsl::sql;
    use crate::tag::NewTag;
    use super::*;
    use crate::validator::NewValidator;

    #[test]
    fn save_operator() {
        println!("starting...");
        let connect_url = "mysql://root:wonders,1@192.168.1.128:3308/ssv_alert_service_5".to_string();
        let storage = MysqlStorage::new(connect_url);
        storage.add_operator(128, "tank".to_string(), "0x0001".to_string(), Status::Active, 423, 1.2);
        println!("ended")
    }

    #[test]
    fn test_save() {
        println!("starting...");
        let connect_url = "mysql://root:wonders,1@192.168.1.128:3308/ssv_alert_service_5".to_string();
        let storage = MysqlStorage::new(connect_url);
        let validator = NewValidator {
            account_public_key: "0xx11".to_string(),
            public_key: "a6170688b3553b6b34419e891290c4c21aa12088dd4acc2eae6a1f2bb8c909d773be1285892aaa55de626f02f4c44f99".to_string(),
        };
        // storage.save(schema::validator::table, validator);
        println!("ended")
    }

    #[test]
    fn test_get_all_operator() {
        println!("starting...");
        let connect_url = "mysql://root:wonders,1@192.168.1.128:3308/ssv_alert_service_5".to_string();
        let storage = MysqlStorage::new(connect_url);
        let ops = storage.get_all_operators();
        println!("a")
    }

    #[test]
    fn test_update_operator() {
        println!("starting...");
        let connect_url = "mysql://root:wonders,1@192.168.1.128:3308/ssv_alert_service_5".to_string();
        let storage = MysqlStorage::new(connect_url);
        let ops = storage.update_operator(
            127,
            "0x000".to_string(),
            "0x0001".to_string(),
            Status::Removed,
            444,
            1.1,
            1.2,
            23.3,
        );
        println!("ended")
    }

    #[test]
    fn test_1() {
        let result = test_calc_performance();
    }

    struct OperatorDecidedCount {
        operator_id: u32,
        count: i64,
    }

    fn print_type_of<T>(_: &T) {
        let t = std::any::type_name::<T>();
        println!("{}", std::any::type_name::<T>())
    }

    fn test_calc_performance() {
        println!("starting...");
        let connect_url = "mysql://root:wonders,1@192.168.1.128:3308/ssv_alert_service_5".to_string();
        let storage = MysqlStorage::new(connect_url);
        let connection = &mut storage.get_connection();

        match schema::operator_decided_record::table
            .group_by(schema::operator_decided_record::operator_id)
            .select((schema::operator_decided_record::operator_id, diesel::dsl::count(schema::operator_decided_record::operator_id)))
            .load::<(u32, i64)>(connection) {
            Ok(r) => {
                for item in r {
                    let (operator_id, count) = item;
                    // let c = count.as_u32();

                    let a: i64 = 32;
                    // let b = a as u32;
                    // print_type_of(item);
                    println!("lll")

                    // let b = item.get(1);
                    // println!(item.get(0));
                }
            }
            Err(error) => {}
        }
    }

    #[test]
    fn test_add_tag() {
        println!("starting...");
        let connect_url = "mysql://root:wonders,1@192.168.1.128:3308/ssv_alert_service_5".to_string();
        let storage = MysqlStorage::new(connect_url);
        let ops = storage.add_tag(
            "name".to_string(),
            "zhangsan".to_string(),
        );
        println!("ended")
    }

    #[test]
    fn test_get_tag() {
        println!("starting...");
        let connect_url = "mysql://root:wonders,1@192.168.1.128:3308/ssv_alert_service_5".to_string();
        let storage = MysqlStorage::new(connect_url);
        let tag = storage.get_tag("name".to_string());
        println!("tag: {:?}", tag)
    }

    #[test]
    fn test_get_tag2() {
        println!("starting...");
        let connect_url = "mysql://root:wonders,1@192.168.1.128:3308/ssv_alert_service_5".to_string();
        let storage = MysqlStorage::new(connect_url);
        let tag = storage.get_tag("name".to_string());
        println!("tag: {:?}", tag)
    }

    #[test]
    fn test_delete_decided_by_validator_height() {
        println!("starting...");
        // 16363600,ATTESTER,b4cf25b2ebd994524c6741a8df82d59bc98663e46cc75a05e9ebd560532dcad73e1a0156c885d4dcd2f56da28f6ddc1c,tnTMYzC7soFCxtG5VDEaaX8RlWY/LNI6pCHHY2w/5Y5bDadK3xaUlA5eotVW53xKEghdhdV1NVh6smY9OTYKbghJy6DE7yyFh23KAFX20ZNK0CYLcKYZ1RPJaGvENvpD,16652,1,tM8lsuvZlFJMZ0Go34LVm8mGY+Rsx1oF6evVYFMtytc+GgFWyIXU3NL1baKPbdwcAAAAAA==,2,1669623127
        let connect_url = "mysql://root:wonders,1@192.168.1.128:3308/ssv_alert_service_5".to_string();
        let storage = MysqlStorage::new(connect_url);
        let result = storage.delete_decided_by_validator_height(
            "aec22129fae7fdd21402d49e418fb86575d11b145e64e5f86bc61fd29c6f9cdd62a69e9e79ab9cc79555d427c39497ba".to_string(),
            1113);
        println!("result: {:?}", result)
    }

    #[test]
    fn test_delete_operator_decided_record_by_validator_height() {
        println!("starting...");
        // 16363600,ATTESTER,b4cf25b2ebd994524c6741a8df82d59bc98663e46cc75a05e9ebd560532dcad73e1a0156c885d4dcd2f56da28f6ddc1c,tnTMYzC7soFCxtG5VDEaaX8RlWY/LNI6pCHHY2w/5Y5bDadK3xaUlA5eotVW53xKEghdhdV1NVh6smY9OTYKbghJy6DE7yyFh23KAFX20ZNK0CYLcKYZ1RPJaGvENvpD,16652,1,tM8lsuvZlFJMZ0Go34LVm8mGY+Rsx1oF6evVYFMtytc+GgFWyIXU3NL1baKPbdwcAAAAAA==,2,1669623127
        let connect_url = "mysql://root:wonders,1@192.168.1.128:3308/ssv_alert_service_5".to_string();
        let storage = MysqlStorage::new(connect_url);
        let result = storage.delete_operator_decided_record_by_validator_height(
            "aec22129fae7fdd21402d49e418fb86575d11b145e64e5f86bc61fd29c6f9cdd62a69e9e79ab9cc79555d427c39497ba".to_string(),
            1113);
        println!("result: {:?}", result)
    }

    #[test]
    fn test_update_tag() {
        println!("starting...");
        let connect_url = "mysql://root:wonders,1@192.168.1.128:3308/ssv_alert_service_5".to_string();
        let storage = MysqlStorage::new(connect_url);
        let result = storage.update_tag("name".to_string(), "lisi".to_string());
        println!("result: {:?}", result)
    }
}


// impl NonNull<mysqlclient_sys::st_mysql> for MysqlStorage {
//
// }


pub async fn test_get_all_accounts() {
    let connect_url = "mysql://root:wonders,1@192.168.1.128:3308/ssv_alert_service_5".to_string();

    let connection = &mut MysqlConnection::establish(connect_url.as_str())
        .unwrap_or_else(|_| panic!("Error connecting to {}", connect_url));

    let result = schema::account::dsl::account
        .load::<Account>(connection);
    println!("end")
}