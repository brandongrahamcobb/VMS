use thiserror::Error;

#[derive(Debug, Error)]
pub enum SessionError {
    #[error("Failed to locate session: {0}")]
    NotFound(i32),

    #[error("Failed to locate the session HashSet")]
    NoSessions,

    #[error("Failed to retrieve account in session: {0}")]
    NoAccount(i32),

    #[error("Failed to retrieve channel in session: {0}")]
    NoChannel(i32),

    #[error("Failed to retrieve world in session: {0}")]
    NoWorld(i32),

    #[error("Failed to retrieve hardware id in session: {0}")]
    NoHwid(i32),

    #[error("Failed to validate successful authentication in session: {0}")]
    NotAuthenticated(i32),

    #[error("Failed to retrieve character in session: {0}")]
    NoChar(i32),

    #[error("Failed to retrieve map in session: {0}")]
    NoMap(i32),

    #[error("Missing field in session: {0}")]
    MissingField(i32),
}
