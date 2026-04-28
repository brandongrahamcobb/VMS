use crate::config::settings;
use crate::db::error::DatabaseError;
use crate::db::models::character::core::Character;
use crate::db::models::{character, world};
use crate::net::error::NetworkError;
use crate::net::packet::core::Packet;
use crate::net::packet::error::PacketError;
use crate::net::packet::handler::action::Action;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::io::error::IOError::{ReadError, WriteError};
use crate::op::send::SendOpcode;
use crate::prelude::*;
use crate::runtime::error::SessionError;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;
use std::io::Cursor;

pub struct CharListHandler;

impl CharListHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        self: &Self,
        state: SharedState,
        session: Session,
        packet: Packet,
    ) -> Result<HandlerResult<Action>, NetworkError> {
        let mut reader = Cursor::new(packet.bytes);
        reader
            .read_short()
            .map_err(ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        reader
            .read_byte()
            .map_err(ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        let world_id = reader
            .read_byte()
            .map_err(ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        let channel_id = reader
            .read_byte()
            .map_err(ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        let acc_id = session
            .acc_id
            .ok_or(SessionError::NoAccount)
            .map_err(NetworkError::from)?;
        let chars = character::service::get_characters_by_account_id(state.clone(), acc_id)
            .await
            .map_err(DatabaseError::from)
            .map_err(NetworkError::from)?;
        let char_max = world::service::get_character_max_by_account_and_world_id(
            state.clone(),
            acc_id,
            world_id as i16,
        )
        .await
        .map_err(DatabaseError::from)
        .map_err(NetworkError::from)
        .unwrap_or(8);
        let use_pic = settings::get_pic_required()?;
        let pic_status = if use_pic { 2 } else { 0 };
        let mut result = HandlerResult::new();
        let packet = build_char_list(chars, char_max, pic_status)?;
        let action = Action::SendPacket { packet };
        result.add_action(action)?;
        Ok(result)
    }
}

pub fn build_char_list(
    chars: Vec<Character>,
    char_max: i32,
    pic_status: i8,
) -> Result<Packet, NetworkError> {
    let mut packet = Packet::new_empty();
    let op = SendOpcode::CharList as i16;
    packet
        .write_short(op)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_byte(0) // account status
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_byte(chars.len() as u8) // number of chars
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    for character in chars {
        write_char(&mut packet, &character)?;
    }
    packet
        .write_byte(pic_status as u8) // use pic?
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_int(char_max) // Number of character slots
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    Ok(packet)
}

pub fn write_char(packet: &mut Packet, character: &Character) -> Result<(), NetworkError> {
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
