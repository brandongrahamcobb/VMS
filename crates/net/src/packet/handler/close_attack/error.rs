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

use assembly::character::error::CharacterAssemblyError;
use config::error::ConfigError;
use db::error::DatabaseError;
use domain::error::DomainError;
use entity::character::error::CharacterEntityError;
use entity::item::error::ItemEntityError;
use packet::build::error::PacketBuildError;
use packet::io::error::IOError;
use session::error::SessionError;
use state::error::StateError;
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

    #[error("Item entity error in close attack layer")]
    ItemEntityError(#[from] ItemEntityError),

    #[error("Character entity error in close attack layer")]
    CharacterEntityError(#[from] CharacterEntityError),

    #[error("Character assembly error in close attack layer")]
    CharacterAssemblyError(#[from] CharacterAssemblyError),

    #[error("Packet build error in close attack layer")]
    PacketBuildError(#[from] PacketBuildError),

    #[error("Configuration error in close attack layer")]
    ConfigErorr(#[from] ConfigError),

    #[error("Domain error in close attack layer")]
    DomainError(#[from] DomainError),
}
