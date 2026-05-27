/* domain/src/item.rs
 * The purpose of this module is to provide domain logic for items.
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

use config::settings;
use db;
use db::pool::DbPool;
use entity;
use entity::item::model::{
    DropData, InventoryMod, InventoryModMode, InventoryTab, ItemModel, ItemWzInfo,
};
use entity::item::wrapper::{Inventory, Item};
use entity::mob::model::MobWzLife;
use metadata;
use rand::RngExt;

use crate::error::DomainError;

pub async fn equip(pool: &DbPool, inv: &mut Inventory, item_id: i32) -> Result<(), DomainError> {
    let item_pos = inv
        .equip_tab
        .iter()
        .find_map(|(pos, stack)| {
            stack.iter().find_map(|item| {
                item.model
                    .get_id()
                    .ok()
                    .filter(|id| *id == item_id)
                    .map(|_| *pos)
            })
        })
        .ok_or(DomainError::ItemError)?;
    let mut stack = inv
        .equip_tab
        .remove(&item_pos)
        .ok_or(DomainError::ItemError)?;
    let item = stack
        .iter_mut()
        .find(|item| item.model.get_id().is_ok_and(|id| id == item_id))
        .ok_or(DomainError::ItemError)?;

    let new_pos = metadata::item::equip::get_equip_ipos_by_wz(item.model.wz)?;
    item.model.ipos = Some(new_pos);
    db::item::setters::update_item(pool, &item.model).await?;
    inv.equipped_tab.insert(new_pos, stack);
    Ok(())
}

pub async fn unequip(pool: &DbPool, inv: &mut Inventory, item_id: i32) -> Result<(), DomainError> {
    let item = inv
        .equipped_tab
        .values()
        .flatten()
        .find(|item| item.model.get_id().is_ok_and(|id| id == item_id))
        .ok_or(DomainError::ItemError)?;
    let itab: InventoryTab = metadata::item::inventory::get_inventory_tab_by_wz(item.model.wz)?;
    let new_pos = entity::item::service::next_free_pos(inv, &itab)?;
    let item = inv
        .equipped_tab
        .values_mut()
        .flatten()
        .find(|item| item.model.get_id().is_ok_and(|id| id == item_id))
        .ok_or(DomainError::ItemError)?;
    item.model.ipos = Some(new_pos);
    db::item::setters::update_item(pool, &item.model).await?;
    Ok(())
}

pub async fn pick_up(
    pool: &DbPool,
    inv: &mut Inventory,
    ign: String,
    char_id: i32,
    mut item: Item,
) -> Result<InventoryMod, DomainError> {
    let item_model: ItemModel = item.model.clone();
    let item_info: ItemWzInfo = item.info.clone();
    let values = [&inv.cash_tab, &inv.etc_tab, &inv.setup_tab, &inv.use_tab];
    let (pos, stack_size, itab) = values
        .iter()
        .find_map(|value| {
            value.iter().find_map(|(pos, inv_item_stack)| {
                if inv_item_stack[0].model.wz == item.model.wz {
                    Some((
                        *pos,
                        inv_item_stack.len(),
                        metadata::item::inventory::get_inventory_tab_by_wz(item.model.wz).ok()?,
                    ))
                } else {
                    None
                }
            })
        })
        .unwrap_or((
            entity::item::service::next_free_pos(
                inv,
                &metadata::item::inventory::get_inventory_tab_by_wz(item.model.wz)?,
            )?,
            0,
            metadata::item::inventory::get_inventory_tab_by_wz(item.model.wz)?,
        ));

    item.model.ipos = Some(pos);
    item.model.char_id = Some(char_id);
    db::item::setters::update_item(pool, &item.model).await?;
    match itab {
        InventoryTab::Equip => {
            inv.equip_tab.insert(pos, vec![item]);
        }
        InventoryTab::Cash => {
            inv.cash_tab.entry(pos).or_default().push(item);
        }
        InventoryTab::Use => {
            inv.use_tab.entry(pos).or_default().push(item);
        }
        InventoryTab::Etc => {
            inv.etc_tab.entry(pos).or_default().push(item);
        }
        InventoryTab::Setup => {
            inv.setup_tab.entry(pos).or_default().push(item);
        }
    }
    let inv_mod: InventoryMod = InventoryMod {
        mode: InventoryModMode::Add,
        inv_type: itab as i8,
        pos,
        count: stack_size as i16 + 1,
        item_model: Some(item_model),
        item_info: Some(item_info),
        char_name: ign,
    };
    Ok(inv_mod)
}

pub fn get_item_from_inventory(inv: &Inventory, item_id: i32) -> Result<&Item, DomainError> {
    let tabs = [
        inv.equip_tab.values(),
        inv.cash_tab.values(),
        inv.use_tab.values(),
        inv.etc_tab.values(),
        inv.setup_tab.values(),
    ];
    tabs.into_iter()
        .flatten()
        .flatten()
        .find(|item| item.model.get_id().is_ok_and(|id| id == item_id))
        .ok_or(DomainError::ItemError)
}

pub async fn get_random_drops(
    pool: &DbPool,
    mob_wz_life: MobWzLife,
) -> Result<Vec<Item>, DomainError> {
    let drop_rate: f64 = settings::get_item_drop_rate()?;
    let drop_data: Vec<DropData> =
        db::item::getters::get_item_drop_data(pool, mob_wz_life.wz).await?;
    let mut items: Vec<Item> = Vec::new();
    for drop_entry in drop_data {
        let chance: f64 = (drop_entry.chance as f64 / 1_000_000.0) * drop_rate;
        let to_drop = if chance > 1.0 {
            true
        } else {
            rand::rng().random_bool(chance)
        };
        if to_drop {
            if chance > 2.0 {
                let multiplier = chance.div_euclid(1.0) as i32;
                for _ in 0..multiplier {
                    let drop_model: ItemModel =
                        metadata::item::inventory::create_item_model_by_wz(drop_entry.item_wz)?;
                    let drop_model: ItemModel =
                        db::item::setters::update_item(pool, &drop_model).await?;
                    let drop: Item =
                        assembly::item::assemble::assemble_item_by_id(pool, drop_model.get_id()?)
                            .await?;
                    items.push(drop);
                }
            } else {
                let drop_model: ItemModel =
                    metadata::item::inventory::create_item_model_by_wz(drop_entry.item_wz)?;
                let drop_model: ItemModel =
                    db::item::setters::update_item(pool, &drop_model).await?;
                let drop: Item =
                    assembly::item::assemble::assemble_item_by_id(pool, drop_model.get_id()?)
                        .await?;
                items.push(drop);
            }
        }
    }
    Ok(items)
}
