use crate::db::schema::skills;
use crate::models::character::skill::model::SkillModel;
use crate::runtime::state::SharedState;
use diesel::expression_methods::*;
use diesel::{QueryDsl, QueryResult, RunQueryDsl};

pub async fn get_skill_by_character_id_and_skill_id(
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

pub async fn get_skills_by_character_id(
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
        .load::<Vec<SkillModel>>(&mut conn)
}

pub async fn create_skills_by_character_id_and_job_id(
    state: &SharedState,
    char_id: i32,
    job_id: i32,
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
    skills::table.load::<SkillModel>(&mut conn)
    // need to setup skill/job mapping for new chars
}
