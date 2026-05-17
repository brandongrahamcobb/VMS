/* move_mob/store.rs
 * The purpose of this module is to resolve relevant variables for mob movement.
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

use crate::net::packet::handler::move_mob::error::MoveMobError;
use crate::net::packet::handler::move_mob::reader::MoveMobReader;
use crate::runtime::session::model::Session;
use crate::runtime::state::SharedState;

pub struct MoveMobStore {
    pub mob_id: u32,
    pub skillb: u8,
    pub skill0: u8,
    pub skill1: u8,
    pub skill2: u8,
    pub skill3: u8,
    pub skill4: u8,
    pub pos_x: i16,
    pub pos_y: i16,
    pub command: u8,
    pub x: i16,
    pub y: i16,
    pub last_x: i16,
    pub last_y: i16,
    pub fh: i16,
    pub new_state: u8,
    pub duration: i16,
}

impl MoveMobStore {
    pub async fn store_move_mob(
        state: &SharedState,
        session: &Session,
        reader: &MoveMobReader,
    ) -> Result<Self, MoveMobError> {
        std::hint::black_box(state);
        std::hint::black_box(session);
        Ok(Self {
            mob_id: reader.mob_id,
            skill0: reader.skill0,
            skill1: reader.skill1,
            skill2: reader.skill2,
            skill3: reader.skill3,
            skill4: reader.skill4,
            skillb: reader.skillb,
            pos_x: reader.pos_x,
            pos_y: reader.pos_y,
            command: reader.command,
            x: reader.x,
            y: reader.y,
            last_x: reader.last_x,
            last_y: reader.last_y,
            fh: reader.fh,
            new_state: reader.new_state,
            duration: reader.duration,
        })
    }
}
