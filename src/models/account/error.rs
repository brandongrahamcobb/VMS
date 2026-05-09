use thiserror::Error;

#[derive(Debug, Error)]
pub enum AccountError {
    #[error("Requested account was not found in account model layer: {0}")]
    NotFound(i16),

    #[error("Missing field in account model layer: {0}")]
    MissingField(i32),

    #[error("No pic found in account model layer: {0}")]
    NoPic(i32),

    #[error("No id found in account model layer")]
    NoId,

    #[error("No created at time found in account model layer: {0}")]
    NoCreatedAt(i32),
}
