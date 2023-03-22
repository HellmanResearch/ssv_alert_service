
use diesel::prelude::*;
use schema::users::dsl::*;

use crate::account::Account;


#[derive(Queryable)]
#[diesel(belongs_to(Operator))]
pub struct Operator {
    id: u32,
    name: String,
    account_public_key: String,
    status: Status,
    validator_count: u32,
    fee_human: f32,
}

pub enum Status {
    Active,
    Inactive,
    Removed,
}

#[derive(Insertable)]
#[diesel(table_name = operator)]
pub struct NewOperator<'a> {
    pub id: &'a u32,
    pub name: &'a str,
    pub account_public_key: &'a str,
    pub status: &'a str,
    pub validator_count: &'a u32,
    pub fee_human: &'a f32,
}


pub fn add_operator(conn: &mut MysqlConnection, id: u32, name: String, account_public_key: String, status: Status, validator_count: u32, fee_human: f32) -> Post {
    use crate::schema::posts;

    let new_operator = NewOperator {
        id： &id,
        name,
        account_public_key,
        status,
        validator_count,
        fee_human
    };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .execute(conn)
        .expect("error saving new post");

    return posts::table.order(posts::id.desc()).first(conn).unwrap();
}