/* app/src/system/handler/codec/load_char.rs
 * The purpose of this module is to process character loading.
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

use std::collections::HashMap;

use bevy::ecs::entity::Entity;
use bevy::ecs::hierarchy::ChildOf;
use bevy::ecs::system::Query;

use crate::component::character::MapleCharacter;
use crate::component::item::MapleItem;
use crate::system::system_params::{InParams, InventoryParams, SessionParams};

pub fn load_char_with_equips(
    client_entities: Vec<Entity>,
    in_params: &InParams,
    session_params: &SessionParams,
    inv_params: &InventoryParams,
    items: Query<(&MapleItem, &ChildOf)>,
) -> HashMap<MapleCharacter, Vec<MapleItem>> {
    let mut char_map: HashMap<MapleCharacter, Vec<MapleItem>> = HashMap::new();
    for client_entity in client_entities {
        let Ok(in_char) = in_params.in_chars.get(client_entity) else {
            continue;
        };
        let Ok((char_entity, char, _)) = session_params.chars.get(in_char.0) else {
            continue;
        };
        let Some((inv_entity, _, _)) = inv_params
            .inventories
            .iter()
            .find(|(_, _, parent)| parent.0 == char_entity)
        else {
            continue;
        };
        let Some((equipped_tab_entity, _, _)) = inv_params
            .equipped_tabs
            .iter()
            .find(|(_, _, parent)| parent.0 == inv_entity)
        else {
            continue;
        };
        let filled_item_slots: Vec<_> = inv_params
            .filled_slots
            .iter()
            .filter(|(_, _, parent)| parent.0 == equipped_tab_entity)
            .collect();
        let mut equips: Vec<MapleItem> = Vec::new();
        for (filled_item_slot_entity, _, _) in filled_item_slots {
            let Some((equip, _)) = items
                .iter()
                .find(|(_, parent)| parent.0 == filled_item_slot_entity)
            else {
                continue;
            };
            equips.push(equip.clone());
        }
        char_map.insert(char.clone(), equips);
    }
    char_map
}
