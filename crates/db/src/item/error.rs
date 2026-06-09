/* db/src/item/error.rs
 * The purpose of this module is to provide errors related to db items.
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
pub enum ItemModelError {
    #[error("No item id found for item model in database layer")]
    NoId,

    #[error("No item position found for item model in database layer: {0}")]
    NoPos(i32),

    #[error("No owner found for item model in database layer: {0}")]
    NoCharId(i32),

    #[error("No created at time found for item model in database layer: {0}")]
    NoCreatedAt(i32),
}
