use crate::error::HarnessError;
use crate::net::connection::TestConnection;
use core::net::Ipv4Addr;
use op::recv::RecvOpcode;
use packet::io::error::IOError::{ReadError, WriteError};
use packet::model::Packet;
use packet::prelude::*;
use std::io::Cursor;

pub const PHASE: &str = "server redirect";

pub struct ServerRedirectResult {
    pub ip: Ipv4Addr,
    pub port: i16,
    pub char_id: i32,
}

pub async fn assert_server_redirect(
    mut conn: TestConnection,
    char_id: i32,
) -> Result<i16, HarnessError> {
    dbg!(PHASE);
    conn.send_packet(build_server_redirect(char_id)?, PHASE)
        .await?;
    let port: i16 = assert_server_redirect_result(&mut conn, char_id).await?;
    Ok(port)
}

pub fn build_server_redirect(character_id: i32) -> Result<Packet, HarnessError> {
    let mut packet = Packet::new_empty();
    packet
        .write_short(RecvOpcode::CharSelect as i16)
        .map_err(|e| HarnessError::PacketIOError(WriteError(e)))?;
    packet
        .write_int(character_id)
        .map_err(|e| HarnessError::PacketIOError(WriteError(e)))?;
    packet
        .write_str_with_length(String::from(""))
        .map_err(|e| HarnessError::PacketIOError(WriteError(e)))?;
    packet
        .write_str_with_length(String::from(""))
        .map_err(|e| HarnessError::PacketIOError(WriteError(e)))?;
    Ok(packet)
}

async fn assert_server_redirect_result(
    conn: &mut TestConnection,
    char_id: i32,
) -> Result<i16, HarnessError> {
    let packet: Packet = conn.read_packet(PHASE).await?;
    let result: ServerRedirectResult = read_server_redirect_packet(&packet)?;
    assert_eq!(result.char_id, char_id);
    Ok(result.port)
}

pub fn read_server_redirect_packet(packet: &Packet) -> Result<ServerRedirectResult, HarnessError> {
    let mut cursor = Cursor::new(&packet.bytes[..]);
    cursor
        .read_short()
        .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
    cursor
        .read_short()
        .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
    let ip_bytes = cursor
        .read_bytes(4)
        .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
    let port = cursor
        .read_short()
        .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
    let char_id = cursor
        .read_int()
        .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
    Ok(ServerRedirectResult {
        ip: Ipv4Addr::new(ip_bytes[0], ip_bytes[1], ip_bytes[2], ip_bytes[3]),
        port,
        char_id,
    })
}
