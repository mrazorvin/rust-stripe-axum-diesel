use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::customers)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub(crate) struct Customer {
    #[allow(unused)]
    id: i32,
}
