/* app/src/component/item.rs
 * The purpose of this module is to provide an item component.
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

use core::convert::From;
use std::time::Instant;

use base::item::BaseItem;
use bevy::ecs::component::Component;
use db::item::model::ItemModel;

#[derive(Clone, Component)]
pub struct MapleItem {
    pub id: i32,
    pub char_id: Option<i32>,
    pub ipos: Option<i16>,
    pub strength: i16,
    pub dexterity: i16,
    pub intelligence: i16,
    pub luck: i16,
    pub attack: i16,
    pub weapon_defense: i16,
    pub magic: i16,
    pub magic_defense: i16,
    pub hp: i16,
    pub mp: i16,
    pub accuracy: i16,
    pub avoid: i16,
    pub hands: i16,
    pub speed: i16,
    pub jump: i16,
    pub expire: i64,
    pub level: i16,
    pub item_level: i16,
    pub flag: i16,
    pub item_exp: i16,
    pub vicious: i32,
    pub base: BaseItem,
}

#[derive(Component)]
pub struct Lootable {
    pub dropped_at: Instant,
}

impl From<(BaseItem, ItemModel)> for MapleItem {
    fn from((base, model): (BaseItem, ItemModel)) -> Self {
        let id = if let Some(id) = model.id { id } else { 0 };
        Self {
            id: id,
            char_id: model.char_id,
            ipos: model.ipos,
            strength: model.strength,
            dexterity: model.dexterity,
            intelligence: model.intelligence,
            luck: model.luck,
            attack: model.attack,
            weapon_defense: model.weapon_defense,
            magic: model.magic,
            magic_defense: model.magic_defense,
            hp: model.hp,
            mp: model.mp,
            accuracy: model.accuracy,
            avoid: model.avoid,
            hands: model.hands,
            speed: model.speed,
            jump: model.jump,
            expire: model.expire,
            level: model.level,
            item_level: model.item_level,
            flag: model.flag,
            item_exp: model.item_exp,
            vicious: model.vicious,
            base,
        }
    }
}
