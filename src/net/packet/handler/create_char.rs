use crate::constants::{DEFAULT_ACTION, DEFAULT_KEY, DEFAULT_TYPE};
use crate::db::error::DatabaseError;
use crate::models::character::model::{
    CashEquipment, Character, CharacterEquipment, NewCashEquipment, NewCharacter,
    NewCharacterEquipment,
};
use crate::models::error::ModelError;
use crate::models::keybinding::model::NewKeybinding;
use crate::models::world::error::WorldError;
use crate::models::{account, character, keybinding};
use crate::net::error::NetworkError;
use crate::net::packet::core::Packet;
use crate::net::packet::error::PacketError;
use crate::net::packet::handler::action::LoginAction;
use crate::net::packet::handler::error::HandlerError;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::io::error::IOError::{ReadError, WriteError};
use crate::op::send::SendOpcode;
use crate::prelude::*;
use crate::runtime::error::SessionError;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;
use itertools::izip;
use std::io::Cursor;
pub struct CreateCharacterHandler;

impl CreateCharacterHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        self: &Self,
        state: SharedState,
        session: Session,
        packet: Packet,
    ) -> Result<HandlerResult<LoginAction>, NetworkError> {
        let mut reader = Cursor::new(packet.bytes);
        reader
            .read_short()
            .map_err(ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
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
        let top = reader
            .read_int() // Slot 5
            .map_err(ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        let bottom = reader
            .read_int() // Slot 6
            .map_err(ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        let shoes = reader
            .read_int() // Slot 7
            .map_err(ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        let weapon = reader
            .read_int() // Special
            .map_err(ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        let gender = reader
            .read_byte()
            .map_err(ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)? as i16;
        let acc_id = session
            .acc_id
            .ok_or(SessionError::NoAccount)
            .map_err(NetworkError::from)?;
        let acc = account::query::get_account_by_id(state.clone(), acc_id)
            .await
            .map_err(DatabaseError::from)
            .map_err(NetworkError::from)?;
        let new_char = NewCharacter {
            acc_id,
            ign: ign.clone(),
            world_id: acc
                .selected_world_id
                .ok_or(WorldError::NotSelected(acc_id))
                .map_err(ModelError::from)
                .map_err(NetworkError::from)?,
            job,
            face,
            hair,
            hair_color,
            skin,
            gender,
            map: map,
        };
        let char = character::query::create_character(state.clone(), &new_char)
            .await
            .map_err(DatabaseError::from)
            .map_err(NetworkError::from)?;
        let char_equips = NewCharacterEquipment {
            char_id: char.id,
            hat: None, // might exist or not based on class
            top,
            bottom: Some(bottom), // might be none
            shoes,
            weapon,
            shield: None, // might exist or not based on class
        };
        let char_equips = character::query::create_character_equipment(state.clone(), &char_equips)
            .await
            .map_err(DatabaseError::from)
            .map_err(NetworkError::from)?;
        let cash_equips = NewCashEquipment { char_id: char.id };
        let cash_equips = character::query::create_cash_equipment(state.clone(), &cash_equips)
            .await
            .map_err(DatabaseError::from)
            .map_err(NetworkError::from)?;
        let binds: Vec<NewKeybinding> = izip!(DEFAULT_KEY, DEFAULT_TYPE, DEFAULT_ACTION)
            .map(|(key, bind_type, action): (i16, u8, i16)| NewKeybinding {
                char_id: char.id,
                key,
                bind_type: bind_type.into(),
                action,
            })
            .collect();
        keybinding::query::update_keybindings(state.clone(), binds)
            .await
            .map_err(DatabaseError::from)
            .map_err(NetworkError::from)?;
        let mut result = HandlerResult::new();
        let packet = build_create_char_packet(&char, &char_equips, &cash_equips)?;
        let action = LoginAction::SendPacket { packet };
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

fn build_create_char_packet(
    char: &Character,
    char_equips: &CharacterEquipment,
    cash_equips: &CashEquipment,
) -> Result<Packet, NetworkError> {
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
    character::list_service::write_list_char(&mut packet, &char, &char_equips, &cash_equips)
        .map_err(ModelError::from)
        .map_err(NetworkError::from)?;
    Ok(packet)
}
