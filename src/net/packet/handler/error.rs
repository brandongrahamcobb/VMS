use thiserror::Error;

#[derive(Debug, Error)]
pub enum HandlerError {
    #[error("Login error in packet handler layer")]
    LoginError,
}
