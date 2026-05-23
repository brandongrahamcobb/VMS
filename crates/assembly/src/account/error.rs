/* assembly/src/account/error.rs
 * The purpose of this module is to provide errors related to account assembly.
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

use crate::character::error::CharacterAssemblyError;
use db::error::DatabaseError;
use entity::account::error::AccountEntityError;
use entity::character::error::CharacterEntityError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AccountAssemblyError {
    #[error("Character assembly error in account assembly layer")]
    CharacterAssemblyError(#[from] CharacterAssemblyError),

    #[error("Database error in account assembly layer")]
    DatabaseError(#[from] DatabaseError),

    #[error("Character entity error in account assembly layer")]
    CharacterEntityError(#[from] CharacterEntityError),

    #[error("Account entity error in account assembly layer")]
    AccountEntityError(#[from] AccountEntityError),
}
