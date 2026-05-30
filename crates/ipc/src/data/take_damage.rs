/* close_attack/store.rs
 * The purpose of this module is to resolve relevant variables for close attacks.
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

use crate::take_damage::error::TakeDamageError;
use crate::take_damage::reader::TakeDamageReader;
use core::cmp::Ordering;
use db::pool::DbPool;
use entity;
use entity::character::wrapper::Character;
use session::model::Session;

pub struct TakeDamageEvent {
    pub char: Character,
    pub hp: i16,
    pub max_hp: i16,
    pub return_map_wz: i32,
}

pub struct TakeDamageCommand {
    pub client_id: i32,
    pub char_id: i32,
    pub from: i16,
    pub element: i16,
    pub damage: i32,
    pub mob_wz: i32,
    pub mob_id: i32,
    pub direction: i16,
}

impl TakeDamageEvent {
    pub async fn store_take_damage(
        pool: &DbPool,
        session: &Session,
        reader: &TakeDamageReader,
    ) -> Result<Self, TakeDamageError> {
        let char_id: i32 = session.get_char_id()?;
        let map_wz: i32 = session.get_map_wz()?;
        let return_map_wz: i32 = metadata::map::death::get_death_map_by_wz(map_wz)?;
        let mut char: Character =
            assembly::character::assemble::assemble_char_by_id(pool, char_id).await?;
        let max_hp = char.model.max_hp;
        let calc: i16 = char.model.hp - reader.damage as i16;
        let hp = match calc.cmp(&0) {
            Ordering::Greater | Ordering::Equal => calc,
            _ => 0,
        };
        if hp != 0 {
            char.model.hp = hp;
            db::character::setters::update_characters(pool, vec![char.model.clone()]).await?;
        } else {
            char.model.hp = max_hp;
            db::character::setters::update_characters(pool, vec![char.model.clone()]).await?;
        }
        Ok(Self {
            char,
            hp,
            max_hp,
            return_map_wz,
        })
    }
}
