/* create_char/message.rs
 * The purpose of this module is to handle character creation.
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
use ipc::data::create_char::CreateCharCommand;

#[derive(Message)]
pub struct CreateCharMessage {
    pub client_id: i32,
    pub ign: String,
    pub job_wz: i16,
    pub face_wz: i32,
    pub hair_wz: i32,
    pub hair_color_wz: i32,
    pub skin_wz: i32,
    pub top_wz: i32,
    pub bottom_wz: i32,
    pub shoes_wz: i32,
    pub weapon_wz: i32,
    pub gender_wz: i16,
}

impl From<(CreateCharMessage, i32, i16)> for CreateCharCommand {
    fn from((msg, acc_id, world_id): (CreateCharMessage, i32, i16)) -> Self {
        Self {
            client_id: msg.client_id,
            acc_id,
            world_id,
            ign: msg.ign,
            job_wz: msg.job_wz,
            face_wz: msg.face_wz,
            hair_wz: msg.hair_wz,
            hair_color_wz: msg.hair_color_wz,
            skin_wz: msg.skin_wz,
            top_wz: msg.top_wz,
            bottom_wz: msg.bottom_wz,
            shoes_wz: msg.shoes_wz,
            weapon_wz: msg.weapon_wz,
            gender_wz: msg.gender_wz,
        }
    }
}
