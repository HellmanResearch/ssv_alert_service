
use diesel::prelude::*;
use crate::account::Account;


#[derive(Queryable)]
#[diesel(belongs_to(Account))]
pub struct Validator {
    account_public_key: String,
    public_key: String,
    performance: u32,
}