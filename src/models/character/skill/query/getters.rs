use crate::db::schema::skills;
use crate::models::character::skill::model::SkillModel;
use crate::runtime::state::SharedState;
use diesel::expression_methods::*;
use diesel::{QueryDsl, QueryResult, RunQueryDsl};

pub async fn get_skill_model_by_character_id_and_skill_id(
    state: &SharedState,
    char_id: i32,
    skill_id: i32,
) -> QueryResult<SkillModel> {
    let db = {
        let state = state.lock().await;
        state.db.clone()
    };
    let mut conn = db.get().map_err(|e| {
        diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::UnableToSendCommand,
            Box::new(e.to_string()),
        )
    })?;
    skills::table
        .filter(skills::char_id.eq(char_id))
        .filter(skills::wz_id.eq(skill_id))
        .first::<SkillModel>(&mut conn)
}

pub async fn get_skill_models_by_character_id(
    state: &SharedState,
    char_id: i32,
) -> QueryResult<Vec<SkillModel>> {
    let db = {
        let state = state.lock().await;
        state.db.clone()
    };
    let mut conn = db.get().map_err(|e| {
        diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::UnableToSendCommand,
            Box::new(e.to_string()),
        )
    })?;
    skills::table
        .filter(skills::char_id.eq(char_id))
        .load::<SkillModel>(&mut conn)
}
