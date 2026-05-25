use crate::error::HarnessError;
use crate::net::connection::TestConnection;
use op::recv::RecvOpcode;
use packet::io::error::IOError::{ReadError, WriteError};
use packet::model::Packet;
use packet::prelude::*;
use std::io::Cursor;

const PHASE: &str = "server list";

#[derive(Debug, PartialEq, Eq)]
pub enum ServerListResult {
    WorldDetails,
    EndOfList,
}

pub async fn assert_server_list_request(
    mut conn: TestConnection,
) -> Result<TestConnection, HarnessError> {
    conn.send_packet(build_server_list_request()?, PHASE)
        .await?;
    assert_server_list_result(&mut conn, ServerListResult::WorldDetails).await?;
    assert_server_list_result(&mut conn, ServerListResult::EndOfList).await?;
    Ok(conn)
}

pub fn build_server_list_request() -> Result<Packet, HarnessError> {
    let mut packet = Packet::new_empty();
    packet
        .write_short(RecvOpcode::ServerListRequest as i16)
        .map_err(|e| HarnessError::PacketIOError(WriteError(e)))?;
    Ok(packet)
}

fn read_server_list_request_packet(packet: &Packet) -> Result<ServerListResult, HarnessError> {
    let mut cursor = Cursor::new(&packet.bytes[..]);
    cursor
        .read_short()
        .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
    let marker = cursor
        .read_byte()
        .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
    if marker == 0xFF {
        Ok(ServerListResult::EndOfList)
    } else {
        Ok(ServerListResult::WorldDetails)
    }
}

pub async fn assert_server_list_result(
    conn: &mut TestConnection,
    kind: ServerListResult,
) -> Result<(), HarnessError> {
    let packet: Packet = conn.read_packet(PHASE).await?;
    let result: ServerListResult = read_server_list_request_packet(&packet)?;
    assert_eq!(result, kind);
    Ok(())
}
