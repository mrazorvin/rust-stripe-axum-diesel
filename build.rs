use diesel::{Connection, SqliteConnection};
use diesel_migrations::{
    embed_migrations, EmbeddedMigrations, HarnessWithOutput, MigrationHarness,
};
use std::error::Error;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    drop(rusqlite::Connection::open("./db.sqlite")?);
    let mut connection = SqliteConnection::establish("file:./db.sqlite")?;
    let mut harness = HarnessWithOutput::write_to_stdout(&mut connection);
    MigrationHarness::run_pending_migrations(&mut harness, MIGRATIONS)?;

    Ok(())
}
