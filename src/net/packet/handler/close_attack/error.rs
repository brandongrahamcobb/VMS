/* close_attack/error.rs
 * The purpose of this module is to provide errors related to close attacks.
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

use crate::db::error::DatabaseError;
use crate::models::character::error::CharacterError;
use crate::models::item::error::ItemError;
use crate::models::mob::error::MobError;
use crate::net::packet::codec::item::error::CodecItemError;
use crate::net::packet::codec::mob::error::CodecMobError;
use crate::net::packet::codec::player::error::CodecPlayerError;
use crate::net::packet::io::error::IOError;
use crate::runtime::error::StateError;
use crate::runtime::session::error::SessionError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CloseAttackError {
    #[error("Packet io error in close attack layer")]
    IOError(#[from] IOError),

    #[error("Session error in close attack layer")]
    SessionError(#[from] SessionError),

    #[error("Database error in close attack layer")]
    DatabaseError(#[from] DatabaseError),

    #[error("State error in close attack layer")]
    StateError(#[from] StateError),

    #[error("Mob model error in close attack layer")]
    MobError(#[from] MobError),

    #[error("Codec mob packet error in close attack layer")]
    CodecMobError(#[from] CodecMobError),

    #[error("Codec playr packet error in close attack layer")]
    CodecPlayerError(#[from] CodecPlayerError),

    #[error("Item model error in close attack layer")]
    ItemError(#[from] ItemError),

    #[error("Character model error in close attack layer")]
    CharacterError(#[from] CharacterError),

    #[error("Codec item packet error in close attack layer")]
    CodecItemError(#[from] CodecItemError),
}
