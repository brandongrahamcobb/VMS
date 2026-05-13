/* wz/error.rs
 * The purpose of this module is to provide errors related to Wz metadata.
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

use crate::models::item::equip_stats::error::EquipError;

#[derive(Debug, thiserror::Error)]
pub enum WzError {
    #[error("Requested Wz resource was not found in wz model layer: {0}")]
    NotFound(i32),

    #[error("Wz anyhow error in wz model layer")]
    AnyHowError(#[from] anyhow::Error),

    #[error("Wz binrw error in wz model layer")]
    BinRwError(#[from] binrw::error::Error),

    #[error("Wz version error in wz model layer")]
    NoVersion,

    #[error("Equip error in wz model layer")]
    EquipError(#[from] EquipError),

    #[error("WzDirEntry error in wz model layer")]
    EntryError,

    #[error("Wz part error in wz model layer")]
    PartError,

    #[error("Wz Object error in wz model layer")]
    ObjectError,
}
