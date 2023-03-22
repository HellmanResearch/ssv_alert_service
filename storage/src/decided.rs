
use diesel::prelude::Queryable;
use diesel::prelude::Insertable;

use crate::schema::{decided, operator_decided_record};



// use crate::{establish_connection, schema};

#[derive(Queryable, Clone)]
pub struct Decided {
    pub role: String,
    pub validator_public_key: String,
    pub signature: String,
    pub height: u64,
    pub round: u32,
    pub identifier: String,
    pub message_type: u32,
    pub timestamp: u32,
}


#[derive(Queryable, Clone)]
pub struct OperatorDecidedRecord {
    pub decied_id: u64,
    pub operator_id: u32,
    pub validator_public_key: String,
    pub height: u64,
    pub round: u32,
    pub timestamp: u32,
}



#[derive(Insertable)]
#[diesel(table_name = decided)]
pub struct NewDecided {
    pub role: String,
    pub validator_public_key: String,
    pub signature: String,
    pub height: u32,
    pub round: u32,
    pub identifier: String,
    pub message_type: u32,
    pub timestamp: u64,
}



#[derive(Insertable)]
#[diesel(table_name = operator_decided_record)]
pub struct NewOperatorDecidedRecord {
    pub operator_id: u32,
    pub validator_public_key: String,
    pub height: u32,
    pub round: u32,
    pub timestamp: u64,
}


pub struct OperatorDecidedCount {
    pub operator_id: u32,
    pub count: u32
}