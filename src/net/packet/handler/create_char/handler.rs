use crate::constants::{DEFAULT_ACTION, DEFAULT_KEY, DEFAULT_TYPE};
use crate::models::character::equipment_set::model::{
    NewAndroidEquipmentSet, NewCashEquipmentSet, NewPetEquipmentSet, NewRegularEquipmentSet,
};
use crate::models::character::keybinding::model::NewKeybinding;
use crate::models::character::model::{Character, NewCharacter};
use crate::models::character::{equipment_set, keybinding};
use crate::models::{character, wz};
use crate::net::action::model::LoginAction;
use crate::net::error::NetworkError;
use crate::net::packet::handler::create_char;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::model::Packet;
use crate::runtime::error::SessionError;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;
use itertools::izip;

pub struct CreateCharHandler;

impl CreateCharHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        &self,
        state: &SharedState,
        session: &Session,
        packet: &Packet,
    ) -> Result<HandlerResult<LoginAction>, NetworkError> {
        let read = create_char::read::read_create_character_packet(packet)?;
        let map_id = create_char::service::get_map_id_for_job(&read.job_id)?;
        let world_id = session
            .world_id
            .ok_or(SessionError::NoWorldSelected(session.id))?;
        let acc_id = session.acc_id;
        let char = NewCharacter {
            acc_id,
            ign: read.ign,
            world_id: world_id as i16,
            job_id: read.job_id,
            face_id: read.face_id,
            hair_id: read.hair_id,
            hair_color_id: read.hair_color_id,
            skin_id: read.skin_id,
            gender_id: read.gender_id,
            map_id: map_id,
        };
        let char = character::query::create_character(state, &char).await?;
        let binds: Vec<NewKeybinding> = izip!(DEFAULT_KEY, DEFAULT_TYPE, DEFAULT_ACTION)
            .map(|(key, bind_type, action): (i32, i16, i32)| NewKeybinding {
                char_id: char.id.clone(),
                key,
                bind_type: bind_type.into(),
                action,
            })
            .collect();
        let top = wz::equip::service::generate_new_equip(state, &read.top_id).await?;
        let bottom = wz::equip::service::generate_new_equip(state, &read.bottom_id).await?;
        let shoes = wz::equip::service::generate_new_equip(state, &read.shoes_id).await?;
        let weapon = wz::equip::service::generate_new_equip(state, &read.weapon_id).await?;
        let regular_equips = NewRegularEquipmentSet {
            char_id: char.id.clone(),
            top: top.id,
            bottom: bottom.id,
            shoes: shoes.id,
            weapon: weapon.id,
        };
        let cash_equips = NewCashEquipmentSet {
            char_id: char.id.clone(),
        };
        let android_equips = NewAndroidEquipmentSet {
            char_id: char.id.clone(),
        };
        let pet_equips = NewPetEquipmentSet {
            char_id: char.id.clone(),
        };
        let result = complete_create_char_handler(
            state,
            &char,
            &binds,
            &regular_equips,
            &cash_equips,
            &android_equips,
            &pet_equips,
        )
        .await?;
        Ok(result)
    }
}

async fn complete_create_char_handler(
    state: &SharedState,
    char: &Character,
    binds: &Vec<NewKeybinding>,
    regular_equips: &NewRegularEquipmentSet,
    cash_equips: &NewCashEquipmentSet,
    android_equips: &NewAndroidEquipmentSet,
    pet_equips: &NewPetEquipmentSet,
) -> Result<HandlerResult<LoginAction>, NetworkError> {
    keybinding::query::update_keybindings(state, binds).await?;
    let regular_equips =
        equipment_set::query::create_regular_equipment_set_for_new_character(state, regular_equips)
            .await?;
    let cash_equips =
        equipment_set::query::create_cash_equipment_set_for_new_character(state, cash_equips)
            .await?;
    equipment_set::query::create_android_equipment_set_for_new_character(state, android_equips)
        .await?;
    equipment_set::query::create_pet_equipment_set_for_new_character(state, pet_equips).await?;
    let mut result: HandlerResult<LoginAction> = HandlerResult::new();
    let packet: Packet = Packet::new_empty()
        .build_create_char_handler_packet(state, char, &regular_equips, &cash_equips)
        .await?
        .finish();
    result.add_action(LoginAction::SendLocalPacket {
        packet: packet.clone(),
    });
    Ok(result)
}
