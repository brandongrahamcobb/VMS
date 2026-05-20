/* pickup_item/error.rs
 * The purpose of this module is to provide errors related to picking up items.
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

use crate::models::character::error::CharacterError;
use crate::models::item::error::ItemError;
use crate::net::packet::io::error::IOError;
use crate::runtime::session::error::SessionError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PickupItemError {
    #[error("Packet io error in pickup item layer")]
    IOError(#[from] IOError),

    #[error("Session error in pickup item layer")]
    SessionError(#[from] SessionError),

    #[error("Character model error in pickup item layer")]
    CharacterError(#[from] CharacterError),

    #[error("Item model error in pickup item layer")]
    ItemError(#[from] ItemError),
}
