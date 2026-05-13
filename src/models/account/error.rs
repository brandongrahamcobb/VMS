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

use thiserror::Error;

#[derive(Debug, Error)]
pub enum AccountError {
    #[error("Requested account was not found in account model layer: {0}")]
    NotFound(i16),

    #[error("Missing field in account model layer: {0}")]
    MissingField(i32),

    #[error("No pic found in account model layer: {0}")]
    NoPic(i32),

    #[error("No id found in account model layer")]
    NoId,

    #[error("No channel id found in account model layer")]
    NoChannelId,

    #[error("No character id found in account model layer")]
    NoCharId,

    #[error("No map wz found in account model layer")]
    NoMapWz,

    #[error("No world id found in account model layer")]
    NoWorldId,

    #[error("No created at time found in account model layer: {0}")]
    NoCreatedAt(i32),
}
