// use core::num::flt2dec::strategy::grisu::max_pow10_no_more_than;
use std::fmt::{Display, Formatter};
use diesel::prelude::*;
use diesel::sql_types;
use crate::schema::{operator, validator, validator_operator};


use crate::account::Account;
use crate::operator::{Status, NewOperator};

use crate::schema;
use crate::models;

#[derive(Queryable)]
pub struct Validator {
    account_public_key: String,
    public_key: String,
}


#[derive(Queryable)]
pub struct ValidatorOperator {
    validator_public_key: String,
    operator_id: u32,
}



#[derive(Insertable)]
#[diesel(table_name = validator)]
pub struct NewValidator {
    pub account_public_key: String,
    pub public_key: String,
}


#[derive(Insertable)]
#[diesel(table_name = validator_operator)]
pub struct NewValidatorOperator {
    pub validator_public_key: String,
    pub operator_id: u32,
}

