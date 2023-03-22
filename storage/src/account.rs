
use diesel::prelude::Queryable;
use diesel::prelude::Insertable;
use diesel::prelude::AsChangeset;

use crate::schema::account;



// use crate::{establish_connection, schema};

#[derive(Queryable, Clone)]
pub struct Account {
    pub public_key: String,
    pub ssv_balance_human: f32,
    pub is_liquidation: bool
}



#[derive(Insertable)]
#[diesel(table_name = account)]
pub struct NewAccount {
    pub public_key: String,
    pub ssv_balance_human: f32,
    pub is_liquidation: bool

}


#[derive(AsChangeset)]
#[diesel(table_name = account)]
pub struct UpdateAccount {
    pub ssv_balance_human: f32,
    pub is_liquidation: bool
}
