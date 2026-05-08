use crate::constants::{DEFAULT_ACTION, DEFAULT_KEY, DEFAULT_TYPE};
use crate::models::character::equipment_set::android::model::NewCharacterAndroidEquipmentSetInsert;
use crate::models::character::equipment_set::cash::model::NewCharacterCashEquipmentSetInsert;
use crate::models::character::equipment_set::pet::model::NewCharacterPetEquipmentSetInsert;
use crate::models::character::equipment_set::regular::model::NewCharacterRegularEquipmentSetInsert;
use crate::models::character::keybinding::model::NewCharacterKeybindingInsert;
use crate::models::character::model::NewCharacterInsert;
use crate::models::character::{equipment_set, keybinding, skill};
use crate::models::map::model::MapModel;
use crate::models::world::model::WorldModel;
use crate::models::wz;
use crate::models::{character, map};
use crate::net::error::NetworkError;
use crate::net::packet::handler::create_char::reader::CreateCharReader;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;
use itertools::izip;

#[derive(Clone)]
pub struct CreateCharStore {
    pub char: Character,
}

impl CreateCharStore {
    pub async fn store_create_char(
        state: &SharedState,
        session: Session,
        reader: CreateCharReader,
    ) -> Result<Self, NetworkError> {
        let map_model: MapModel = map::service::get_map_model_for_job(&reader.job_id)?;
        let world_model: WorldModel = session.world.model.clone();
        let acc_model = session.acc.model.clone();
        let char_insert = NewCharacterInsert {
            acc_id: acc_model.id,
            ign: reader.ign.clone(),
            world_id: world_model.id as i16,
            job_id: reader.job_id,
            face_id: reader.face_id,
            hair_id: reader.hair_id,
            hair_color_id: reader.hair_color_id,
            skin_id: reader.skin_id,
            gender_id: reader.gender_id,
            map_id: map_model.id,
        };
        let char_model = character::query::create_character_model(state, &char_insert).await?;
        let binds_insert: Vec<NewCharacterKeybindingInsert> =
            izip!(DEFAULT_KEY, DEFAULT_TYPE, DEFAULT_ACTION)
                .map(
                    |(key, bind_type, action): (i32, i16, i32)| NewCharacterKeybindingInsert {
                        char_id: char_model.id,
                        key,
                        bind_type,
                        action,
                    },
                )
                .collect();
        let binds = keybinding::query::update_keybindings(state, &binds_insert).await?;
        let top_insert = wz::equip::service::create_equip_insert(reader.top_id)?;
        let top_model = wz::equip::query::create_equip_model(state, top_model.clone()).await?;
        let bottom_insert = wz::equip::service::create_equip_insert(reader.bottom_id)?;
        let bottom_model =
            wz::equip::query::create_equip_model(state, bottom_model.clone()).await?;
        let shoes_insert = wz::equip::service::create_equip_insert(reader.shoes_id)?;
        let shoes_model = wz::equip::query::create_equip_model(state, shoes_model.clone()).await?;
        let weapon_insert = wz::equip::service::create_equip_insert(reader.weapon_id).await?;
        let weapon_model =
            wz::equip::query::create_equip_model(state, weapon_model.clone()).await?;
        let regular_equips_insert = NewCharacterRegularEquipmentSetInsert {
            char_id: char_model.id,
            top_id: top_model.id,
            bottom_id: bottom_model.id,
            shoes_id: shoes_model.id,
            weapon_id: weapon_model.id,
        };
        let cash_equips_insert = NewCharacterCashEquipmentSetInsert {
            char_id: char_model.id,
        };
        let android_equips_insert = NewCharacterAndroidEquipmentSetInsert {
            char_id: char_model.id,
        };
        let pet_equips_insert = NewCharacterPetEquipmentSetInsert {
            char_id: char_model.id,
        };
        let regular_equips_model =
            equipment_set::regular::query::create_regular_equipment_set_model_for_new_character(
                state,
                regular_equips_insert,
            )
            .await?;
        let cash_equips_model =
            equipment_set::cash::query::create_cash_equipment_set_model_for_new_character(
                state,
                cash_equips_insert,
            )
            .await?;
        let android_equips_model =
            equipment_set::android::query::create_android_equipment_set_model_for_new_character(
                state,
                android_equips_insert,
            )
            .await?;
        let pet_equips_model =
            equipment_set::pet::query::create_pet_equipment_set_model_for_new_character(
                state,
                pet_equips_insert,
            )
            .await?;
        let android_equips = equipment_set::android::service::get_android_equipment_set_from_model(
            state,
            android_equips_model.clone(),
        )
        .await?;
        let cash_equips = equipment_set::cash::service::get_cash_equipment_set_from_model(
            state,
            cash_equips_model.clone(),
        )
        .await?;
        let pet_equips = equipment_set::pet::service::get_pet_equipment_set_from_model(
            state,
            pet_equips_model.clone(),
        )
        .await?;
        let regular_equips = equipment_set::regular::service::get_regular_equipment_set_from_model(
            state,
            regular_equips_model.clone(),
        )
        .await?;
        let skills = skill::query::create_skills_by_character_id_and_job_id(
            state,
            char_model.id,
            reader.job_id,
        )
        .await?;
        let char = Character {
            model: char_model,
            regular_equips,
            cash_equips,
            pet_equips,
            android_equips,
            skills,
            binds,
            world,
            map,
        };
        Ok(Self { char })
    }
}
