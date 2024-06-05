use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::charge)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub(crate) struct Charge {
    id: i32,
    amount: i32,
    created: i32,
    payment_id: i32,
}
