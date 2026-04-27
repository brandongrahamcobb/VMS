use crate::net::error::NetworkError;
use crate::net::packet::core::Packet;
use crate::net::packet::error::PacketError;
use crate::net::packet::io::error::IOError::WriteError;
use crate::prelude::*;

pub async fn build_handshake_packet(
    recv_iv: &Vec<u8>,
    send_iv: &Vec<u8>,
    version: i16,
) -> Result<Packet, NetworkError> {
    let mut packet = Packet::new_empty();
    packet
        .write_short(0x0E)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_short(version)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;

    // Not sure what this part is meant to represent...
    // HeavenClient doesn't seem to care for these values but the
    // official clients do...
    packet
        .write_short(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_byte(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_bytes(&recv_iv)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_bytes(&send_iv)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_byte(8) // Locale byte
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    Ok(packet)
}
