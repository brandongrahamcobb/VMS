use crate::models::error::ModelError;
use crate::net::packet::handler::error::HandlerError;
use crate::net::packet::io::error::IOError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PacketError {
    #[error("Packet handler error in packet layer")]
    HandlerError(#[from] HandlerError),

    #[error("Packet io error in packet layer")]
    IOError(#[from] IOError),

    #[error("Invalid packet header in packet layer")]
    InvalidHeader,

    #[error("Invalid packet length in packet layer: {0}")]
    InvalidPacketLength(i16),

    #[error("Empty packet in packet layer")]
    EmptyPacket,

    #[error("Model error in packet layer")]
    ModelError(#[from] ModelError),
}
