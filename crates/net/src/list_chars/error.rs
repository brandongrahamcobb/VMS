/* list_chars/error.rs
 * The purpose of this module is to provide errors related to listing characters.
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

use assembly::account::error::AccountAssemblyError;
use config::error::ConfigError;
use entity::account::error::AccountEntityError;
use entity::character::error::CharacterEntityError;
use packet::build::error::PacketBuildError;
use packet::io::error::IOError;
use session::error::SessionError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ListCharsError {
    #[error("Packet io error in list characters layer")]
    IOError(#[from] IOError),

    #[error("Session error in list characters layer")]
    SessionError(#[from] SessionError),

    #[error("Configuration error in list characters layer")]
    ConfigError(#[from] ConfigError),

    #[error("Character entity error in list characters layer")]
    CharacterEntityError(#[from] CharacterEntityError),

    #[error("Account entity error in list characters layer")]
    AccountEntityError(#[from] AccountEntityError),

    #[error("Account assembly error in list characters layer")]
    AccountAssemblyError(#[from] AccountAssemblyError),

    #[error("Packet build error in list characters layer")]
    PacketBuildError(#[from] PacketBuildError),
}
