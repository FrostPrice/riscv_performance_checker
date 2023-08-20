use std::error::Error;

use diesel::{
    r2d2::{self, ConnectionManager},
    sqlite::Sqlite,
    SqliteConnection,
};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use log::info;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub type Connection = SqliteConnection;

pub type Pool = r2d2::Pool<ConnectionManager<Connection>>;

fn create_connection_pool(database_url: &str) -> Pool {
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool: {database_url}")
}

fn run_migrations(
    connection: &mut impl MigrationHarness<Sqlite>,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    connection.run_pending_migrations(MIGRATIONS)?;

    Ok(())
}

pub fn migrate_and_config_db(database_url: &str) -> Pool {
    info!("Migrating and Configuring SQLite Database...");
    let pool = create_connection_pool(database_url);

    let mut conn = pool
        .get()
        .expect("Failed to get connection: {database_url}");

    run_migrations(&mut conn).expect("Failed to run migrations.");

    pool
}
