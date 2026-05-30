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

use core::convert::From;

use bevy::prelude::Message;
use ipc::data::take_damage::TakeDamageCommand;

#[derive(Message)]
pub struct TakeDamageMessage {
    pub client_id: i32,
    pub from: i16,
    pub element: i16,
    pub damage: i32,
    pub mob_wz: i32,
    pub mob_id: i32,
    pub direction: i16,
}

impl From<(TakeDamageMessage, i32)> for TakeDamageCommand {
    fn from((msg, char_id): (TakeDamageMessage, i32)) -> Self {
        Self {
            client_id: msg.client_id,
            char_id,
            from: msg.from,
            element: msg.element,
            damage: msg.damage,
            mob_wz: msg.mob_wz,
            mob_id: msg.mob_id,
            direction: msg.direction,
        }
    }
}
