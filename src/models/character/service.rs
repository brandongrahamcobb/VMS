use crate::models::character::equipment_set::android::model::AndroidEquipmentSet;
use crate::models::character::equipment_set::cash::model::CashEquipmentSet;
use crate::models::character::equipment_set::pet::model::PetEquipmentSet;
use crate::models::character::equipment_set::regular::model::RegularEquipmentSet;
use crate::models::character::keybinding::model::Keybinding;
use crate::models::character::model::{Character, CharacterModel};
use crate::models::character::skill::model::Skill;
use crate::models::character::{self, equipment_set, keybinding, skill};
use crate::models::error::ModelError;
use crate::models::map::model::Map;
use crate::models::world::model::World;
use crate::models::{map, world};
use crate::runtime::state::SharedState;
use std::time::SystemTime;

pub async fn get_characters_by_account_id(
    state: &SharedState,
    acc_id: i32,
) -> Result<Vec<Character>, ModelError> {
    let char_models = character::query::get_characters_models_by_account_id(state, acc_id).await?;
    let mut chars: Vec<Character> = Vec::<Character>::new();
    for char_model in char_models {
        chars.push(get_character_by_id(state, char_model.id).await?)
    }
    Ok(chars)
}

pub async fn get_character_by_id(
    state: &SharedState,
    char_id: i32,
) -> Result<Character, ModelError> {
    let char_model = character::query::get_character_model_by_id(state, char_id).await?;
    let binds_models =
        keybinding::query::get_keybinding_models_by_character_id(state, char_model.id).await?;
    let mut binds: Vec<Keybinding> = Vec::new();
    for bind_model in binds_models {
        binds.push(Keybinding { model: bind_model });
    }
    let android_equips_model =
        equipment_set::android::query::get_android_equipment_set_model_by_character_id(
            state,
            char_model.id,
        )
        .await?;
    let android_equips = equipment_set::android::service::get_android_equipment_set_from_model(
        state,
        android_equips_model.clone(),
    )
    .await?;
    let cash_equips_model =
        equipment_set::cash::query::get_cash_equipment_set_model_by_character_id(
            state,
            char_model.id,
        )
        .await?;
    let cash_equips = equipment_set::cash::service::get_cash_equipment_set_from_model(
        state,
        cash_equips_model.clone(),
    )
    .await?;
    let pet_equips_model = equipment_set::pet::query::get_pet_equipment_set_model_by_character_id(
        state,
        char_model.id,
    )
    .await?;
    let pet_equips = equipment_set::pet::service::get_pet_equipment_set_from_model(
        state,
        pet_equips_model.clone(),
    )
    .await?;
    let regular_equips_model =
        equipment_set::regular::query::get_regular_equipment_set_model_by_character_id(
            state,
            char_model.id,
        )
        .await?;
    let regular_equips = equipment_set::regular::service::get_regular_equipment_set_from_model(
        state,
        regular_equips_model.clone(),
    )
    .await?;
    let skills = skill::service::get_skills_by_character_id(state, char_model.id).await?;
    let map = map::service::get_map_by_id(char_model.map_id)?;
    let world = world::service::get_world_by_id(state, char_model.world_id).await?;
    Ok(Character {
        model: char_model,
        regular_equips,
        cash_equips,
        pet_equips,
        android_equips,
        skills,
        binds,
        world,
        map,
    })
}

impl Character {
    pub fn new() -> Self {
        Self {
            model: CharacterModel::new(),
            regular_equips: RegularEquipmentSet::new(),
            cash_equips: CashEquipmentSet::new(),
            pet_equips: PetEquipmentSet::new(),
            android_equips: AndroidEquipmentSet::new(),
            skills: Vec::<Skill>::new(),
            binds: Vec::<Keybinding>::new(),
            world: World::new(),
            map: Map::new(),
        }
    }
}

impl CharacterModel {
    pub fn new() -> Self {
        Self {
            id: -1,
            acc_id: -1,
            world_id: -1,
            ign: String::new(),
            level: -1,
            exp: -1,
            strength: -1,
            dexterity: -1,
            luck: -1,
            intelligence: -1,
            hp: -1,
            mp: -1,
            max_hp: -1,
            max_mp: -1,
            ap: -1,
            fame: -1,
            meso: -1,
            job_id: -1,
            face_id: -1,
            hair_id: -1,
            hair_color_id: -1,
            skin_id: -1,
            gender_id: -1,
            map_id: -1,
            created_at: SystemTime::now(),
            updated_at: SystemTime::now(),
        }
    }
}
