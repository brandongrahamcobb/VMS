/* entity/src/item/service.rs
 * The purpose of this module is to provide associated funtions with for items.
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
use crate::item::model::InventoryTab;
use crate::item::wrapper::Inventory;

pub fn next_free_pos(inv: &Inventory, tab: &InventoryTab) -> Result<i16, ItemEntityError> {
    let tab = match tab {
        InventoryTab::Equip => &inv.equip_tab,
        InventoryTab::Use => &inv.use_tab,
        InventoryTab::Setup => &inv.setup_tab,
        InventoryTab::Etc => &inv.etc_tab,
        InventoryTab::Cash => &inv.cash_tab,
    };
    if let Some(pos) = (1..=96).find(|pos| !tab.contains_key(pos)) {
        Ok(pos)
    } else {
        Err(ItemEntityError::InventoryFull)
    }
}
