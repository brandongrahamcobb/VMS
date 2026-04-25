use diesel;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("Failed to connect to database in database layer")]
    DatabaseConnectionError,

    #[error("Invalid database url error: {0}")]
    InvalidDatabaseUrlError(String),

    #[error("Entry in database not found in database layer")]
    DieselError(#[from] diesel::result::Error),
}
