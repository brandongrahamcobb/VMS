/* account/error.rs
 * The purpose of this module is to provide errors related to accounts.
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

use crate::character::error::CharacterEntityError;
use bcrypt::BcryptError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AccountEntityError {
    #[error("No pic found in account entity layer: {0}")]
    NoPic(i32),

    #[error("No id found in account entity layer")]
    NoId,

    #[error("No created at time found in account entity layer: {0}")]
    NoCreatedAt(i32),

    #[error("Bcrypt error in account entity layer")]
    CryptError(#[from] BcryptError),

    #[error("Character entity error in account entity layer")]
    CharacterEntityError(#[from] CharacterEntityError),
}
