/* session/error.rs
 * The purpose of this module is to provide errors related to sessions.
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

use crate::models::error::ModelError;

#[derive(Debug, Error)]
pub enum SessionError {
    #[error("Failed to locate session: {0}")]
    NotFound(i32),

    #[error("Failed to locate the session HashSet")]
    NoSessions,

    #[error("Failed to retrieve account in session layer: {0}")]
    NoAccount(i32),

    #[error("Failed to retrieve channel in session layer: {0}")]
    NoChannel(i32),

    #[error("Failed to retrieve world in session layer: {0}")]
    NoWorld(i32),

    #[error("Failed to retrieve hardware id in session layer: {0}")]
    NoHwid(i32),

    #[error("Failed to validate successful authentication in session layer: {0}")]
    NotAuthenticated(i32),

    #[error("Failed to retrieve character in session layer: {0}")]
    NoChar(i32),

    #[error("Failed to retrieve map in session layer: {0}")]
    NoMap(i32),

    #[error("Model error in session layer: {0}")]
    ModelError(#[from] ModelError),

    #[error("Failed to retrieve map lock in session layer")]
    MapLockError,
}
