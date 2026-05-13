/* list_chars/builder.rs
 * The purpose of this module is to build an outgoing character list packet.
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

use crate::models::character::wrapper::Character;
use crate::models::item;
use crate::models::item::inventory::model::{CashEquipType, RegularEquipType};
use crate::models::item::inventory::wrapper::InventoryItem;
use crate::net::error::NetworkError;
use crate::net::packet::io::error::IOError::WriteError;
use crate::net::packet::model::Packet;
use crate::op::send::SendOpcode;
use crate::{prelude::*, wz};

impl Packet {
    pub fn build_list_chars_packet(
        &mut self,
        chars: Vec<Character>,
        channel_id: i16,
        char_slots: i16,
        pic_status: i16,
    ) -> Result<&mut Self, NetworkError> {
        let op = SendOpcode::CharList as i16;
        self.write_short(op).map_err(WriteError)?;
        self.write_byte(channel_id).map_err(WriteError)?;
        self.write_byte(chars.len() as i16).map_err(WriteError)?;
        for char in chars {
            self.build_look_part_packet(char.clone())?;
        }
        self.write_byte(pic_status).map_err(WriteError)?;
        self.write_int(char_slots as i32).map_err(WriteError)?;
        Ok(self)
    }

    fn build_look_regular_equip_meta_part_packet(
        &mut self,
        equip: InventoryItem,
    ) -> Result<&mut Self, NetworkError> {
        self.write_int(equip.model.wz).map_err(WriteError)?;
        Ok(self)
    }

    fn build_look_cash_equip_meta_part_packet(
        &mut self,
        equip: InventoryItem,
    ) -> Result<&mut Self, NetworkError> {
        self.write_int(equip.model.wz).map_err(WriteError)?;
        Ok(self)
    }

    fn build_look_regular_part_packet(
        &mut self,
        equip: InventoryItem,
        regular_equip_type: RegularEquipType,
    ) -> Result<&mut Self, NetworkError> {
        self.write_byte(equip_type as i16).map_err(WriteError)?;
        self.build_look_regular_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_look_cash_part_packet(
        &mut self,
        equip: InventoryItem,
        cash_equip_type: CashEquipType,
    ) -> Result<&mut Self, NetworkError> {
        self.write_byte(equip_Type as i16).map_err(WriteError)?;
        self.build_look_cash_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    pub fn build_look_regular_equipment_part_packet(
        &mut self,
        char: Character,
    ) -> Result<&mut Self, NetworkError> {
        for item in char.items {
            if item.model.equipped {
                let equip_type: EquipType =
                    item::inventory::service::get_equip_type_from_wz(item.model.wz)?;
                self.build_look_regular_part_packet(item, equip_type)?;
            }
        }
        Ok(self)
    }

    pub fn build_look_cash_equipment_part_packet(
        &mut self,
        char: Character,
    ) -> Result<&mut Self, NetworkError> {
        for item in char.items {
            if item.model.equipped {
                let equip_type: EquipType =
                    item::inventory::service::get_equip_type_from_wz(item.model.wz)?;
                self.build_look_cash_part_packet(item.clone(), equip_type)?;
            }
        }
        Ok(self)
    }

    fn build_look_part_packet(&mut self, char: Character) -> Result<&mut Self, NetworkError> {
        self.build_list_char_meta_part_packet(char.clone())?;
        self.build_look_meta_part_packet(char.clone())?;
        self.write_byte(0).map_err(WriteError)?;
        // Disable rank.
        self.write_byte(0).map_err(WriteError)?;
        Ok(self)
    }

    pub fn build_list_char_meta_part_packet(
        &mut self,
        char: Character,
    ) -> Result<&mut Self, NetworkError> {
        self.write_int(char.model.get_id()?).map_err(WriteError)?;
        self.write_str(char.model.ign.clone()).map_err(WriteError)?;
        self.write_bytes(vec![0u8; 13 - char.model.ign.len()])
            .map_err(WriteError)?;
        self.write_byte(char.model.gender_wz as i16)
            .map_err(WriteError)?;
        self.write_byte(char.model.skin_wz as i16)
            .map_err(WriteError)?;
        self.write_int(char.model.face_wz).map_err(WriteError)?;
        self.write_int(char.model.hair_wz).map_err(WriteError)?;
        // Pets... Not implemented yet
        self.write_long(0).map_err(WriteError)?;
        self.write_long(0).map_err(WriteError)?;
        self.write_long(0).map_err(WriteError)?;
        self.write_byte(char.model.level as i16)
            .map_err(WriteError)?;
        self.write_short(char.model.job_wz).map_err(WriteError)?;
        self.write_short(char.model.strength).map_err(WriteError)?;
        self.write_short(char.model.dexterity).map_err(WriteError)?;
        self.write_short(char.model.intelligence)
            .map_err(WriteError)?;
        self.write_short(char.model.luck).map_err(WriteError)?;
        self.write_short(char.model.hp).map_err(WriteError)?;
        self.write_short(char.model.max_hp).map_err(WriteError)?;
        self.write_short(char.model.mp).map_err(WriteError)?;
        self.write_short(char.model.max_mp).map_err(WriteError)?;
        self.write_short(char.model.ap).map_err(WriteError)?;
        // SP
        self.write_short(0).map_err(WriteError)?;
        self.write_int(char.model.exp).map_err(WriteError)?;
        self.write_short(char.model.fame).map_err(WriteError)?;
        // Gach xp?
        self.write_int(0).map_err(WriteError)?;
        self.write_int(char.model.map_wz).map_err(WriteError)?;
        self.write_byte(0).map_err(WriteError)?;
        self.write_int(0).map_err(WriteError)?;
        Ok(self)
    }

    pub fn build_look_meta_part_packet(
        &mut self,
        char: Character,
    ) -> Result<&mut Self, NetworkError> {
        self.write_byte(char.model.gender_wz as i16)
            .map_err(WriteError)?;
        self.write_byte(char.model.skin_wz as i16)
            .map_err(WriteError)?;
        self.write_int(char.model.face_wz).map_err(WriteError)?;
        self.write_byte(0) // megaphone
            .map_err(WriteError)?;
        self.write_int(char.model.hair_wz).map_err(WriteError)?;
        self.build_look_regular_equipment_part_packet(char.clone())?;
        self.write_byte(0xFF).map_err(WriteError)?;
        self.build_look_cash_equipment_part_packet(char.clone())?;
        self.write_byte(0xFF).map_err(WriteError)?;
        self.write_int(0) //maskedequips -111
            .map_err(WriteError)?;
        // Pet stuff...
        self.write_int(0).map_err(WriteError)?;
        self.write_int(0).map_err(WriteError)?;
        self.write_int(0).map_err(WriteError)?;
        Ok(self)
    }
}
