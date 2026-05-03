use crate::constants::{DEFAULT_ACTION, DEFAULT_KEY, DEFAULT_TYPE};
use crate::db::error::DatabaseError;
use crate::models::character::equipment_set::model::{NewAndroidEquipmentSet, NewCashEquipmentSet, NewPetEquipmentSet, NewRegularEquipmentSet};
use crate::models::character::keybinding::model::NewKeybinding;
use crate::models::character::model::NewCharacter;
use crate::models::error::ModelError;
use crate::models::world::error::WorldError;
use crate::models::{account, character, wz};
use crate::net::error::NetworkError;
use crate::net::packet::error::PacketError;
use crate::net::packet::handler::action::LoginAction;
use crate::net::packet::handler::create_char;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::io::error::IOError::ReadError;
use crate::net::packet::packet::Packet;
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
        &self,
        state: SharedState,
        session: Session,
        packet: Packet,
    ) -> Result<HandlerResult<LoginAction>, NetworkError> {
        let mut pkt_reader = Cursor::new(packet.bytes);
        pkt_reader
            .read_short()
            .map_err(ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        let ign = pkt_reader
            .read_str_with_length()
            .map_err(ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        let job = pkt_reader
            .read_int()
            .map_err(ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)? as i16;
        let map = create_char::service::get_map_id_for_job(job)?;
        let face = pkt_reader
            .read_int()
            .map_err(ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        let hair = pkt_reader
            .read_int()
            .map_err(ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        let hair_color = pkt_reader
            .read_int()
            .map_err(ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        let skin = pkt_reader
            .read_int()
            .map_err(ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        let top = pkt_reader
            .read_int() // Slot 5
            .map_err(ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        let bottom = pkt_reader
            .read_int() // Slot 6
            .map_err(ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        let shoes = pkt_reader
            .read_int() // Slot 7
            .map_err(ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        let weapon = pkt_reader
            .read_int() // Special
            .map_err(ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        let gender = pkt_reader
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
        let binds: Vec<NewKeybinding> = izip!(DEFAULT_KEY, DEFAULT_TYPE, DEFAULT_ACTION)
            .map(|(key, bind_type, action): (i16, u8, i16)| NewKeybinding {
                char_id: char.id,
                key,
                bind_type: bind_type.into(),
                action,
            })
            .collect();
        character::keybinding::query::update_keybindings(state.clone(), binds)
            .await
            .map_err(DatabaseError::from)
            .map_err(NetworkError::from)?;
        let top = wz::equip::service::generate_new_equip(state.clone(), top).await?;
        let bottom = wz::equip::service::generate_new_equip(state.clone(), bottom).await?;
        let shoes = wz::equip::service::generate_new_equip(state.clone(), shoes).await?;
        let weapon = wz::equip::service::generate_new_equip(state.clone(), weapon).await?;
        let regular_equips = NewRegularEquipmentSet {
            char_id: char.id,
            top: top.id,
            bottom: bottom.id,
            shoes: shoes.id,
            weapon: weapon.id,
        };
        let regular_equips =
            character::equipment_set::query::create_regular_equipment_set_for_new_character(state.clone(), &regular_equips)
                .await
                .map_err(DatabaseError::from)
                .map_err(NetworkError::from)?;
        let cash_equips = NewCashEquipmentSet {
            char_id: char.id
        };
        let cash_equips = character::equipment_set::query::create_cash_equipment_set_for_new_character(state.clone(), &cash_equips)
                .await
                .map_err(DatabaseError::from)
                .map_err(NetworkError::from)?;
        let android_equips = NewAndroidEquipmentSet {
            char_id: char.id
        };
        let _android_equips = character::equipment_set::query::create_android_equipment_set_for_new_character(state.clone(), &android_equips)
                .await
                .map_err(DatabaseError::from)
                .map_err(NetworkError::from)?;
        let pet_equips = NewPetEquipmentSet {
            char_id: char.id
        };
        let _pet_equips = character::equipment_set::query::create_pet_equipment_set_for_new_character(state.clone(), &pet_equips)
                .await
                .map_err(DatabaseError::from)
                .map_err(NetworkError::from)?;
        let mut result: HandlerResult<LoginAction> = HandlerResult::new();
        let packet: Packet = Packet::new_empty()
            .build_create_char_handler_packet(state.clone(), &char, &regular_equips, &cash_equips)
            .await?
            .finish();
        let action = LoginAction::SendPacket { packet };
        result.add_action(action)?;
        Ok(result)
    }
}
