/* item/wrapper.rs
 * The purpose of this module is to provide an item wrapper.
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
use crate::item::error::ItemEntityError;
use crate::item::model::{ItemModel, ItemWzInfo};
use std::collections::HashMap;

pub struct Inventory {
    pub equipped_tab: HashMap<i16, Vec<Item>>,
    pub equip_tab: HashMap<i16, Vec<Item>>,
    pub use_tab: HashMap<i16, Vec<Item>>,
    pub setup_tab: HashMap<i16, Vec<Item>>,
    pub etc_tab: HashMap<i16, Vec<Item>>,
    pub cash_tab: HashMap<i16, Vec<Item>>,
}

pub enum EquipSlot {
    Android,
    Cash,
    Pet,
    Regular,
}

pub struct Item {
    pub model: ItemModel,
    pub info: ItemWzInfo,
}

impl Item {
    pub fn get_ipos(&self) -> Result<i16, ItemEntityError> {
        self.model.get_ipos()
    }
}
