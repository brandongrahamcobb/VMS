/* player_logged_in/builder.rs
 * The purpose of this module is to build an outgoing player login packet.
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

use crate::models::channel::wrapper::Channel;
use crate::models::character::wrapper::Character;
use crate::models::item;
use crate::models::item::model::EquipType;
use crate::models::item::wrapper::Item;
use crate::models::keybinding::wrapper::Keybinding;
use crate::net::error::NetworkError;
use crate::net::packet::io::error::IOError::WriteError;
use crate::net::packet::model::Packet;
use crate::op::send::SendOpcode;
use crate::prelude::*;

impl Packet {
    pub fn build_player_logged_in_handler_keymap_packet(
        &mut self,
        binds: Vec<Keybinding>,
    ) -> Result<&mut Self, NetworkError> {
        let op = SendOpcode::KeyMap as i16;
        self.write_short(op).map_err(WriteError)?;
        self.write_byte(0).map_err(WriteError)?;
        for bind in binds {
            let bind_model = bind.model.clone();
            let bind_type = bind_model.bind_type as i16;
            self.write_byte(bind_type).map_err(WriteError)?;
            let bind_action = bind_model.action as i32;
            self.write_int(bind_action).map_err(WriteError)?;
        }
        Ok(self)
    }

    pub fn build_set_field_packet(
        &mut self,
        char: Character,
        channel: Channel,
    ) -> Result<&mut Self, NetworkError> {
        let op = SendOpcode::SetField as i16;
        self.write_short(op).map_err(WriteError)?;
        let channel_id = channel.model.id as i32;
        self.write_int(channel_id).map_err(WriteError)?;
        self //mode 1
            .write_byte(1)
            .map_err(WriteError)?;
        self //mode 2
            .write_byte(2)
            .map_err(WriteError)?;
        // Skip 23 bytes
        let skip = vec![0u8; 23];
        self.write_bytes(skip).map_err(WriteError)?;
        self.write_int(char.model.get_id()?).map_err(WriteError)?;
        self.write_str(char.model.ign.clone()).map_err(WriteError)?;
        self.write_bytes(vec![0u8; 13 - char.model.ign.len()])
            .map_err(WriteError)?;
        let gender_wz = char.model.gender_wz as i16;
        self.write_byte(gender_wz).map_err(WriteError)?;
        let skin_wz = char.model.skin_wz as i16;
        self.write_byte(skin_wz).map_err(WriteError)?;
        self.write_int(char.model.face_wz).map_err(WriteError)?;
        self.write_int(char.model.hair_wz).map_err(WriteError)?;
        // Pets... Not implemented yet
        self.write_long(0).map_err(WriteError)?;
        self.write_long(0).map_err(WriteError)?;
        self.write_long(0).map_err(WriteError)?;
        self.build_player_logged_in_meta_part_packet(char.clone())?;
        Ok(self)
    }

    fn build_inventory_regular_part_packet(
        &mut self,
        equip: Item,
        equip_type: EquipType,
    ) -> Result<&mut Self, NetworkError> {
        let regular_equip_type = match equip_type {
            EquipType::RegularEquipType(regular_equip_type) => regular_equip_type,
            _ => panic!("placeholder"),
        };
        self.write_short(regular_equip_type as i16)
            .map_err(WriteError)?;
        self.build_inventory_regular_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }
    pub fn build_inventory_regular_equipment_part_packet(
        &mut self,
        char: Character,
    ) -> Result<&mut Self, NetworkError> {
        for item in char.items {
            if item.model.equipped {
                let equip_type: EquipType = item::service::get_equip_type_from_wz(item.model.wz)?;
                self.build_inventory_regular_part_packet(item, equip_type)?;
            }
        }
        Ok(self)
    }

    fn build_inventory_regular_equip_meta_part_packet(
        &mut self,
        equip: Item,
    ) -> Result<&mut Self, NetworkError> {
        // Dummy values
        self.write_byte(1).map_err(WriteError)?;
        self.write_int(equip.model.get_id()?).map_err(WriteError)?;
        const NUM_EQUIP_STATS: i16 = 15;
        let is_cash = false as i16;
        self.write_byte(is_cash).map_err(WriteError)?;
        self.write_long(0).map_err(WriteError)?;
        self.write_byte(0).map_err(WriteError)?;
        self.write_byte(0).map_err(WriteError)?;
        for _ in 0..NUM_EQUIP_STATS {
            self.write_short(0).map_err(WriteError)?;
        }
        self.write_str_with_length(String::new())
            .map_err(WriteError)?;
        self.write_short(0).map_err(WriteError)?;
        self.write_byte(0).map_err(WriteError)?;
        self.write_byte(0).map_err(WriteError)?;
        self.write_short(0).map_err(WriteError)?;
        self.write_short(0).map_err(WriteError)?;
        self.write_int(0).map_err(WriteError)?;
        self.write_long(0).map_err(WriteError)?;
        self.write_bytes(vec![0u8; 12]).map_err(WriteError)?;
        Ok(self)
    }

    fn build_inventory_cash_equip_meta_part_packet(
        &mut self,
        equip: Item,
    ) -> Result<&mut Self, NetworkError> {
        self.write_int(equip.model.wz).map_err(WriteError)?;
        Ok(self)
    }

    fn build_inventory_cash_part_packet(
        &mut self,
        equip: Item,
        equip_type: EquipType,
    ) -> Result<&mut Self, NetworkError> {
        let cash_equip_type = match equip_type {
            EquipType::CashEquipType(cash_equip_type) => cash_equip_type,
            _ => panic!("placeholder"),
        };
        self.write_short(cash_equip_type as i16)
            .map_err(WriteError)?;
        self.build_inventory_cash_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    pub fn build_inventory_cash_equipment_part_packet(
        &mut self,
        char: Character,
    ) -> Result<&mut Self, NetworkError> {
        for item in char.items {
            if item.model.equipped {
                let equip_type: EquipType = item::service::get_equip_type_from_wz(item.model.wz)?;
                self.build_inventory_cash_part_packet(item, equip_type)?;
            }
        }
        Ok(self)
    }

    pub fn build_inventory_part_packet(
        &mut self,
        char: Character,
    ) -> Result<&mut Self, NetworkError> {
        self.write_int(char.model.meso).map_err(WriteError)?;
        // Dummy values
        // Inventory slot Capacities
        self.write_bytes(vec![0u8; 5]).map_err(WriteError)?;
        // Time?
        self.write_long(0).map_err(WriteError)?;
        self.build_inventory_regular_equipment_part_packet(char.clone())?;
        self.build_inventory_cash_equipment_part_packet(char.clone())?;
        // End of equipment equipped (all id's) MUST BE ENDED WITH A SHORT 0
        self.write_short(0).map_err(WriteError)?;
        // Start of equipment inventory (negative id's) MUST BE ENDED WITH A SHORT 0
        self.write_short(0).map_err(WriteError)?;
        // Start of equipment inventory (postive id's)  MUST BE ENDED WITH A SHORT 0
        self.write_short(0).map_err(WriteError)?;
        // Skip 2 bytes after equips
        let skip = vec![0u8; 2];
        self.write_bytes(skip).map_err(WriteError)?;
        // Dummy values
        // Start of USE
        self.write_byte(0).map_err(WriteError)?;
        // Start of SETUP
        self.write_byte(0).map_err(WriteError)?;
        // Start of ETC
        self.write_byte(0).map_err(WriteError)?;
        // Start of CASH
        self.write_byte(0).map_err(WriteError)?;
        Ok(self)
    }

    pub fn build_player_logged_in_meta_part_packet(
        &mut self,
        char: Character,
    ) -> Result<&mut Self, NetworkError> {
        let level = char.model.level as i16;
        self.write_byte(level).map_err(WriteError)?;
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
        let bl_capacity = 25;
        self.write_byte(bl_capacity).map_err(WriteError)?;
        self.write_byte(0).map_err(WriteError)?;
        self.build_inventory_part_packet(char.clone())?
            .build_skills_part_packet()?
            .build_quests_part_packet()?
            .build_minigames_part_packet()?
            .build_rings_part_packet()?
            .build_teleport_part_packet()?
            .build_codex_part_packet()?
            .build_new_year_cards_part_packet()?
            .build_area_info_part_packet()?;
        Ok(self)
    }

    fn build_skills_part_packet(&mut self) -> Result<&mut Self, NetworkError> {
        // Dummy values
        // No skills!
        self.write_short(0).map_err(WriteError)?;
        // No no cooldowns!
        self.write_short(0).map_err(WriteError)?;
        Ok(self)
    }

    fn build_quests_part_packet(&mut self) -> Result<&mut Self, NetworkError> {
        // Dummy values
        let started_quests = 0;
        self.write_short(started_quests).map_err(WriteError)?;
        let completed_quests = 0;
        self.write_short(completed_quests).map_err(WriteError)?;
        Ok(self)
    }

    fn build_minigames_part_packet(&mut self) -> Result<&mut Self, NetworkError> {
        // Dummy values
        self.write_short(0).map_err(WriteError)?;
        Ok(self)
    }

    fn build_rings_part_packet(&mut self) -> Result<&mut Self, NetworkError> {
        // Dummy values
        let num_crush_rings = 0;
        let num_friendship_rings = 0;
        self.write_short(num_crush_rings).map_err(WriteError)?;
        self.write_short(num_friendship_rings).map_err(WriteError)?;
        // Not married
        self.write_short(0).map_err(WriteError)?;
        Ok(self)
    }

    fn build_teleport_part_packet(&mut self) -> Result<&mut Self, NetworkError> {
        // Dummy values
        for _ in 0..5 {
            self.write_int(0).map_err(WriteError)?;
        }
        for _ in 0..10 {
            self.write_int(0).map_err(WriteError)?;
        }
        Ok(self)
    }

    fn build_codex_part_packet(&mut self) -> Result<&mut Self, NetworkError> {
        // Dummy values
        let codex_cover = 1;
        let num_cards = 0;
        self.write_int(codex_cover).map_err(WriteError)?;
        self.write_byte(0).map_err(WriteError)?;
        self.write_short(num_cards).map_err(WriteError)?;
        Ok(self)
    }

    fn build_new_year_cards_part_packet(&mut self) -> Result<&mut Self, NetworkError> {
        // Dummy values
        let num_cards = 0;
        self.write_short(num_cards).map_err(WriteError)?;
        Ok(self)
    }

    fn build_area_info_part_packet(&mut self) -> Result<&mut Self, NetworkError> {
        let num_areas = 0;
        self.write_short(num_areas).map_err(WriteError)?;
        Ok(self)
    }
}
