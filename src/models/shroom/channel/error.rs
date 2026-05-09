use thiserror::Error;

#[derive(Debug, Error)]
pub enum ChannelError {
    #[error("Requested channel was not found in channel model layer: {0}")]
    NotFound(i16),

    #[error("Unexpected error in channel model layer")]
    UnexpectedError,

    #[error("No capacity in channel model layer: {0}")]
    NoCapacity(i16),
}
