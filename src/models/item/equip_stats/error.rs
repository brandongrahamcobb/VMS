/* equip_stats/error.rs
 * The purpose of this module is to provide errors related to equip statistics.
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

#[derive(Debug, thiserror::Error)]
pub enum EquipError {
    #[error("Requested equip resource was not found in equip model layer: {0}")]
    NotFound(i32),

    #[error("Wz read error in wz model layer")]
    ReadError(#[from] anyhow::Error),

    #[error("Wz from reader error in wz model layer")]
    PropertyError(#[from] binrw::error::Error),

    #[error("Wz version error in wz model layer")]
    NoVersion,

    #[error("Wz Base.wz read error in wz model layer")]
    FileNotFound(#[from] std::io::Error),

    #[error("Database error in equip model layer")]
    DatabaseError(#[from] DatabaseError),
}
