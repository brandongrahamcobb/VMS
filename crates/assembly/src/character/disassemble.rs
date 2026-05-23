/* assembly/src/character/disassemble.rs
 * The purpose of this module is to deconstruct a character.
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

use db;
use db::pool::DbPool;
use entity::character::wrapper::Character;

use crate::character::error::CharacterAssemblyError;

pub async fn disassemble_character(
    pool: &DbPool,
    char: &Character,
) -> Result<(), CharacterAssemblyError> {
    let char_id: i32 = char.model.get_id()?;
    db::character::setters::delete_char_by_id(pool, char_id).await?;
    for (_, items) in &char.inventory.equipped_tab {
        for item in items {
            db::item::setters::delete_item_by_id(pool, item.model.get_id()?).await?;
        }
    }
    for (_, items) in &char.inventory.equip_tab {
        for item in items {
            db::item::setters::delete_item_by_id(pool, item.model.get_id()?).await?;
        }
    }
    for (_, items) in &char.inventory.cash_tab {
        for item in items {
            db::item::setters::delete_item_by_id(pool, item.model.get_id()?).await?;
        }
    }
    for (_, items) in &char.inventory.setup_tab {
        for item in items {
            db::item::setters::delete_item_by_id(pool, item.model.get_id()?).await?;
        }
    }
    for (_, items) in &char.inventory.use_tab {
        for item in items {
            db::item::setters::delete_item_by_id(pool, item.model.get_id()?).await?;
        }
    }
    for (_, items) in &char.inventory.etc_tab {
        for item in items {
            db::item::setters::delete_item_by_id(pool, item.model.get_id()?).await?;
        }
    }
    for (keybinding_id, _) in &char.binds {
        db::keybinding::setters::delete_keybinding_by_id(pool, *keybinding_id).await?;
    }
    for (skill_id, _) in &char.skills {
        db::skill::setters::delete_skill_by_id(pool, *skill_id).await?;
    }
    Ok(())
}
