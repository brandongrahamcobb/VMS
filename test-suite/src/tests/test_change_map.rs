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
pub const THIRD_MAP_WZ: i32 = 40000;
pub const FIRST_PORTAL_WZ: u8 = 3;
pub const SECOND_PORTAL_WZ: u8 = 2;
pub const THIRD_PORTAL_WZ: u8 = 5;
pub const PHASE: &str = "change map";

pub struct SetFieldResult {
    channel_id: i32,
    mode_one: u8,
    mode_two: u8,
    map_wz: i32,
    portal_wz: u8,
}

pub struct SpawnMobResult {
    mob_id: u32,
    mob_wz: i32,
    x: i16,
    y: i16,
    stance: u8,
    fh: i16,
    effect: u8,
    team: u8,
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

pub async fn assert_third_change_map(
    mut conn: TestConnection,
) -> Result<(TestConnection, u32), HarnessError> {
    dbg!(PHASE);
    conn.send_packet(build_change_map(THIRD_MAP_WZ)?, PHASE)
        .await?;
    assert_change_map_result(&mut conn, THIRD_MAP_WZ, THIRD_PORTAL_WZ).await?;
    let mob_id = assert_spawn_mob_result(&mut conn).await?;
    Ok((conn, mob_id))
}

async fn assert_spawn_mob_result(conn: &mut TestConnection) -> Result<u32, HarnessError> {
    let mob_id = loop {
        let packet = conn.read_packet(PHASE).await?;
        let mut cursor = Cursor::new(&packet.bytes[..]);
        let op = cursor
            .read_short()
            .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
        match op {
            x if x == SendOpcode::SpawnMob as i16 => {
                let result = read_spawn_mob_packet(&packet)?;
                break result.mob_id;
            }
            _ => {}
        }
    };
    Ok(mob_id)
}

fn read_spawn_mob_packet(packet: &Packet) -> Result<SpawnMobResult, HarnessError> {
    let mut cursor = Cursor::new(&packet.bytes[..]);
    let _op = cursor
        .read_short()
        .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
    let mob_id = cursor
        .read_int()
        .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
    let skip: usize = 1;
    cursor
        .read_bytes(skip)
        .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
    let mob_wz = cursor
        .read_int()
        .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
    let skip: usize = 22;
    cursor
        .read_bytes(skip)
        .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
    let mob_base_x = cursor
        .read_short()
        .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
    let mob_base_y = cursor
        .read_short()
        .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
    let stance = cursor
        .read_byte()
        .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
    let skip: usize = 2;
    cursor
        .read_bytes(skip)
        .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
    let fh = cursor
        .read_short()
        .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
    let effect = cursor
        .read_byte()
        .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
    let team = cursor
        .read_byte()
        .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
    let skip: usize = 4;
    cursor
        .read_bytes(skip)
        .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
    Ok(SpawnMobResult {
        mob_id: mob_id as u32,
        mob_wz,
        x: mob_base_x,
        y: mob_base_y,
        stance,
        fh,
        effect,
        team,
    })
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
