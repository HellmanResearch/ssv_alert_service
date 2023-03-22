// use core::num::flt2dec::strategy::grisu::max_pow10_no_more_than;
use std::fmt::{Display, Formatter};
use diesel::prelude::*;
use diesel::sql_types;
use crate::schema::performance_record;


use crate::account::Account;

use crate::schema;
use crate::models;


#[derive(Queryable)]
#[diesel(table_name = performance_record)]
pub struct PerformanceRecord {
    pub id: u32,
    pub operator_id: u32,
    pub performance: f32,
    pub timestamp: u64,
}

#[derive(Insertable)]
#[diesel(table_name = performance_record)]
pub struct NewPerformanceRecord {
    pub operator_id: u32,
    pub performance: f32,
    pub timestamp: u64,
}