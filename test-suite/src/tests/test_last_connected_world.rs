use crate::error::HarnessError;
use crate::net::connection::TestConnection;
use op::recv::RecvOpcode;
use packet::io::error::IOError::{ReadError, WriteError};
use packet::model::Packet;
use packet::prelude::*;
use std::io::Cursor;

pub const PHASE: &str = "last connected world";
const LAST_CONNECTED_WORLD_ID: i32 = 0;

struct LastConnectedWorldResult {
    world_id: i32,
}

pub async fn assert_last_connected_world(
    mut conn: TestConnection,
) -> Result<TestConnection, HarnessError> {
    dbg!(PHASE);
    conn.send_packet(build_server_list_request()?, PHASE)
        .await?;
    assert_last_connected_world_result(&mut conn).await?;
    Ok(conn)
}

fn build_server_list_request() -> Result<Packet, HarnessError> {
    let mut packet = Packet::new_empty();
    packet
        .write_short(RecvOpcode::ServerListRequest as i16)
        .map_err(|e| HarnessError::PacketIOError(WriteError(e)))?;
    Ok(packet)
}

fn read_last_connected_world_packet(
    packet: &Packet,
) -> Result<LastConnectedWorldResult, HarnessError> {
    let mut cursor = Cursor::new(&packet.bytes[..]);
    cursor
        .read_short()
        .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
    let world_id = cursor
        .read_int()
        .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
    Ok(LastConnectedWorldResult { world_id })
}

async fn assert_last_connected_world_result(conn: &mut TestConnection) -> Result<(), HarnessError> {
    let packet: Packet = conn.read_packet(PHASE).await?;
    let result: LastConnectedWorldResult = read_last_connected_world_packet(&packet)?;
    assert_eq!(result.world_id, LAST_CONNECTED_WORLD_ID);
    Ok(())
}
