/* spawn_player/builder.rs
 * The purpose of this module is to build an outgoing spawn player packet.
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
use crate::models::item::wrapper::{EquipItem, Item};
use crate::net::packet::codec::spawn_player::error::CodecSpawnPlayerError;
use crate::net::packet::io::error::IOError::WriteError;
use crate::net::packet::model::Packet;
use crate::op::send::SendOpcode;
use crate::prelude::*;

impl Packet {
    pub fn build_spawn_player_packet(
        &mut self,
        char: &Character,
    ) -> Result<&mut Self, CodecSpawnPlayerError> {
        let op = SendOpcode::SpawnPlayer as i16;
        self.write_short(op).map_err(WriteError)?;
        self.write_int(char.model.get_id()?).map_err(WriteError)?;
        let level = char.model.level as i16;
        self.write_byte(level).map_err(WriteError)?;
        self.write_str_with_length(char.model.ign.clone())
            .map_err(WriteError)?;
        let guild_name = String::from("Guild Name");
        self.write_str_with_length(guild_name).map_err(WriteError)?;
        let skip = 0 as i16;
        self.write_short(skip).map_err(WriteError)?; // guildlogobg
        let skip = 0 as i16;
        self.write_byte(skip).map_err(WriteError)?; //guildlogobgcolor
        let skip = 0 as i16;
        self.write_short(skip).map_err(WriteError)?; //guildlogo
        let skip = 0 as i16;
        self.write_byte(skip).map_err(WriteError)?; //guildlogocolor
        let skip = vec![0u8; 8];
        self.write_bytes(skip).map_err(WriteError)?;
        let morphed = 0; // 2 if morphed
        self.write_int(morphed).map_err(WriteError)?;
        let buff_mask_one = 0;
        self.write_int(buff_mask_one).map_err(WriteError)?;
        if buff_mask_one != 0 {
            if morphed == 2 {
                let buff_value = 0; // changes if morphed
                self.write_short(buff_value).map_err(WriteError)?;
            } else {
                let buff_value = 0; // changes if not morphed
                self.write_byte(buff_value).map_err(WriteError)?;
            }
        }
        let buff_mask_two = 0; // 0 not sure
        self.write_int(buff_mask_two).map_err(WriteError)?;
        let skip = vec![0u8; 43];
        self.write_bytes(skip).map_err(WriteError)?;
        let mount = 0; // 0 not sure
        self.write_int(mount).map_err(WriteError)?;
        let skip = vec![0u8; 61];
        self.write_bytes(skip).map_err(WriteError)?;
        self.write_short(char.model.job_wz).map_err(WriteError)?;
        self.build_look_meta_part_packet(char)?;
        let count = 5110000;
        self.write_int(count).map_err(WriteError)?;
        let item_effect = 0; // 0 not sure
        self.write_int(item_effect).map_err(WriteError)?;
        let chair = 0; // 0 not sure
        self.write_int(chair).map_err(WriteError)?;
        let position_x = 0; // 0 this is a point so it might be wrong
        let position_y = 0; // 0 this is a point so it might be wrong
        self.write_short(position_x).map_err(WriteError)?;
        self.write_short(position_y).map_err(WriteError)?;
        let stance = 0 as i16; // 0 not sure
        self.write_byte(stance).map_err(WriteError)?;
        let skip = vec![0u8; 3];
        self.write_bytes(skip).map_err(WriteError)?;
        for _ in 0..3 {
            let available = 0; // 0 not sure
            self.write_byte(available).map_err(WriteError)?;
            if available == 1 {
                let byte_two = 0 as i16; // 0 not sure
                self.write_byte(byte_two).map_err(WriteError)?;
                let pet_id = 0; // 0 is definitely not right
                self.write_int(pet_id).map_err(WriteError)?;
                let pet_name = String::from("George");
                self.write_str_with_length(pet_name).map_err(WriteError)?;
                let unique_id = 0; // 0 not sure
                self.write_int(unique_id).map_err(WriteError)?;
                let skip = 0;
                self.write_int(skip).map_err(WriteError)?;
                self.write_short(position_x).map_err(WriteError)?;
                self.write_short(position_y).map_err(WriteError)?;
                self.write_byte(stance).map_err(WriteError)?;
                let fhid = 0; // 0 not sure
                self.write_int(fhid).map_err(WriteError)?;
            } else {
                break;
            }
        }
        let mount_level = 0; // 0 not sure
        self.write_int(mount_level).map_err(WriteError)?;
        let mount_exp = 0; // 0 not sure
        self.write_int(mount_exp).map_err(WriteError)?;
        let mount_tiredness = 0; // 0 not sure
        self.write_int(mount_tiredness).map_err(WriteError)?;
        let skip = 0 as i16;
        self.write_byte(skip).map_err(WriteError)?; // shop stuff
        let chalkboard_bool: bool = false; // false not sure
        let chalkboard: i16 = chalkboard_bool as i16; // false not sure
        self.write_byte(chalkboard).map_err(WriteError)?;
        if chalkboard_bool {
            let chalkboard_text = String::from("Placeholder");
            self.write_str_with_length(chalkboard_text)
                .map_err(WriteError)?;
        }
        let skip = vec![0u8; 3];
        self.write_bytes(skip).map_err(WriteError)?;
        let team = 0 as i16; // 0 not sure
        self.write_byte(team).map_err(WriteError)?;
        Ok(self)
    }

    pub fn build_look_cash_equipment_part_packet(
        &mut self,
        char: &Character,
    ) -> Result<&mut Self, CodecSpawnPlayerError> {
        for (ipos, equip) in char.inventory.equipped_tab.iter() {
            match equip {
                Item::CashEquip(i) => {
                    self.write_byte(*ipos).map_err(WriteError)?;
                    self.write_int(i.model.wz).map_err(WriteError)?;
                }
                _ => (),
            }
        }
        Ok(self)
    }

    pub fn build_look_regular_equipment_part_packet(
        &mut self,
        char: &Character,
    ) -> Result<&mut Self, CodecSpawnPlayerError> {
        for (ipos, equip) in char.inventory.equipped_tab.iter() {
            match equip {
                Item::Equip(i) => {
                    self.write_byte(*ipos).map_err(WriteError)?;
                    self.write_int(i.model.wz).map_err(WriteError)?;
                }
                _ => (),
            }
        }
        Ok(self)
    }

    pub fn build_list_char_meta_part_packet(
        &mut self,
        char: &Character,
    ) -> Result<&mut Self, CodecSpawnPlayerError> {
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
        char: &Character,
    ) -> Result<&mut Self, CodecSpawnPlayerError> {
        self.write_byte(char.model.gender_wz as i16)
            .map_err(WriteError)?;
        self.write_byte(char.model.skin_wz as i16)
            .map_err(WriteError)?;
        self.write_int(char.model.face_wz).map_err(WriteError)?;
        self.write_byte(0) // megaphone
            .map_err(WriteError)?;
        self.write_int(char.model.hair_wz).map_err(WriteError)?;
        self.build_look_regular_equipment_part_packet(char)?;
        self.write_byte(0xFF).map_err(WriteError)?;
        self.build_look_cash_equipment_part_packet(char)?;
        self.write_byte(0xFF).map_err(WriteError)?;
        self.write_int(0) //maskedequips -111
            .map_err(WriteError)?;
        // Pet stuff...
        self.write_int(0).map_err(WriteError)?;
        self.write_int(0).map_err(WriteError)?;
        self.write_int(0).map_err(WriteError)?;
        Ok(self)
    }

    pub fn build_player_logged_in_meta_part_packet(
        &mut self,
        char: &Character,
    ) -> Result<&mut Self, CodecSpawnPlayerError> {
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
        self.build_inventory_part_packet(char)?
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

    fn build_skills_part_packet(&mut self) -> Result<&mut Self, CodecSpawnPlayerError> {
        // Dummy values
        // No skills!
        self.write_short(0).map_err(WriteError)?;
        // No no cooldowns!
        self.write_short(0).map_err(WriteError)?;
        Ok(self)
    }

    fn build_quests_part_packet(&mut self) -> Result<&mut Self, CodecSpawnPlayerError> {
        // Dummy values
        let started_quests = 0;
        self.write_short(started_quests).map_err(WriteError)?;
        let completed_quests = 0;
        self.write_short(completed_quests).map_err(WriteError)?;
        Ok(self)
    }

    fn build_minigames_part_packet(&mut self) -> Result<&mut Self, CodecSpawnPlayerError> {
        // Dummy values
        self.write_short(0).map_err(WriteError)?;
        Ok(self)
    }

    fn build_rings_part_packet(&mut self) -> Result<&mut Self, CodecSpawnPlayerError> {
        // Dummy values
        let num_crush_rings = 0;
        let num_friendship_rings = 0;
        self.write_short(num_crush_rings).map_err(WriteError)?;
        self.write_short(num_friendship_rings).map_err(WriteError)?;
        // Not married
        self.write_short(0).map_err(WriteError)?;
        Ok(self)
    }

    fn build_teleport_part_packet(&mut self) -> Result<&mut Self, CodecSpawnPlayerError> {
        // Dummy values
        for _ in 0..5 {
            self.write_int(0).map_err(WriteError)?;
        }
        for _ in 0..10 {
            self.write_int(0).map_err(WriteError)?;
        }
        Ok(self)
    }

    fn build_codex_part_packet(&mut self) -> Result<&mut Self, CodecSpawnPlayerError> {
        // Dummy values
        let codex_cover = 1;
        let num_cards = 0;
        self.write_int(codex_cover).map_err(WriteError)?;
        self.write_byte(0).map_err(WriteError)?;
        self.write_short(num_cards).map_err(WriteError)?;
        Ok(self)
    }

    fn build_new_year_cards_part_packet(&mut self) -> Result<&mut Self, CodecSpawnPlayerError> {
        // Dummy values
        let num_cards = 0;
        self.write_short(num_cards).map_err(WriteError)?;
        Ok(self)
    }

    fn build_area_info_part_packet(&mut self) -> Result<&mut Self, CodecSpawnPlayerError> {
        let num_areas = 0;
        self.write_short(num_areas).map_err(WriteError)?;
        Ok(self)
    }

    pub fn build_inventory_cash_equipment_part_packet(
        &mut self,
        char: &Character,
    ) -> Result<&mut Self, CodecSpawnPlayerError> {
        for (ipos, equip) in char.inventory.equipped_tab.iter() {
            match equip {
                Item::CashEquip(i) => {
                    self.write_short(*ipos).map_err(WriteError)?;
                    self.write_int(i.model.wz).map_err(WriteError)?;
                }
                _ => (),
            }
        }
        Ok(self)
    }

    pub fn build_inventory_regular_equipment_part_packet(
        &mut self,
        char: &Character,
    ) -> Result<&mut Self, CodecSpawnPlayerError> {
        for (ipos, equip) in char.inventory.equipped_tab.iter() {
            match equip {
                Item::Equip(i) => {
                    self.write_short(*ipos).map_err(WriteError)?;
                    self.build_inventory_regular_equip_meta_part_packet(&i)?;
                }
                _ => (),
            }
        }
        Ok(self)
    }

    fn build_inventory_regular_equip_meta_part_packet(
        &mut self,
        equip: &EquipItem,
    ) -> Result<&mut Self, CodecSpawnPlayerError> {
        // Dummy values
        self.write_byte(1).map_err(WriteError)?;
        self.write_int(equip.model.wz).map_err(WriteError)?;
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

    pub fn build_inventory_part_packet(
        &mut self,
        char: &Character,
    ) -> Result<&mut Self, CodecSpawnPlayerError> {
        self.write_int(char.model.meso).map_err(WriteError)?;
        // Dummy values
        // Inventory slot Capacities
        self.write_bytes(vec![0u8; 5]).map_err(WriteError)?;
        // Time?
        self.write_long(0).map_err(WriteError)?;
        self.build_inventory_regular_equipment_part_packet(char)?;
        self.build_inventory_cash_equipment_part_packet(char)?;
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
}
