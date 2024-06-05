use diesel::prelude::*;

// TODO switch to postgress and use pool, https://github.com/tokio-rs/axum/blob/main/examples/diesel-async-postgres/src/main.rs
fn db_connection() -> SqliteConnection {
    SqliteConnection::establish("file:db.sqlite").unwrap()
}
