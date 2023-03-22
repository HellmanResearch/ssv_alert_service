// use diesel::deserialize::FromSqlRow;
// use diesel::prelude::*;
// use crate::schema::posts;
// use crate::schema::operator;
//
// #[derive(Queryable, Debug)]
// pub struct Post {
//     pub id: i32,
//     pub title: String,
//     pub body: String,
//     pub published: bool,
// }
//
// #[derive(Insertable)]
// #[diesel(table_name = posts)]
// pub struct NewPost<'a> {
//     pub title: &'a str,
//     pub body: &'a str,
// }
//
//
//
// use diesel::prelude::*;
// use diesel::sql_types;
//
// use crate::account::Account;
//
// use crate::schema;
//
//
// #[derive(Queryable)]
// pub struct Operator {
//     pub id: u32,
//     pub name: String,
//     pub account_public_key: String,
//     pub status: Status,
//     pub validator_count: u32,
//     pub fee_human: f32,
// }
//
// pub enum Status {
//     Active,
//     Inactive,
//     Removed,
// }
//
//
// #[derive(Insertable)]
// #[diesel(table_name = operator)]
// pub struct NewOperator {
//     id: u32,
//     name: String,
//     account_public_key: String,
//     validator_count: u32,
// }