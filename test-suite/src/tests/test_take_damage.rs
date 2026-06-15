use crate::error::HarnessError;
use crate::net::connection::TestConnection;
use crate::tests::{test_change_channel, test_change_map, test_player_logged_in};
use net::packet::io::error::IOError::{ReadError, WriteError};
use net::packet::io::prelude::*;
use net::packet::model::Packet;
use op::recv::RecvOpcode;
use op::send::SendOpcode;
use std::io::Cursor;

pub const PHASE: &str = "take damage";
pub const MOB_WZ: i32 = 5200000;
pub const DEAD_HP: i16 = 50 - FULL_DAMAGE;
pub const HALF_HP: i16 = 50 - HALF_DAMAGE;
pub const HALF_DAMAGE: i16 = 50 / 2;
pub const FULL_DAMAGE: i16 = 50;
pub const RETURN_MAP_WZ: i32 = 40000;

pub async fn assert_take_damage(
    mut conn: TestConnection,
    char_id: i32,
    mob_id: u32,
) -> Result<TestConnection, HarnessError> {
    dbg!(PHASE);
    conn.send_packet(
        build_take_damage_packet(HALF_DAMAGE, mob_id, MOB_WZ)?,
        PHASE,
    )
    .await?;
    assert_take_damage_result(&mut conn).await?;
    conn.send_packet(
        build_take_damage_packet(FULL_DAMAGE, mob_id, MOB_WZ)?,
        PHASE,
    )
    .await?;
    assert_death_result(&mut conn, char_id).await?;
    Ok(conn)
}

pub fn build_take_damage_packet(
    damage: i16,
    mob_id: u32,
    mob_wz: i32,
) -> Result<Packet, HarnessError> {
    let mut packet = Packet::new_empty();
    packet
        .write_short(RecvOpcode::TakeDamage as i16)
        .map_err(|e| HarnessError::PacketIOError(WriteError(e)))?;
    let tick_time: i32 = 0;
    packet
        .write_int(tick_time)
        .map_err(|e| HarnessError::PacketIOError(WriteError(e)))?;
    let from: i16 = 0;
    packet
        .write_byte(from)
        .map_err(|e| HarnessError::PacketIOError(WriteError(e)))?;
    let element: i16 = 0;
    packet
        .write_byte(element)
        .map_err(|e| HarnessError::PacketIOError(WriteError(e)))?;
    packet
        .write_int(damage as i32)
        .map_err(|e| HarnessError::PacketIOError(WriteError(e)))?;
    packet
        .write_int(mob_id as i32)
        .map_err(|e| HarnessError::PacketIOError(WriteError(e)))?;
    packet
        .write_int(mob_wz)
        .map_err(|e| HarnessError::PacketIOError(WriteError(e)))?;
    let direction: i16 = 0;
    packet
        .write_byte(direction)
        .map_err(|e| HarnessError::PacketIOError(WriteError(e)))?;
    Ok(packet)
}

async fn assert_take_damage_result(conn: &mut TestConnection) -> Result<(), HarnessError> {
    loop {
        let packet = conn.read_packet(PHASE).await?;
        let mut cursor = Cursor::new(&packet.bytes[..]);
        let op = cursor
            .read_short()
            .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
        if op == SendOpcode::ChangeStats as i16 {
            let hp = read_take_damage_packet(&packet)?;
            assert_eq!(hp, HALF_HP);
            break;
        }
    }
    Ok(())
}

async fn assert_death_result(conn: &mut TestConnection, char_id: i32) -> Result<(), HarnessError> {
    let mut despawn_packet_seen: bool = false;
    let mut set_field_packet_seen: bool = false;
    loop {
        let packet = conn.read_packet(PHASE).await?;
        let mut cursor = Cursor::new(&packet.bytes[..]);
        let op = cursor
            .read_short()
            .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
        if op == SendOpcode::DespawnPlayer as i16 {
            let result_char_id = test_change_channel::read_despawn_player_packet(&packet)?;
            assert_eq!(result_char_id, char_id);
            despawn_packet_seen = true;
        } else if op == SendOpcode::SetField as i16 {
            let result = test_change_map::read_set_field_packet(&packet)?;
            assert_eq!(result.map_wz, RETURN_MAP_WZ);
            conn.send_packet(test_player_logged_in::build_player_map_transfer()?, PHASE)
                .await?;
            set_field_packet_seen = true;
        }
        if despawn_packet_seen && set_field_packet_seen {
            break;
        }
    }
    Ok(())
}

pub fn read_take_damage_packet(packet: &Packet) -> Result<i16, HarnessError> {
    let mut cursor = Cursor::new(&packet.bytes[..]);
    let _op = cursor
        .read_short()
        .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
    let _item_reaction = cursor
        .read_byte()
        .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
    let _update_mask = cursor
        .read_int()
        .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
    let hp = cursor
        .read_short()
        .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
    Ok(hp)
}
