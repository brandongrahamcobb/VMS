use thiserror::Error;

#[derive(Debug, Error)]
pub enum WorldError {
    #[error("Requested world was not found in world layer: {0}")]
    NotFound(i16),

    #[error("No world selected for in world layer: {0}")]
    NotSelected(i64),
}
