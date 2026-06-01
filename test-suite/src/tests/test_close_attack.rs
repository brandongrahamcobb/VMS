use crate::error::HarnessError;
use crate::net::connection::TestConnection;
use op::recv::RecvOpcode;
use op::send::SendOpcode;
use net::packet::io::error::IOError::{ReadError, WriteError};
use net::packet::model::Packet;
use net::packet::io::prelude::*;
use std::collections::HashMap;
use std::io::Cursor;

pub const SEND_PHASE: &str = "send close attack";
pub const RECEIVE_PHASE: &str = "receive close attack";
struct CloseAttackResult {
    char_id: i32,
    count: i16,
    skill_id: i32,
    skill_level: i16,
    display: i16,
    toleft: i16,
    stance: i16,
    speed: i16,
    mob_damages: HashMap<u32, Vec<i32>>,
}

pub async fn assert_close_attack(
    mut conn: TestConnection,
    char_id: i32,
) -> Result<TestConnection, HarnessError> {
    dbg!(RECEIVE_PHASE);
    assert_close_attack_result(&mut conn, char_id).await?;
    Ok(conn)
}

fn read_close_attack_packet(packet: &Packet) -> Result<CloseAttackResult, HarnessError> {
    let mut cursor = Cursor::new(&packet.bytes[..]);
    let _op = cursor
        .read_short()
        .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
    let char_id = cursor
        .read_int()
        .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
    let count = cursor
        .read_byte()
        .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
    let skip: usize = 1;
    cursor
        .read_bytes(skip)
        .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
    let skill_level = cursor
        .read_byte()
        .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
    let skill_id = cursor
        .read_int()
        .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
    let display = cursor
        .read_byte()
        .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
    let toleft = cursor
        .read_byte()
        .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
    let stance = cursor
        .read_byte()
        .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
    let speed = cursor
        .read_byte()
        .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
    let skip: usize = 1;
    cursor
        .read_bytes(skip)
        .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
    let mut mob_damages: HashMap<u32, Vec<i32>> = HashMap::new();
    for _ in 0..count {
        let mob_id = cursor
            .read_int()
            .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
        let skip: usize = 1;
        cursor
            .read_bytes(skip)
            .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
        if skill_id == 4211006 {
            let _max_hits = cursor
                .read_byte()
                .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
        }
        let mut damages: Vec<i32> = Vec::new();
        while let Ok(damage) = cursor.read_int() {
            damages.push(damage);
        }
        mob_damages.insert(mob_id as u32, damages);
    }
    Ok(CloseAttackResult {
        char_id,
        count: count as i16,
        skill_id,
        skill_level: skill_level as i16,
        display: display as i16,
        toleft: toleft as i16,
        stance: stance as i16,
        speed: speed as i16,
        mob_damages,
    })
}

async fn assert_close_attack_result(
    conn: &mut TestConnection,
    char_id: i32,
) -> Result<(), HarnessError> {
    loop {
        let packet = conn.read_packet(RECEIVE_PHASE).await?;
        let mut cursor = Cursor::new(&packet.bytes[..]);
        let op = cursor
            .read_short()
            .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
        if op == SendOpcode::AttackedClose as i16 {
            let result: CloseAttackResult = read_close_attack_packet(&packet)?;
            assert_eq!(result.char_id, char_id);
            break;
        }
    }
    Ok(())
}

pub async fn send_close_attack(mut conn: TestConnection) -> Result<TestConnection, HarnessError> {
    dbg!(SEND_PHASE);
    conn.send_packet(build_close_attack()?, SEND_PHASE).await?;
    Ok(conn)
}

fn build_close_attack() -> Result<Packet, HarnessError> {
    let mut packet = Packet::new_empty();
    packet
        .write_short(RecvOpcode::CloseAttack as i16)
        .map_err(|e| HarnessError::PacketIOError(WriteError(e)))?;
    let skip: Vec<u8> = vec![0; 1];
    packet
        .write_bytes(skip)
        .map_err(|e| HarnessError::PacketIOError(WriteError(e)))?;
    let counts: i16 = 0;
    let hit_count = counts & 0x0F;
    let mob_count = (counts >> 4) & 0x0F;
    packet
        .write_byte(counts)
        .map_err(|e| HarnessError::PacketIOError(WriteError(e)))?;
    let skill_id: i32 = 0;
    packet
        .write_int(skill_id)
        .map_err(|e| HarnessError::PacketIOError(WriteError(e)))?;
    let skip: Vec<u8> = vec![0; 8];
    packet
        .write_bytes(skip)
        .map_err(|e| HarnessError::PacketIOError(WriteError(e)))?;
    let display: i16 = 0;
    packet
        .write_byte(display)
        .map_err(|e| HarnessError::PacketIOError(WriteError(e)))?;
    let to_left: i16 = 0;
    packet
        .write_byte(to_left)
        .map_err(|e| HarnessError::PacketIOError(WriteError(e)))?;
    let stance: i16 = 0;
    packet
        .write_byte(stance)
        .map_err(|e| HarnessError::PacketIOError(WriteError(e)))?;
    let skip: Vec<u8> = vec![0; 1];
    packet
        .write_bytes(skip)
        .map_err(|e| HarnessError::PacketIOError(WriteError(e)))?;
    let speed: i16 = 4;
    packet
        .write_byte(speed)
        .map_err(|e| HarnessError::PacketIOError(WriteError(e)))?;
    let skip: Vec<u8> = vec![0; 4];
    packet
        .write_bytes(skip)
        .map_err(|e| HarnessError::PacketIOError(WriteError(e)))?;
    for _ in 0..mob_count {
        let mob_id: i32 = 100100;
        packet
            .write_int(mob_id)
            .map_err(|e| HarnessError::PacketIOError(WriteError(e)))?;
        let skip: Vec<u8> = vec![0; 14];
        packet
            .write_bytes(skip)
            .map_err(|e| HarnessError::PacketIOError(WriteError(e)))?;
        for _ in 0..hit_count {
            let damage: i32 = 100;
            packet
                .write_int(damage)
                .map_err(|e| HarnessError::PacketIOError(WriteError(e)))?;
        }
        if skill_id != 5221004 {
            let skip: Vec<u8> = vec![0; 4];
            packet
                .write_bytes(skip)
                .map_err(|e| HarnessError::PacketIOError(WriteError(e)))?;
        }
    }
    Ok(packet)
}
