use thiserror::Error;

#[derive(Debug, Error)]
pub enum AccountError {
    #[error("Requested account was not found in account model layer: {0}")]
    NotFound(i16),
}
