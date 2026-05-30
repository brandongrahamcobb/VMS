use crate::error::HarnessError;
use crate::net::connection::TestConnection;
use crate::tests::test_char_list::{self, CharacterResult};
use crate::tests::test_credentials::GENDER_WZ;
use op::recv::RecvOpcode;
use net::packet::io::error::IOError::{ReadError, WriteError};
use net::packet::model::Packet;
use net::packet::prelude::*;
use std::io::Cursor;

pub const PHASE: &str = "character creation";
const HAIR_COLOR_WZ: i32 = 0;
const SKIN_WZ: i32 = 0;
const FACE_WZ: i32 = 20000;
const HAIR_WZ: i32 = 30000;
const TOP_WZ: i32 = 1040002;
const BOTTOM_WZ: i32 = 1060002;
const SHOES_WZ: i32 = 1072001;
const WEAPON_WZ: i32 = 1302000;
const JOB_WZ: i16 = 0;
const SUCCESS_STATUS: i32 = 0;

pub struct NewCharacterResult {
    pub status: u8,
    pub character: CharacterResult,
}

pub async fn assert_create_char(
    mut conn: TestConnection,
    char_ign: &str,
) -> Result<(i32, TestConnection), HarnessError> {
    dbg!(PHASE);
    conn.send_packet(build_create_char(char_ign.to_string())?, PHASE)
        .await?;
    let char_id: i32 = assert_create_char_result(&mut conn, char_ign).await?;
    Ok((char_id, conn))
}

pub fn build_create_char(name: String) -> Result<Packet, HarnessError> {
    let mut packet: Packet = Packet::new_empty();
    packet
        .write_short(RecvOpcode::CreateChar as i16)
        .map_err(|e| HarnessError::PacketIOError(WriteError(e)))?;
    packet
        .write_str_with_length(name)
        .map_err(|e| HarnessError::PacketIOError(WriteError(e)))?;
    packet
        .write_int(JOB_WZ as i32)
        .map_err(|e| HarnessError::PacketIOError(WriteError(e)))?;
    packet
        .write_int(FACE_WZ)
        .map_err(|e| HarnessError::PacketIOError(WriteError(e)))?;
    packet
        .write_int(HAIR_WZ)
        .map_err(|e| HarnessError::PacketIOError(WriteError(e)))?;
    packet
        .write_int(HAIR_COLOR_WZ)
        .map_err(|e| HarnessError::PacketIOError(WriteError(e)))?;
    packet
        .write_int(SKIN_WZ)
        .map_err(|e| HarnessError::PacketIOError(WriteError(e)))?;
    packet
        .write_int(TOP_WZ)
        .map_err(|e| HarnessError::PacketIOError(WriteError(e)))?;
    packet
        .write_int(BOTTOM_WZ)
        .map_err(|e| HarnessError::PacketIOError(WriteError(e)))?;
    packet
        .write_int(SHOES_WZ)
        .map_err(|e| HarnessError::PacketIOError(WriteError(e)))?;
    packet
        .write_int(WEAPON_WZ)
        .map_err(|e| HarnessError::PacketIOError(WriteError(e)))?;
    packet
        .write_byte(GENDER_WZ)
        .map_err(|e| HarnessError::PacketIOError(WriteError(e)))?;
    Ok(packet)
}

async fn assert_create_char_result(
    conn: &mut TestConnection,
    char_ign: &str,
) -> Result<i32, HarnessError> {
    let packet = conn.read_packet(PHASE).await?;
    let result = read_create_char_packet(&packet)?;
    assert_eq!(result.status as i32, SUCCESS_STATUS);
    assert_eq!(result.character.name, char_ign);
    Ok(result.character.id)
}

pub fn read_create_char_packet(packet: &Packet) -> Result<NewCharacterResult, HarnessError> {
    let mut cursor = Cursor::new(&packet.bytes[..]);
    cursor
        .read_short()
        .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
    let status = cursor
        .read_byte()
        .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
    let meta = test_char_list::read_char_meta(&mut cursor)?;
    let skip: usize = 31;
    cursor
        .read_bytes(skip)
        .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
    Ok(NewCharacterResult {
        status,
        character: CharacterResult {
            id: meta.id,
            name: meta.name,
        },
    })
}
