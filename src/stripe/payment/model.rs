use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::payments)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub(crate) struct Payment {
    id: i32,
    created: i32,
    amount: i32,
    currency: String,
    status: String,
    customer_id: i32,
    method_type: String,
}
