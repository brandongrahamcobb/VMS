use crate::db::error::DatabaseError;
use crate::models::character::model::Character;
use crate::models::error::ModelError;
use crate::models::keybinding::model::Keybinding;
use crate::models::{character, keybinding};
use crate::net::error::NetworkError;
use crate::net::packet::core::Packet;
use crate::net::packet::error::PacketError;
use crate::net::packet::handler::action::ChannelAction;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::io::error::IOError::{ReadError, WriteError};
use crate::op::send::SendOpcode;
use crate::prelude::*;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;
use std::io::Cursor;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct PlayerLoggedInHandler;

impl PlayerLoggedInHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        self: &Self,
        state: SharedState,
        session: Session,
        packet: Packet,
    ) -> Result<HandlerResult<ChannelAction>, NetworkError> {
        let mut reader = Cursor::new(packet.bytes);
        reader
            .read_short() // prune opcode
            .map_err(ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        let char_id = reader
            .read_int()
            .map_err(ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        let acc_id = character::query::get_account_id_by_character_id(state.clone(), char_id)
            .await
            .map_err(DatabaseError::from)
            .map_err(NetworkError::from)?;
        {
            let state = state.lock().await;
            state.sessions.update(session.id as u32, |session| {
                session.acc_id = Some(acc_id);
            });
        }
        let channel_id = reader
            .read_byte()
            .map_err(ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;

        let char = character::query::get_character_by_id(state.clone(), char_id)
            .await
            .map_err(DatabaseError::from)
            .map_err(NetworkError::from)?;
        let binds = keybinding::query::get_keybindings_by_character_id(state.clone(), char_id)
            .await
            .map_err(DatabaseError::from)
            .map_err(NetworkError::from)?;
        let binds = normalize_keybindings(binds, char.id);
        let mut result = HandlerResult::new();
        let mut packets = Vec::new();
        packets.push(build_keymap(&binds)?);
        packets.push(build_char_info(&char, channel_id as i16)?);
        for packet in packets {
            let action = ChannelAction::SendPacket { packet };
            result.add_action(action)?;
        }
        Ok(result)
    }
}

pub fn build_keymap(binds: &Vec<Keybinding>) -> Result<Packet, NetworkError> {
    let mut packet = Packet::new_empty();
    let op = SendOpcode::KeyMap as i16;
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
    for bind in binds {
        packet
            .write_byte(bind.bind_type as u8)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        packet
            .write_int(bind.action as i32)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
    }
    Ok(packet)
}

pub fn normalize_keybindings(bindings: Vec<Keybinding>, char_id: i32) -> Vec<Keybinding> {
    let mut result: Vec<Keybinding> = Vec::with_capacity(90);
    for i in 0..90 {
        result.push(Keybinding::empty(char_id, i as i16));
    }
    for bind in bindings {
        let idx = bind.key as usize;
        if idx < 90 {
            result[idx] = bind;
        }
    }
    result
}

pub fn build_char_info(char: &Character, channel_id: i16) -> Result<Packet, NetworkError> {
    let mut packet = Packet::new_empty();
    let op = SendOpcode::SetField as i16;
    packet
        .write_short(op)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_int(channel_id as i32)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_byte(1)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_byte(1)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_short(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    // These are random... No idea what they are though.
    packet
        .write_int(1)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_int(2)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_int(3)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_long(-1)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_byte(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    character::service::write_char_meta(&mut packet, &char)
        .map_err(ModelError::from)
        .map_err(NetworkError::from)?;
    character::service::write_game_char(&mut packet, &char)
        .map_err(ModelError::from)
        .map_err(NetworkError::from)?;
    let time = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis() as i64;
    packet
        .write_long(time)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    Ok(packet)
}
