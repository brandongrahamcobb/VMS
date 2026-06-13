/* app/src/message/packet/mob_moved.rs
 * The purpose of this module is to serve mob movement packet system messages.
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

use bevy::prelude::Message;

#[derive(Message)]
pub struct ReadMobMovedRequestMessage {
    pub client_id: i32,
    pub mob_id: u32,
    pub t: i16,
    pub skillb: u8,
    pub skill0: u8,
    pub skill1: u8,
    pub skill2: u8,
    pub skill3: u8,
    pub skill4: u8,
    pub origin_x: i16,
    pub origin_y: i16,
    pub command: u8,
    pub next_x: i16,
    pub next_y: i16,
    pub last_x: i16,
    pub last_y: i16,
    pub fh: u16,
    pub new_state: u8,
    pub duration: i16,
}
