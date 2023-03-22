// use core::panicking::panic;
// use std::f32::consts::E;
// use core::num::flt2dec::strategy::grisu::max_pow10_no_more_than;
use std::fmt::{Display, Formatter};
use diesel::prelude::*;
use diesel::sql_types;
use crate::schema::operator;
use diesel::deserialize::{FromSqlRow, FromSql};
use diesel::sql_types::Text;


use crate::account::Account;

use crate::schema;
use crate::models;
use diesel::row::Row;


#[derive(Queryable)]
pub struct Operator {
    pub id: u32,
    pub name: String,
    pub account_public_key: String,
    pub status: String,
    pub validator_count: u32,
    pub fee_human: f32,
    pub performance_1day: f32,
    pub performance_1month: f32,
}

// #[derive(diesel_derive_enum::DbEnum, Debug)]
// #[derive(die)]
pub enum Status {
    Active,
    Inactive,
    Removed,
}

impl Status {
    pub fn from_string(status_string: String) -> Result<Self, String> {
        if status_string == "active" {
            return Ok(Status::Active)
        }else if status_string == "inactive" {
            return Ok(Status::Inactive)
        }else if status_string == "removed" {
            return Ok(Status::Removed)
        }
        return Err(format!("can not transfer {} to Status", status_string));
    }
}

// impl FromSqlRow<Text, diesel::mysql::Mysql> for Status {
//     fn build_from_row<'a>(row: &impl Row<'a, diesel::mysql::Mysql>) -> diesel::deserialize::Result<Status> {
//         // let r = row.take();
//         return Ok(Status::Active)
//     }
// }

impl Display for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::Active => f.write_str("active"),
            Status::Inactive => f.write_str("inactive"),
            Status::Removed => f.write_str("removed"),
        }
    }
}


#[derive(Insertable)]
#[diesel(table_name = operator)]
pub struct NewOperator {
    pub id: u32,
    pub name: String,
    pub status: String,
    pub account_public_key: String,
    pub validator_count: u32,
    pub fee_human: f32,
    pub performance_1day: f32,
    pub performance_1month: f32,
}


#[derive(AsChangeset)]
#[diesel(table_name = operator)]
pub struct UpdateOperator {
    pub name: String,
    pub status: String,
    pub account_public_key: String,
    pub validator_count: u32,
    pub fee_human: f32,
    pub performance_1day: f32,
    pub performance_1month: f32,
}


// pub fn add_operator(conn: &mut MysqlConnection, id: u32, name: String, account_public_key: String, status: Status, validator_count: u32, fee_human: f32) {
//
//     let operator = NewOperator {
//         id,
//         name,
//         status: status.to_string(),
//         account_public_key,
//         validator_count,
//         fee_human,
//     };
//
//     diesel::insert_into(schema::operator::table)
//         .values(&operator)
//         .execute(conn)
//         .expect("error saving new post");
//
// }

#[cfg(test)]
mod tests {

    use crate::operator::{Status};
    // use crate::establish_connection;

    // #[test]
    // fn test_add_operator() {
    //     println!("test_add_operator");
    //     let mut conn = establish_connection();
    //     let r = add_operator(&mut conn, 127, "hellman".to_string(), "0x0001".to_string(), Status::Active, 423, 1.2);
    // }
    //
    // #[test]
    // fn test_enum_to_string() {
    //     let status = Status::Active;
    //
    // }
}


pub struct OperatorValidatorCount {
    pub operator_id: u32,
    pub count: u32
}