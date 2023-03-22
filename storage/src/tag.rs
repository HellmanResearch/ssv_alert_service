use diesel::AsChangeset;
use diesel::prelude::Queryable;
use diesel::prelude::Insertable;

use crate::schema::tag;


#[derive(Queryable, Debug)]
pub struct Tag {
    pub key: String,
    pub value: String,
}


#[derive(Insertable)]
#[diesel(table_name = tag)]
pub struct NewTag {
    pub key: String,
    pub value: String,
}


#[derive(AsChangeset)]
#[diesel(table_name = tag)]
pub struct UpdateTag {
    pub value: String
}
