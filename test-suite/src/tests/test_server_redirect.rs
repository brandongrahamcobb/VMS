use crate::error::HarnessError;
use crate::net::connection::TestConnection;
use crate::tests::test_create_char::CHAR_ID;
use core::net::Ipv4Addr;
use packet::io::error::IOError::ReadError;
use packet::model::Packet;
use packet::prelude::*;
use std::io::Cursor;

pub const PHASE: &str = "server redirect";

pub struct ServerRedirectResult {
    pub ip: Ipv4Addr,
    pub port: u8,
    pub char_id: i32,
}

pub async fn assert_server_redirect(mut conn: TestConnection) -> Result<u8, HarnessError> {
    dbg!(PHASE);
    let port: u8 = assert_server_redirect_result(&mut conn).await?;
    Ok(port)
}

async fn assert_server_redirect_result(conn: &mut TestConnection) -> Result<u8, HarnessError> {
    let packet: Packet = conn.read_packet(PHASE).await?;
    let result: ServerRedirectResult = read_server_redirect_packet(&packet)?;
    assert_eq!(result.char_id, CHAR_ID);
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
        port: port as u8,
        char_id,
    })
}
