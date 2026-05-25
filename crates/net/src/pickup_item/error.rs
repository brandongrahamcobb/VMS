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

use assembly::character::error::CharacterAssemblyError;
use assembly::item::error::ItemAssemblyError;
use domain::error::DomainError;
use entity::item::error::ItemEntityError;
use packet::build::error::PacketBuildError;
use packet::io::error::IOError;
use session::error::SessionError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PickupItemEntityError {
    #[error("Packet io error in pickup item layer")]
    IOError(#[from] IOError),

    #[error("Session error in pickup item layer")]
    SessionError(#[from] SessionError),

    #[error("Character assembly error in pickup item layer")]
    CharacterAssemblyError(#[from] CharacterAssemblyError),

    #[error("Item entity error in pickup item layer")]
    ItemEntityError(#[from] ItemEntityError),

    #[error("Item assembly error in pickup item layer")]
    ItemAssemblyError(#[from] ItemAssemblyError),

    #[error("Domain error in pickup item layer")]
    DomainError(#[from] DomainError),

    #[error("Packet build error in pickup item layer")]
    PacketBuildError(#[from] PacketBuildError),
}
