use crate::models::character;
use crate::models::character::model::Character;
use crate::models::character::skill::model::Skill;
use crate::net::error::NetworkError;
use crate::net::packet::handler::close_attack::reader::CloseAttackReader;
use crate::runtime::session::model::Session;
use crate::runtime::state::SharedState;
use std::collections::HashMap;

#[derive(Clone)]
pub struct CloseAttackStore {
    pub char: Character,
    pub skill: Skill,
    pub count: i16,
    pub display: i16,
    pub toleft: i16,
    pub stance: i16,
    pub speed: i16,
    pub mob_damages: HashMap<i32, Vec<i32>>,
}

impl CloseAttackStore {
    pub async fn store_close_attack(
        state: &SharedState,
        session: Session,
        reader: CloseAttackReader,
    ) -> Result<Self, NetworkError> {
        let char = session.get_char()?;
        let skill_model =
            character::skill::query::getters::get_skill_model_by_character_id_and_skill_id(
                state,
                char.model.get_id()?,
                reader.skill_id,
            )
            .await?;
        let skill = skill_model.load()?;
        return Ok(Self {
            char: char.clone(),
            skill: skill.clone(),
            count: reader.count,
            display: reader.display,
            toleft: reader.toleft,
            stance: reader.stance,
            speed: reader.speed,
            mob_damages: reader.mob_damages.clone(),
        });
    }
}
