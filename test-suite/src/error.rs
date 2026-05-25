use config::error::ConfigError;
use db::error::DatabaseError;
use packet::io::error::IOError;
use state::error::StateError;
use std::string::FromUtf8Error;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum HarnessError {
    #[error("{0} in test harness layer")]
    DockerComposeError(String),

    #[error("TCP connection error in test harness layer")]
    ConnectionError,

    #[error("Account error in test harness layer: {0}")]
    AccountError(bcrypt::BcryptError),

    #[error("{0} in test harness layer")]
    CargoError(String),

    #[error("{0} in test harness layer")]
    DockerInfoError(String),

    #[error("{0} in test harness layer")]
    DockerVersionError(String),

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

    #[error("State error in test harness layer")]
    StateError(#[from] StateError),

    #[error("From UTF-8 error in test harness layer")]
    FromUtf8Error(#[from] FromUtf8Error),
}
