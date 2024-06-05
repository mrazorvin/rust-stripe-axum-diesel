use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::payments)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Payment {
    pub id: i32,
    pub created: i32,
    pub amount: i32,
    pub currency: String,
    pub status: String,
    pub customer_id: i32,
    pub method_type: String,
}
