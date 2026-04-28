use crate::config::error::ConfigError;
use crate::db::error::DatabaseError;
use crate::models::error::ModelError;
use crate::net::error::NetworkError;
use thiserror::Error;
use tokio::task::JoinError;

#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("Config error in runtime layer")]
    ConfigError(#[from] ConfigError),

    #[error("Network error in runtime layer")]
    NetworkError(#[from] NetworkError),

    #[error("Concurrency join error in runtime layer")]
    JoinError(#[from] JoinError),

    #[error("Unexpected end of output in runtime layer")]
    UnexpectedOf(#[from] std::io::Error),

    #[error("Failed to connect to server in runtime layer")]
    FailedServerConnection(#[from] RuntimeServerConnectionError),

    #[error("Failed to create relay in runtime layer")]
    FailedRelayCreation(#[from] RuntimeRelayCreationError),

    #[error("Failed database in runtime layer")]
    DatabaseError(#[from] DatabaseError),

    #[error("Unexpected error in runtime layer")]
    UnexpectedError,

    #[error("Environment loading error in runtime layer")]
    DotenvError(#[from] dotenvy::Error),

    #[error("Session error in runtime layer")]
    SessionError(#[from] SessionError),

    #[error("Unsupported opcode error in runtime layer: {0} {1}")]
    UnsupportedOpcodeError(i16, String),

    #[error("Model error in runtime layer")]
    ModelError(#[from] ModelError),
}

#[derive(Debug, Error)]
pub enum RuntimeServerConnectionError {
    #[error("Failed login server connection: {0}")]
    FailedLoginServerConnection(String),

    #[error("Failed world server connection: {0}")]
    FailedWorldServerConnection(String),
}

#[derive(Debug, Error)]
pub enum RuntimeRelayCreationError {
    #[error("Failed to create login relay: {0}")]
    FailedLoginRelayCreation(String),

    #[error("Failed to create world relay: {0}")]
    FailedWorldRelayCreation(String),
}

#[derive(Debug, Error)]
pub enum SessionError {
    #[error("Failed to locate session: {0}")]
    NotFound(i32),

    #[error("Failed to retrieve account in session")]
    NoAccount,

    #[error("Failed to retrieve selected channel in session")]
    NoChannelSelected,

    #[error("Failed to retrieve selected world in session")]
    NoWorldSelected,

    #[error("Failed to retrieve hardware id in session")]
    NoHWID,

    #[error("Failed to validate successful authentication in session")]
    NotAuthenticated,
}
