use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::charge)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub(crate) struct Charge {
    #[allow(unused)]
    id: i32,
}
