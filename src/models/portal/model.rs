/* portal/model.rs
 * The purpose of this module is to provide a portal model.
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

use crate::models::error::ModelError;
use crate::models::portal::wrapper::Portal;

#[derive(Clone)]
pub struct PortalModel {
    pub pid: i16,
    pub pn: String,
    pub tm: i32,
    pub tn: String,
}

impl PortalModel {
    pub fn load(&self) -> Result<Portal, ModelError> {
        Ok(Portal {
            model: self.clone(),
        })
    }
}
