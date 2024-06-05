use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::payments)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub(crate) struct Payment {
    #[allow(unused)]
    id: i32,
}
