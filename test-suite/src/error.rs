use config::error::ConfigError;
use db::{
    account::error::AccountModelError, character::error::CharacterModelError, error::DatabaseError,
};
use net::packet::io::error::IOError;
use std::string::FromUtf8Error;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum HarnessError {
    #[error("{0} in test harness layer")]
    DockerError(String),

    #[error("TCP connection error in test harness layer")]
    ConnectionError,

    #[error("Encryption error in test harness layer: {0}")]
    EncryptionError(bcrypt::BcryptError),
    //
    #[error("Account database model error in test harness layer")]
    AccountModelError(#[from] AccountModelError),

    #[error("Character database model error in test harness layer")]
    CharacterModelError(#[from] CharacterModelError),

    #[error("{0} in test harness layer")]
    CargoError(String),

    #[error("{0} in test harness layer")]
    EndpointError(String),

    #[error("Configuration error in test harness layer")]
    ConfigError(#[from] ConfigError),

    #[error("Packet IO error during {0} against {1} as source {2} in test harness layer")]
    IOError(&'static str, String, std::io::Error),

    #[error("Packet IO error in test harness layer")]
    PacketIOError(#[from] IOError),

    #[error("Database error in test harness layer")]
    DatabaseError(#[from] DatabaseError),

    #[error("Database error in test harness layer")]
    R2d2Error(#[from] r2d2::Error),

    #[error("From UTF-8 error in test harness layer")]
    FromUtf8Error(#[from] FromUtf8Error),
}
