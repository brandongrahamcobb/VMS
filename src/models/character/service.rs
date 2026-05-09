use crate::models::character::error::CharacterError;
use crate::models::character::keybinding::model::{Keybinding, KeybindingModel};
use crate::models::character::model::{Character, CharacterModel};
use crate::models::character::skill::model::Skill;
use crate::models::character::{self, equipment_set, keybinding, skill};
use crate::models::error::ModelError;
use crate::models::shroom::{job, map, world};
use crate::runtime::state::SharedState;

impl CharacterModel {
    pub fn get_id(&self) -> Result<i32, ModelError> {
        if let Some(char_id) = self.id {
            Ok(char_id)
        } else {
            Err(ModelError::from(CharacterError::NoId))
        }
    }

    pub async fn load(&self, state: &SharedState) -> Result<Character, ModelError> {
        let char_id: i32 = self.get_id()?;
        let bind_models: Vec<KeybindingModel> =
            keybinding::query::getters::get_keybinding_models_by_character_id(state, char_id)
                .await?;
        let mut binds: Vec<Keybinding> = Vec::<Keybinding>::new();
        for bind_model in bind_models {
            binds.push(bind_model.load()?);
        }
        let android_equips_model =
            equipment_set::android::query::getters::get_android_equipment_set_model_by_character_id(
                state, char_id,
            )
            .await?;
        let android_equip_set = android_equips_model.load(state).await?;
        let cash_equips_model =
            equipment_set::cash::query::getters::get_cash_equipment_set_model_by_character_id(
                state, char_id,
            )
            .await?;
        let cash_equip_set = cash_equips_model.load(state).await?;
        let pet_equips_model =
            equipment_set::pet::query::getters::get_pet_equipment_set_model_by_character_id(
                state, char_id,
            )
            .await?;
        let pet_equip_set = pet_equips_model.load(state).await?;
        let regular_equips_model =
            equipment_set::regular::query::getters::get_regular_equipment_set_model_by_character_id(
                state, char_id,
            )
            .await?;
        let regular_equip_set = regular_equips_model.load(state).await?;
        let skill_models =
            skill::query::getters::get_skill_models_by_character_id(state, char_id).await?;
        let mut skills: Vec<Skill> = Vec::<Skill>::new();
        for skill_model in skill_models {
            skills.push(skill_model.load()?);
        }
        let job = job::service::get_job_by_id(self.job_id)?;
        let map = map::service::get_map_by_id(self.map_id)?;
        let world = world::service::get_world_by_id(state, self.world_id).await?;
        Ok(Character {
            model: self.clone(),
            regular_equip_set,
            cash_equip_set,
            pet_equip_set,
            android_equip_set,
            skills,
            binds,
            world,
            map,
            job,
        })
    }
}

pub async fn get_character_by_id(
    state: &SharedState,
    char_id: i32,
) -> Result<Character, ModelError> {
    let char_model = character::query::getters::get_character_model_by_id(state, char_id).await?;
    let char = char_model.load(state).await?;
    Ok(char)
}
