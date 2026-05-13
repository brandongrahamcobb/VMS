/* character/error.rs
 * The purpose of this module is to provide errors related to characters.
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

use thiserror::Error;

#[derive(Debug, Error)]
pub enum CharacterError {
    #[error("Requested character was not found in character model layer: {0}")]
    NotFound(i16),

    #[error("No character is selected for account in character model layer: {0}")]
    NotSelected(i32),

    #[error("Missing field in character model layer: {0}")]
    MissingField(i32),

    #[error("No id found in character model layer")]
    NoId,
}
