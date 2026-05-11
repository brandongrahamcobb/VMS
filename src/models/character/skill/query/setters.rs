use crate::db::schema::skills;
use crate::models::character::skill::model::SkillModel;
use crate::runtime::state::SharedState;
use diesel::expression_methods::*;
use diesel::{QueryDsl, QueryResult, RunQueryDsl};

pub async fn update_skills(
    state: &SharedState,
    skill_models: Vec<SkillModel>,
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
    let mut results = Vec::new();
    for skill_model in &skill_models {
        results.push(
            diesel::insert_into(skills::table)
                .values(skill_model)
                .on_conflict((skills::char_id, skills::wz_id))
                .do_update()
                .set(skill_model)
                .get_result::<SkillModel>(&mut conn)?,
        )
    }
    Ok(results)
}

pub async fn delete_skills_by_char_id(state: &SharedState, char_id: i32) -> QueryResult<usize> {
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
    diesel::delete(skills::table.filter(skills::char_id.eq(char_id))).execute(&mut conn)
}
