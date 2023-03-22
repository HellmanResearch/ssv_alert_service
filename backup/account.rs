
use diesel::prelude::Queryable;

#[derive(Queryable)]
pub struct Account {
    public_key: String,
    ssv_balance_human: f32
}