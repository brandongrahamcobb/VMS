/* entity/src/map/model.rs
 * The purpose of this module is to provide a map models.
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

use tokio::time::Instant;

#[derive(Clone)]
pub struct Point {
    pub x: i16,
    pub y: i16,
}

#[derive(Clone)]
pub struct MapWzInfo {
    pub death_map_wz: i32,
    pub wz: i32,
    pub mob_rate: f32,
}

#[derive(Clone)]
pub struct MapModel {
    pub wz: i32,
}

pub enum VacancyState {
    Populated { start: Instant },
    Vacant,
}

// let mob_respawn_handler: MobRespawnHandler = mob_respawn::handler::MobRespawnHandler::new();
// mob_respawn_handler
//     .handle(state, tick_tx.clone(), world_id, channel_id, map_wz)
//     .await?;
