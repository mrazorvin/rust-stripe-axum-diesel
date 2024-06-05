use diesel::prelude::*;

// Switch to postgress and use pool, https://github.com/tokio-rs/axum/blob/main/examples/diesel-async-postgres/src/main.rsfn
pub(crate) fn db() -> SqliteConnection {
    SqliteConnection::establish("file:db.sqlite").unwrap()
}
