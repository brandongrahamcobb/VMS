use itertools::izip;

use crate::constants::{DEFAULT_ACTION, DEFAULT_KEY, DEFAULT_TYPE};
use crate::db::error::DatabaseError;
use crate::db::models::character::core::{Character, NewCharacter};
use crate::db::models::keybinding::core::NewKeybinding;
use crate::db::models::{character, keybinding};
use crate::net::error::NetworkError;
use crate::net::packet::core::Packet;
use crate::net::packet::error::PacketError;
use crate::net::packet::handler::action::Action;
use crate::net::packet::handler::error::HandlerError;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::io::error::IOError::{ReadError, WriteError};
use crate::op::send::SendOpcode;
use crate::prelude::*;
use crate::runtime::state::SharedState;
use std::io::Cursor;
pub struct CreateCharacterHandler;

impl CreateCharacterHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        self: &Self,
        state: SharedState,
        packet: Packet,
    ) -> Result<HandlerResult<Action>, NetworkError> {
        let mut reader = Cursor::new(packet.bytes);
        use tracing::debug;
        let value = reader
            .read_short()
            .map_err(ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        debug!("CreateCharacter: {}", value);
        let ign = reader
            .read_str_with_length()
            .map_err(ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        let job = reader
            .read_int()
            .map_err(ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)? as i16;
        let map = get_map_id_for_job(job)?;
        let face = reader
            .read_int()
            .map_err(ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        let hair = reader
            .read_int()
            .map_err(ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        let hair_color = reader
            .read_int()
            .map_err(ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        let skin = reader
            .read_int()
            .map_err(ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        let _top = reader
            .read_int() // Slot 5
            .map_err(ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        let _bot = reader
            .read_int() // Slot 6
            .map_err(ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        let _shoes = reader
            .read_int() // Slot 7
            .map_err(ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        let _weapon = reader
            .read_int() // Special
            .map_err(ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        let gender = reader
            .read_byte()
            .map_err(ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)? as i16;
        let map = get_map_id_for_job(job)?;
        let char = NewCharacter {
            account: 0, //placeholder
            ign: ign.clone(),
            world: 0, //placeholder
            level: None,
            exp: None,
            strength: None,
            dexterity: None,
            luck: None,
            intelligence: None,
            hp: None,
            mp: None,
            max_hp: None,
            max_mp: None,
            ap: None,
            fame: None,
            meso: None,
            job,
            face,
            hair,
            hair_color,
            skin,
            gender,
            created_at: None,
            map: Some(map),
            updated_at: None,
        };
        let character = character::service::create_character(state.clone(), &char)
            .await
            .map_err(DatabaseError::from)
            .map_err(NetworkError::from)?;
        let binds: Vec<NewKeybinding> = izip!(DEFAULT_KEY, DEFAULT_TYPE, DEFAULT_ACTION)
            .map(|(key, bind_type, action): (i16, u8, i16)| NewKeybinding {
                character_id: character.id,
                key,
                bind_type: bind_type.into(),
                action,
            })
            .collect();
        keybinding::service::update_keybindings(state.clone(), binds)
            .await
            .map_err(DatabaseError::from)
            .map_err(NetworkError::from)?;
        let mut result = HandlerResult::new();
        let packet = build_create_char_packet(character)?;
        let action = Action::SendPacket { packet };
        result.add_action(action)?;
        Ok(result)
    }
}

fn get_map_id_for_job(job: i16) -> Result<i32, NetworkError> {
    match job {
        1 => Ok(0),
        1000 => Ok(130000000),
        2000 => Ok(140000000),
        _ => Err(NetworkError::from(PacketError::from(
            HandlerError::LoginError,
        ))),
    }
}

fn build_create_char_packet(character: Character) -> Result<Packet, NetworkError> {
    let mut packet = Packet::new_empty();
    let op = SendOpcode::NewCharacter as i16;
    packet
        .write_short(op)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_byte(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    write_char(&mut packet, &character)?;
    Ok(packet)
}

fn write_char(packet: &mut Packet, character: &Character) -> Result<(), NetworkError> {
    write_char_meta(packet, &character)?;
    write_char_look(packet, &character)?;
    packet
        .write_byte(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    // Disable rank.
    packet
        .write_byte(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    Ok(())
}

fn write_char_meta(packet: &mut Packet, character: &Character) -> Result<(), NetworkError> {
    packet
        .write_int(character.id)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_str(&character.ign)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_bytes(&vec![0u8; 13 - character.ign.len()])
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_byte(character.gender as u8)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_byte(character.skin as u8)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_int(character.face)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_int(character.hair)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    // Pets... Not implemented yet
    packet
        .write_long(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_long(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_long(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_byte(character.level.ok_or(NetworkError::UnexpectedError)? as u8)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_short(character.job)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_short(character.strength.ok_or(NetworkError::UnexpectedError)?)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_short(character.dexterity.ok_or(NetworkError::UnexpectedError)?)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_short(
            character
                .intelligence
                .ok_or(NetworkError::UnexpectedError)?,
        )
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_short(character.luck.ok_or(NetworkError::UnexpectedError)?)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_short(character.hp.ok_or(NetworkError::UnexpectedError)?)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_short(character.max_hp.ok_or(NetworkError::UnexpectedError)?)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_short(character.mp.ok_or(NetworkError::UnexpectedError)?)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_short(character.max_mp.ok_or(NetworkError::UnexpectedError)?)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_short(character.ap.ok_or(NetworkError::UnexpectedError)?)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    // SP
    packet
        .write_short(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_int(character.exp.ok_or(NetworkError::UnexpectedError)?)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_short(character.fame.ok_or(NetworkError::UnexpectedError)?)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    // Gach xp?
    packet
        .write_int(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_int(character.map.ok_or(NetworkError::UnexpectedError)?)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_byte(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_int(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    Ok(())
}

fn write_char_look(packet: &mut Packet, character: &Character) -> Result<(), NetworkError> {
    packet
        .write_byte(character.gender as u8)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_byte(character.skin as u8)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_int(character.face)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_byte(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_int(character.hair)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    write_char_equips(packet, character)?;
    Ok(())
}
fn write_char_equips(packet: &mut Packet, _character: &Character) -> Result<(), NetworkError> {
    // Overall (Top slot)
    packet
        .write_byte(5)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_int(1052122)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    // Shoes
    packet
        .write_byte(7)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_int(1072318)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_byte(0xFF)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    // Cash shop equips
    packet
        .write_byte(0xFF)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    // Weapon
    packet
        .write_int(1302000)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    // Pet stuff...
    packet
        .write_int(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_int(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_int(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    Ok(())
}
