use crate::config::error::ConfigError;
use crate::db::error::DatabaseError;
use crate::models::error::ModelError;
use crate::net::packet::error::PacketError;
use crate::runtime::error::SessionError;
use bcrypt::BcryptError;
use std::time::SystemTimeError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum NetworkError {
    #[error("Config error in network layer")]
    ConfigError(#[from] ConfigError),

    #[error("Packet error in network layer")]
    PacketError(#[from] PacketError),

    #[error("Database error in network layer")]
    DatabaseError(#[from] DatabaseError),

    #[error("System time error in network layer")]
    SystemTimeError(#[from] SystemTimeError),

    #[error("Unexpected error in network layer")]
    UnexpectedError,

    #[error("Integer conversion error in network layer")]
    IntConversion(#[from] std::num::TryFromIntError),

    #[error("Bcrypt error in network layer")]
    CryptError(#[from] BcryptError),

    #[error("Session error in network layer")]
    SessionError(#[from] SessionError),

    #[error("Model error in network layer")]
    ModelError(#[from] ModelError),
}
