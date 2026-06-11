use crate::error::HarnessError;
use crate::net::connection::TestConnection;
use config::settings;
use net::packet::io::error::IOError::{ReadError, WriteError};
use net::packet::io::prelude::*;
use net::packet::model::Packet;
use op::recv::RecvOpcode;
use std::io::Cursor;

pub const PHASE: &str = "server list";

#[derive(Debug, PartialEq, Eq)]
pub enum ServerListResult {
    WorldDetails,
    EndOfList,
    LastConnectedWorld,
    RecommendedWorlds,
}

pub async fn assert_server_list_request(
    mut conn: TestConnection,
) -> Result<TestConnection, HarnessError> {
    dbg!(PHASE);
    conn.send_packet(build_server_list_request()?, PHASE)
        .await?;
    let world_count: i8 = settings::get_world_count()?;
    assert_server_list_result(&mut conn, world_count).await?;
    Ok(conn)
}

fn build_server_list_request() -> Result<Packet, HarnessError> {
    let mut packet = Packet::new_empty();
    packet
        .write_short(RecvOpcode::ServerListRequest as i16)
        .map_err(|e| HarnessError::PacketIOError(WriteError(e)))?;
    Ok(packet)
}

fn read_server_list_request_packet(packet: &Packet) -> Result<ServerListResult, HarnessError> {
    let mut cursor = Cursor::new(&packet.bytes[..]);
    let _op = cursor
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

async fn assert_server_list_result(
    conn: &mut TestConnection,
    world_count: i8,
) -> Result<(), HarnessError> {
    for i in 0..world_count + 1 {
        let packet: Packet = conn.read_packet(PHASE).await?;
        let result = read_server_list_request_packet(&packet)?;
        if i == world_count {
            assert_eq!(result, ServerListResult::EndOfList);
        } else {
            assert_eq!(result, ServerListResult::WorldDetails);
        }
    }
    Ok(())
}
