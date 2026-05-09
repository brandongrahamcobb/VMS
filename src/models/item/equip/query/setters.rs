use crate::db::schema::equips;
use crate::models::item::equip::model::EquipModel;
use crate::runtime::state::SharedState;
use diesel::{QueryResult, RunQueryDsl};

pub async fn update_equips(
    state: &SharedState,
    equips: Vec<EquipModel>,
) -> QueryResult<Vec<EquipModel>> {
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
    let mut results: Vec<EquipModel> = Vec::<EquipModel>::new();
    for equip in &equips {
        results.push(
            diesel::insert_into(equips::table)
                .values(equip)
                .on_conflict(equips::id)
                .do_update()
                .set(equip)
                .get_result::<EquipModel>(&mut conn)?,
        )
    }
    Ok(results)
}
