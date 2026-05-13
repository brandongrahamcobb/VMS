/* equip_stats/query/setters.rs
 * The purpose of this module is to provide database setters for equipment statistics.
 *
 * Copyright (C) 2026  https://github.com/brandongrahamcobb/VMS.git
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use crate::db::schema::equip_stats;
use crate::models::item::equip_stats::model::EquipStatsModel;
use crate::runtime::state::SharedState;
use diesel::{QueryResult, RunQueryDsl};

pub async fn update_equip_stats(
    state: &SharedState,
    equip_stats_set: Vec<EquipStatsModel>,
) -> QueryResult<Vec<EquipStatsModel>> {
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
    let mut results: Vec<EquipStatsModel> = Vec::<EquipStatsModel>::new();
    for equip_stats_item in &equip_stats_set {
        results.push(
            diesel::insert_into(equip_stats::table)
                .values(equip_stats_item)
                .on_conflict(equip_stats::id)
                .do_update()
                .set(equip_stats_item)
                .get_result::<EquipStatsModel>(&mut conn)?,
        )
    }
    Ok(results)
}
