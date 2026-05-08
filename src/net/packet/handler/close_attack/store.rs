use crate::models::character;
use crate::models::character::model::CharacterModel;
use crate::models::character::skill::model::SkillModel;
use crate::net::error::NetworkError;
use crate::net::packet::handler::close_attack::reader::CloseAttackReader;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;
use std::collections::HashMap;

#[derive(Clone)]
pub struct CloseAttackStore {
    pub char_model: CharacterModel,
    pub skill_model: SkillModel,
    pub count: i8,
    pub display: i8,
    pub toleft: i8,
    pub stance: i8,
    pub speed: i8,
    pub mob_damages: HashMap<i32, Vec<i32>>,
}

impl CloseAttackStore {
    pub async fn store_close_attack(
        state: &SharedState,
        session: Session,
        reader: CloseAttackReader,
    ) -> Result<Self, NetworkError> {
        let char_model: CharacterModel = session.char.model.clone();
        let skill_model: SkillModel =
            character::skill::query::get_skill_model_by_character_id_and_skill_id(
                state,
                char_model.id,
                reader.skill_id,
            )
            .await?;
        Ok(Self {
            char_model,
            skill_model,
            count: reader.count,
            display: reader.display,
            toleft: reader.toleft,
            stance: reader.stance,
            speed: reader.speed,
            mob_damages: reader.mob_damages,
        })
    }
}
