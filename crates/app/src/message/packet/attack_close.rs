/* close_attack/message.rs
 * The purpose of this module is to handle close attacks.
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
use ipc::data::attack_close::CloseAttackCommand;
use std::collections::HashMap;

#[derive(Message)]
pub struct CloseAttackMessage {
    pub client_id: i32,
    pub count: i16,
    pub skill_id: i32,
    pub display: i16,
    pub toleft: i16,
    pub stance: i16,
    pub speed: i16,
    pub mob_damages: HashMap<u32, Vec<i32>>,
}
