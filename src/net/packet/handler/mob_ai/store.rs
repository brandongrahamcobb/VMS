/* mob_ai/store.rs
 * The purpose of this module is to resolve relevant variables for mob AI.
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

use crate::models::map::model::Point;
use crate::net::packet::handler::mob_ai::error::MobAiError;
use crate::net::packet::handler::mob_ai::reader::MobAiReader;
use crate::runtime::session::model::Session;
use crate::runtime::state::SharedState;

pub struct MobAiStore {
    pub mob_id: u32,
    pub skillb: u8,
    pub skill0: u8,
    pub skill1: u8,
    pub skill2: u8,
    pub skill3: u8,
    pub skill4: u8,
    pub command: u8,
    pub origin: Point,
    pub next: Point,
    pub last: Point,
    pub fh: u16,
    pub new_state: u8,
    pub duration: i16,
}

impl MobAiStore {
    pub async fn store_mob_ai(
        state: &SharedState,
        session: &Session,
        reader: &MobAiReader,
    ) -> Result<Self, MobAiError> {
        let world_id: i16 = session.get_world_id()?;
        let channel_id: u8 = session.get_channel_id()?;
        let map_wz: i32 = session.get_map_wz()?;
        {
            let state = state.lock().await;
            state
                .with_mut_map(world_id, channel_id, map_wz, |map| {
                    if let Some(mut mob) = map.mobs.remove(&reader.mob_id) {
                        mob.model.pos_x = reader.origin_x;
                        mob.model.pos_y = reader.origin_y;
                        mob.life.fh = reader.fh;
                        mob.model.last_x = reader.last_x;
                        mob.model.last_y = reader.last_y;
                        map.mobs.insert(reader.mob_id, mob);
                    }
                })
                .await?;
        }
        std::hint::black_box(session);
        Ok(Self {
            mob_id: reader.mob_id,
            skill0: reader.skill0,
            skill1: reader.skill1,
            skill2: reader.skill2,
            skill3: reader.skill3,
            skill4: reader.skill4,
            skillb: reader.skillb,
            origin: Point {
                x: reader.origin_x,
                y: reader.origin_y,
            },
            next: Point {
                x: reader.next_x,
                y: reader.next_y,
            },
            last: Point {
                x: reader.last_x,
                y: reader.last_y,
            },
            command: reader.command,
            fh: reader.fh,
            new_state: reader.new_state,
            duration: reader.duration,
        })
    }
}
