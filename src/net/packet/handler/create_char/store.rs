use crate::models::character;
use crate::models::character::equipment_set::model::{
    AndroidEquipmentSet, CashEquipmentSet, NewAndroidEquipmentSet, NewCashEquipmentSet,
    NewPetEquipmentSet, NewRegularEquipmentSet, PetEquipmentSet, RegularEquipmentSet,
};
use crate::models::character::keybinding::model::NewKeybinding;
use crate::models::character::model::{Character, NewCharacter};
use crate::models::character::{equipment_set, keybinding};
use crate::models::wz;
use crate::net::packet::handler::create_char::read::CreateCharacterRead;
use crate::runtime::error::SessionError;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;

pub struct CreateCharStore {
    pub char: Character,
    pub binds: Vec<Keybinding>,
    pub regular_equips: RegularEquipmentSet,
    pub cash_equips: CashEquipmentSet,
    pub android_equips: AndroidEquipmentSet,
    pub pet_equips: PetEquipmentSet,
}

impl CreateCharStore {
    pub fn new() -> Self {
        Self
    }

    pub async fn store_create_char(
        &self,
        state: &SharedState,
        session: &Session,
        read: &CreateCharacterRead,
    ) -> Result<Self, NetworkError> {
        let map_id = create_char::service::get_map_id_for_job(&read.job_id)?;
        let world_id = session
            .world_id
            .ok_or(SessionError::NoWorldSelected(session.id))?;
        let acc_id = session.acc.id;
        let new_char = NewCharacter {
            acc_id: acc_id.clone(),
            ign: read.ign.clone(),
            world_id: world_id.clone() as i16,
            job_id: read.job_id.clone(),
            face_id: read.face_id.clone(),
            hair_id: read.hair_id.clone(),
            hair_color_id: read.hair_color_id.clone(),
            skin_id: read.skin_id.clone(),
            gender_id: read.gender_id.clone(),
            map_id: map_id.clone(),
        };
        let char = character::query::create_character(state, &new_char).await?;
        let new_binds: Vec<NewKeybinding> = izip!(DEFAULT_KEY, DEFAULT_TYPE, DEFAULT_ACTION)
            .map(|(key, bind_type, action): (i32, i16, i32)| NewKeybinding {
                char_id: char.id.clone(),
                key: key.clone(),
                bind_type: bind_type,
                action: action.clone(),
            })
            .collect();
        let binds = keybinding::query::update_keybindings(state, &binds).await?;
        let top = wz::equip::service::generate_new_equip(state, &read.top_id).await?;
        let bottom = wz::equip::service::generate_new_equip(state, &read.bottom_id).await?;
        let shoes = wz::equip::service::generate_new_equip(state, &read.shoes_id).await?;
        let weapon = wz::equip::service::generate_new_equip(state, &read.weapon_id).await?;
        let new_regular_equips = NewRegularEquipmentSet {
            char_id: char.id.clone(),
            top: top.id.clone()
            bottom: bottom.id.clone(),
            shoes: shoes.id.clone(),
            weapon: weapon.id.clone(),
        };
        let new_cash_equips = NewCashEquipmentSet {
            char_id: char.id.clone(),
        };
        let new_android_equips = NewAndroidEquipmentSet {
            char_id: char.id.clone(),
        };
        let new_pet_equips = NewPetEquipmentSet {
            char_id: char.id.clone(),
        };
        let regular_equips = equipment_set::query::create_regular_equipment_set_for_new_character(
            state,
            &new_regular_equips,
        )
        .await?;
        let cash_equips = equipment_set::query::create_cash_equipment_set_for_new_character(
            state,
            &new_cash_equips,
        )
        .await?;
        let android_equips = equipment_set::query::create_android_equipment_set_for_new_character(
            state,
            &new_android_equips,
        )
        .await?;
        let pet_equips = equipment_set::query::create_pet_equipment_set_for_new_character(
            state,
            &new_pet_equips,
        )
        .await?;
        Ok(Self {
            char: char.clone(),
            binds: binds.clone(),
            regular_equips: regular_equips.clone(),
            cash_equips: cash_equips.clone(),
            android_equips: android_equips.clone(),
            pet_equips: pet_equips.clone(),
        })
    }
}
