use std::fs::create_dir_all;

use rusqlite::Connection;
use rusqlite_migration::{Migrations, M};
use tauri::{AppHandle, Manager};
use thiserror::Error;

use crate::library::{DATABASE_DIR, DATABASE_FILE};

pub fn open_database_connection(app: &AppHandle) -> Result<Connection, DatabaseError> {
    let app_data = app.path().app_data_dir()?;

    let db_dir = app_data.join(DATABASE_DIR);
    create_dir_all(&db_dir)?;

    let db_path = db_dir.join(DATABASE_FILE);

    let mut connection = Connection::open(&db_path)?;

    run_migrations(&mut connection)?;

    Ok(connection)
}

fn run_migrations(connection: &mut Connection) -> Result<(), rusqlite_migration::Error> {
    let migrations = Migrations::new(vec![M::up(include_str!("migrations/0001_initial.sql"))]);

    migrations.to_latest(connection)?;

    Ok(())
}

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("Unable to get app data directory")]
    Tauri(#[from] tauri::Error),

    #[error("Filesystem error")]
    IO(#[from] std::io::Error),

    #[error("Database connection error: {0}")]
    Connection(#[from] rusqlite::Error),

    #[error("Database migration error: {0}")]
    Migration(#[from] rusqlite_migration::Error),
}
