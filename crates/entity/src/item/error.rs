/* item/error.rs
 * The purpose of this module is to provide errors related to items.
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

use config::error::ConfigError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ItemEntityError {
    #[error("No item id found in item entity layer")]
    NoId,

    #[error("No item position found in item entity layer")]
    NoPos,

    #[error("Tab error in item entity layer")]
    TabError,

    #[error("No islot found in item entity layer")]
    InvalidISlot,

    #[error("No inventory mod item model found in item entity layer")]
    NoItemModel,

    #[error("No inventory mod item info found in item entity layer")]
    NoItemInfo,

    #[error("Inventory full error in item entity layer")]
    InventoryFull,

    #[error("Item not found error in item entity layer")]
    ItemNotFound,

    #[error("Configuration error in item entity layer")]
    ConfigError(#[from] ConfigError),
}
