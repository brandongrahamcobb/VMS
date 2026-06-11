use crate::error::HarnessError;
use crate::net::connection::TestConnection;
use net::packet::io::error::IOError::{ReadError, WriteError};
use net::packet::io::prelude::*;
use net::packet::model::Packet;
use op::recv::RecvOpcode;
use std::io::Cursor;

pub const PHASE: &str = "character list request";
pub const WORLD_ID: i16 = 0;
pub const CHANNEL_ID: u8 = 1;
pub const MAP_WZ: i32 = 10000;
pub const IGN: &str = "Test";

#[derive(Clone)]
pub struct CharacterResult {
    pub id: i32,
    pub name: String,
}

pub struct CharacterMeta {
    pub id: i32,
    pub name: String,
    pub map_wz: i32,
}

pub struct CharListResult {
    pub characters: Vec<CharacterResult>,
}

pub async fn assert_char_list_request(
    mut conn: TestConnection,
) -> Result<TestConnection, HarnessError> {
    dbg!(PHASE);
    conn.send_packet(build_char_list_request(WORLD_ID, CHANNEL_ID as i16)?, PHASE)
        .await?;
    assert_char_list_result(&mut conn).await?;
    Ok(conn)
}

pub fn build_char_list_request(world_id: i16, channel_id: i16) -> Result<Packet, HarnessError> {
    let mut packet = Packet::new_empty();
    packet
        .write_short(RecvOpcode::CharListRequest as i16)
        .map_err(|e| HarnessError::PacketIOError(WriteError(e)))?;
    packet
        .write_byte(0)
        .map_err(|e| HarnessError::PacketIOError(WriteError(e)))?;
    packet
        .write_byte(world_id)
        .map_err(|e| HarnessError::PacketIOError(WriteError(e)))?;
    packet
        .write_byte(channel_id)
        .map_err(|e| HarnessError::PacketIOError(WriteError(e)))?;
    Ok(packet)
}

async fn assert_char_list_result(conn: &mut TestConnection) -> Result<(), HarnessError> {
    let packet = conn.read_packet(PHASE).await?;
    let result = read_char_list_packet(&packet)?;
    assert_eq!(result.characters.len(), 0);
    Ok(())
}

pub fn read_char_list_packet(packet: &Packet) -> Result<CharListResult, HarnessError> {
    let mut cursor = Cursor::new(&packet.bytes[..]);
    cursor
        .read_short()
        .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
    cursor
        .read_byte()
        .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
    let count = cursor
        .read_byte()
        .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
    let mut characters = Vec::with_capacity(count as usize);
    for _ in 0..count {
        let meta = read_char_meta(&mut cursor)?;
        let skip: usize = 31;
        cursor
            .read_bytes(skip)
            .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
        characters.push(CharacterResult {
            id: meta.id,
            name: meta.name,
        });
    }

    Ok(CharListResult { characters })
}

pub fn read_char_meta(cursor: &mut Cursor<&[u8]>) -> Result<CharacterMeta, HarnessError> {
    let id = cursor
        .read_int()
        .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
    let name_bytes = cursor
        .read_bytes(13)
        .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
    let end = name_bytes
        .iter()
        .position(|byte| *byte == 0)
        .unwrap_or(name_bytes.len());
    let name = String::from_utf8(name_bytes[..end].to_vec())?;
    let skip: usize = 37;
    cursor
        .read_bytes(skip)
        .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
    let skip: usize = 18;
    cursor
        .read_bytes(skip)
        .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
    let skip: usize = 2;
    cursor
        .read_bytes(skip)
        .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
    let skip: usize = 10;
    cursor
        .read_bytes(skip)
        .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
    let map_wz = cursor
        .read_int()
        .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
    let skip: usize = 5;
    cursor
        .read_bytes(skip)
        .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
    Ok(CharacterMeta { id, name, map_wz })
}
