use crate::error::HarnessError;
use crate::net::connection::TestConnection;
use crate::tests::test_credentials::GENDER_WZ;
use entity::character::model::CharacterModel;
use op::recv::RecvOpcode;
use packet::io::error::IOError::{ReadError, WriteError};
use packet::model::Packet;
use packet::prelude::*;
use state::model::SharedState;
use std::io::Cursor;
use std::time::SystemTime;

pub const PHASE: &str = "character list request";
pub const WORLD_ID: i16 = 0;
pub const CHANNEL_ID: u8 = 1;
pub const MAP_WZ: i32 = 0;
pub const IGN: &str = "Test";
const STR: i16 = 0;
const INT: i16 = 0;
const LUK: i16 = 0;
const DEX: i16 = 0;
const LEVEL: i16 = 1;
const EXP: i32 = 0;
const HP: i16 = 50;
const MAX_HP: i16 = 50;
const MP: i16 = 5;
const MAX_MP: i16 = 5;
const AP: i16 = 0;
const SP: i16 = 0;
const FAME: i16 = 0;
const MESO: i32 = 0;
const HAIR_COLOR_WZ: i32 = 0;
const SKIN_WZ: i32 = 0;
const FACE_WZ: i32 = 20000;
const HAIR_WZ: i32 = 30000;
const LAST_PORTAL_ID: i16 = 0;
const JOB_WZ: i16 = 0;
pub const CHAR_ID: i32 = 1;

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
    state: &SharedState,
    mut conn: TestConnection,
    acc_id: i32,
) -> Result<TestConnection, HarnessError> {
    dbg!(PHASE);
    {
        let char_model: CharacterModel = CharacterModel {
            id: None,
            acc_id,
            gender_wz: GENDER_WZ,
            world_id: WORLD_ID,
            map_wz: MAP_WZ,
            ign: IGN.to_string(),
            strength: STR,
            dexterity: DEX,
            intelligence: INT,
            luck: LUK,
            exp: EXP,
            level: LEVEL,
            hp: HP,
            max_hp: MAX_HP,
            mp: MP,
            max_mp: MAX_MP,
            ap: AP,
            sp: SP,
            fame: FAME,
            meso: MESO,
            hair_color_wz: HAIR_COLOR_WZ,
            hair_wz: HAIR_WZ,
            skin_wz: SKIN_WZ,
            face_wz: FACE_WZ,
            last_portal: LAST_PORTAL_ID,
            job_wz: JOB_WZ,
            created_at: Some(SystemTime::now()),
            updated_at: SystemTime::now(),
        };
        db::character::setters::update_characters(&state.lock().await.db.clone(), vec![char_model])
            .await?;
    }
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
    if let Some(character) = result
        .characters
        .iter()
        .find(|character| character.name == IGN)
        .cloned()
    {
        assert_eq!(character.id, CHAR_ID);
    }
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
