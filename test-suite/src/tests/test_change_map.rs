use crate::error::HarnessError;
use crate::net::connection::TestConnection;
use crate::tests::test_player_logged_in;
use net::packet::io::error::IOError::{ReadError, WriteError};
use net::packet::io::prelude::*;
use net::packet::model::Packet;
use op::recv::RecvOpcode;
use op::send::SendOpcode;
use std::io::Cursor;

pub const FIRST_MAP_WZ: i32 = 20000;
pub const SECOND_MAP_WZ: i32 = 30000;
pub const FIRST_PORTAL_WZ: u8 = 3;
pub const SECOND_PORTAL_WZ: u8 = 2;
pub const PHASE: &str = "change map";
pub const CHANNEL_ID: u8 = 2;
pub const PORT: i16 = 8588;

pub struct SetFieldResult {
    channel_id: i32,
    mode_one: u8,
    mode_two: u8,
    map_wz: i32,
    portal_wz: u8,
}

pub async fn assert_first_change_map(
    mut conn: TestConnection,
) -> Result<TestConnection, HarnessError> {
    dbg!(PHASE);
    conn.send_packet(build_change_map(FIRST_MAP_WZ)?, PHASE)
        .await?;
    assert_change_map_result(&mut conn, FIRST_MAP_WZ, FIRST_PORTAL_WZ).await?;
    Ok(conn)
}

pub async fn assert_second_change_map(
    mut conn: TestConnection,
) -> Result<TestConnection, HarnessError> {
    dbg!(PHASE);
    conn.send_packet(build_change_map(SECOND_MAP_WZ)?, PHASE)
        .await?;
    assert_change_map_result(&mut conn, SECOND_MAP_WZ, SECOND_PORTAL_WZ).await?;
    Ok(conn)
}

async fn assert_change_map_result(
    conn: &mut TestConnection,
    map_wz: i32,
    portal_wz: u8,
) -> Result<(), HarnessError> {
    loop {
        let packet = conn.read_packet(PHASE).await?;
        let mut cursor = Cursor::new(&packet.bytes[..]);
        let op = cursor
            .read_short()
            .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
        match op {
            x if x == SendOpcode::SetField as i16 => {
                let result = read_set_field_packet(&packet)?;
                assert_eq!(result.map_wz, map_wz);
                assert_eq!(result.portal_wz, portal_wz);
                conn.send_packet(test_player_logged_in::build_player_map_transfer()?, PHASE)
                    .await?;
                break;
            }
            _ => {}
        }
    }
    Ok(())
}

fn build_change_map(map_wz: i32) -> Result<Packet, HarnessError> {
    let mut packet = Packet::new_empty();
    packet
        .write_short(RecvOpcode::ChangeMap as i16)
        .map_err(|e| HarnessError::PacketIOError(WriteError(e)))?;
    let died: bool = false;
    packet
        .write_byte(died as i16)
        .map_err(|e| HarnessError::PacketIOError(WriteError(e)))?;
    packet
        .write_int(map_wz)
        .map_err(|e| HarnessError::PacketIOError(WriteError(e)))?;
    packet
        .write_str_with_length(String::from("out00"))
        .map_err(|e| HarnessError::PacketIOError(WriteError(e)))?;
    let skip: Vec<u8> = vec![0; 1];
    packet
        .write_bytes(skip)
        .map_err(|e| HarnessError::PacketIOError(WriteError(e)))?;
    let use_wheel: bool = false;
    packet
        .write_short(use_wheel as i16)
        .map_err(|e| HarnessError::PacketIOError(WriteError(e)))?;
    Ok(packet)
}

pub fn read_set_field_packet(packet: &Packet) -> Result<SetFieldResult, HarnessError> {
    let mut cursor = Cursor::new(&packet.bytes[..]);
    cursor
        .read_short()
        .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
    let channel_id = cursor
        .read_int()
        .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
    let mode_one = cursor
        .read_byte()
        .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
    let mode_two = cursor
        .read_byte()
        .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
    let skip: usize = 3;
    cursor
        .read_bytes(skip)
        .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
    let map_wz = cursor
        .read_int()
        .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
    let portal_wz = cursor
        .read_byte()
        .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
    Ok(SetFieldResult {
        channel_id,
        mode_one,
        mode_two,
        map_wz,
        portal_wz,
    })
}
