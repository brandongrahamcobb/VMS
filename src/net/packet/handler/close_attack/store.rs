use crate::models::character;
use crate::models::character::model::Character;
use crate::models::character::skill::model::Skill;
use crate::runtime::error::SessionError;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;
use itertools::izip;

pub struct CloseAttackStore {
    pub char: Character,
    pub skill: Skill,
    pub count: u8,
    pub display: u8,
    pub toleft: u8,
    pub stance: u8,
    pub speed: u8,
    pub mob_damages: HashMap<i32, Vec<i32>>,
}

impl CloseAttackStore {
    pub fn new() -> Self {
        Self
    }

    pub async fn store_close_attack(
        &self,
        state: &SharedState,
        session: &Session,
        read: &CloseAttackRead,
    ) -> Result<Self, NetworkError> {
        let char_id = session
            .char_id
            .ok_or(SessionError::NoCharacterSelected(session.id))?;
        let char = character::query::get_character_by_id(state, &char_id)?;
        let skill: u8 = character::skill::query::get_skill_ids(state, &char_id, &read.skill_id)
            .await
            .unwrap_or(0) as u8;
        Ok(Self {
            char: char.clone(),
            skill: skill.clone(),
            count: *read.count.clone(),
            display: *read.display.clone(),
            toleft: *read.toleft.clone(),
            stance: *read.stance.clone(),
            speed: *read.speed.clone(),
            mob_damages: *read.mob_damages.clone(),
        })
    }
}
