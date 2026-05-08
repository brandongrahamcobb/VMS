use crate::db::schema::skills;
use crate::models::character::skill::model::{NewCharacterSkillInsert, SkillModel};
use crate::runtime::state::SharedState;
use diesel::expression_methods::*;
use diesel::{QueryDsl, QueryResult, RunQueryDsl};
use serde_json;

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

pub async fn create_skills_by_character_id_and_job_id(
    state: &SharedState,
    char_id: i32,
    map: serde_json::Value,
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
    let skill_ids: Vec<i32> = map
        .as_object()
        .unwrap()
        .keys()
        .filter_map(|k| k.parse::<i32>().ok())
        .collect();
    let mut skill_model_inserts: Vec<NewCharacterSkillInsert> = Vec::new();
    for skill_id in skill_ids {
        skill_model_inserts.push(NewCharacterSkillInsert {
            char_id: char_id,
            wz_id: skill_id,
            level: 0,
        });
    }
    diesel::insert_into(skills::table)
        .values(skill_model_inserts)
        .load::<SkillModel>(&mut conn)
}
