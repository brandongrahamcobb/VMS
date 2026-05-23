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

use crate::build::error::PacketBuildError;
use crate::io::error::IOError::WriteError;
use crate::model::Packet;
use crate::prelude::*;
use entity::character::wrapper::Character;
use entity::item::wrapper::Item;
use op::send::SendOpcode;

impl Packet {
    pub fn build_spawn_player_packet(
        &mut self,
        char: &Character,
    ) -> Result<&mut Self, PacketBuildError> {
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

    pub fn build_set_field_packet(
        &mut self,
        char: &Character,
        channel_id: u8,
        map_wz: i32,
    ) -> Result<&mut Self, PacketBuildError> {
        let op = SendOpcode::SetField as i16;
        self.write_short(op).map_err(WriteError)?;
        self.write_int(channel_id as i32).map_err(WriteError)?;
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
        self.build_player_logged_in_meta_part_packet(char, map_wz)?;
        Ok(self)
    }

    pub fn build_look_cash_equipment_part_packet(
        &mut self,
        char: &Character,
    ) -> Result<&mut Self, PacketBuildError> {
        for (ipos, equips) in char.inventory.equipped_tab.iter() {
            if equips[0].info.cash {
                self.write_byte(-*ipos).map_err(WriteError)?;
                self.write_int(equips[0].model.wz).map_err(WriteError)?;
            }
        }
        Ok(self)
    }

    pub fn build_look_regular_equipment_part_packet(
        &mut self,
        char: &Character,
    ) -> Result<&mut Self, PacketBuildError> {
        for (ipos, equips) in char.inventory.equipped_tab.iter() {
            if !equips[0].info.cash {
                self.write_byte(-*ipos).map_err(WriteError)?;
                self.write_int(equips[0].model.wz).map_err(WriteError)?;
            }
        }
        Ok(self)
    }

    pub fn build_list_char_meta_part_packet(
        &mut self,
        char: &Character,
    ) -> Result<&mut Self, PacketBuildError> {
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
    ) -> Result<&mut Self, PacketBuildError> {
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
        map_wz: i32,
    ) -> Result<&mut Self, PacketBuildError> {
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
        self.write_int(map_wz).map_err(WriteError)?;
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

    fn build_skills_part_packet(&mut self) -> Result<&mut Self, PacketBuildError> {
        // Dummy values
        // No skills!
        self.write_short(0).map_err(WriteError)?;
        // No no cooldowns!
        self.write_short(0).map_err(WriteError)?;
        Ok(self)
    }

    fn build_quests_part_packet(&mut self) -> Result<&mut Self, PacketBuildError> {
        // Dummy values
        let started_quests = 0;
        self.write_short(started_quests).map_err(WriteError)?;
        let completed_quests = 0;
        self.write_short(completed_quests).map_err(WriteError)?;
        Ok(self)
    }

    fn build_minigames_part_packet(&mut self) -> Result<&mut Self, PacketBuildError> {
        // Dummy values
        self.write_short(0).map_err(WriteError)?;
        Ok(self)
    }

    fn build_rings_part_packet(&mut self) -> Result<&mut Self, PacketBuildError> {
        // Dummy values
        let num_crush_rings = 0;
        let num_friendship_rings = 0;
        self.write_short(num_crush_rings).map_err(WriteError)?;
        self.write_short(num_friendship_rings).map_err(WriteError)?;
        // Not married
        self.write_short(0).map_err(WriteError)?;
        Ok(self)
    }

    fn build_teleport_part_packet(&mut self) -> Result<&mut Self, PacketBuildError> {
        // Dummy values
        for _ in 0..5 {
            self.write_int(0).map_err(WriteError)?;
        }
        for _ in 0..10 {
            self.write_int(0).map_err(WriteError)?;
        }
        Ok(self)
    }

    fn build_codex_part_packet(&mut self) -> Result<&mut Self, PacketBuildError> {
        // Dummy values
        let codex_cover = 1;
        let num_cards = 0;
        self.write_int(codex_cover).map_err(WriteError)?;
        self.write_byte(0).map_err(WriteError)?;
        self.write_short(num_cards).map_err(WriteError)?;
        Ok(self)
    }

    fn build_new_year_cards_part_packet(&mut self) -> Result<&mut Self, PacketBuildError> {
        // Dummy values
        let num_cards = 0;
        self.write_short(num_cards).map_err(WriteError)?;
        Ok(self)
    }

    fn build_area_info_part_packet(&mut self) -> Result<&mut Self, PacketBuildError> {
        let num_areas = 0;
        self.write_short(num_areas).map_err(WriteError)?;
        Ok(self)
    }

    pub fn build_inventory_cash_equipment_part_packet(
        &mut self,
        char: &Character,
    ) -> Result<&mut Self, PacketBuildError> {
        for (ipos, equips) in char.inventory.equipped_tab.iter() {
            if equips[0].info.cash {
                self.write_short(-*ipos).map_err(WriteError)?;
                self.write_int(equips[0].model.wz).map_err(WriteError)?;
            }
        }
        Ok(self)
    }

    pub fn build_inventory_regular_equipment_part_packet(
        &mut self,
        char: &Character,
    ) -> Result<&mut Self, PacketBuildError> {
        for (ipos, equips) in char.inventory.equipped_tab.iter() {
            if !equips[0].info.cash {
                self.write_short(-*ipos).map_err(WriteError)?;
                self.build_inventory_regular_equip_meta_part_packet(&equips[0])?;
            }
        }
        Ok(self)
    }

    fn build_inventory_regular_equip_meta_part_packet(
        &mut self,
        equip: &Item,
    ) -> Result<&mut Self, PacketBuildError> {
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
    ) -> Result<&mut Self, PacketBuildError> {
        self.write_int(char.model.meso).map_err(WriteError)?;
        // Dummy values
        // Inventory slot Capacities
        self.write_bytes(vec![8u8; 5]).map_err(WriteError)?;
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

    pub fn build_despawn_player_packet(
        &mut self,
        char: &Character,
    ) -> Result<&mut Self, PacketBuildError> {
        let op = SendOpcode::DespawnPlayer as i16;
        self.write_short(op).map_err(WriteError)?;
        self.write_int(char.model.get_id()?).map_err(WriteError)?;
        Ok(self)
    }

    pub fn build_set_exp_packet(&mut self, exp: i32) -> Result<&mut Self, PacketBuildError> {
        let op = SendOpcode::ChangeStats as i16;
        self.write_short(op).map_err(WriteError)?;
        self.write_byte(0i16).map_err(WriteError)?; // itemreaction
        self.write_int(0x10000i32).map_err(WriteError)?; // updatemask: HP
        self.write_int(exp).map_err(WriteError)?;
        Ok(self)
    }

    pub fn build_set_level_packet(&mut self, level: i16) -> Result<&mut Self, PacketBuildError> {
        let op = SendOpcode::ChangeStats as i16;
        self.write_short(op).map_err(WriteError)?;
        self.write_byte(0i16).map_err(WriteError)?; // itemreaction
        self.write_int(0x10i32).map_err(WriteError)?; // updatemask: HP
        self.write_byte(level).map_err(WriteError)?;
        Ok(self)
    }

    pub fn build_level_up_effect_packet(
        &mut self,
        char_id: i32,
    ) -> Result<&mut Self, PacketBuildError> {
        let op = SendOpcode::ShowForeignEffect as i16;
        self.write_short(op).map_err(WriteError)?;
        self.write_int(char_id).map_err(WriteError)?;
        self.write_byte(1).map_err(WriteError)?; // level up
        self.write_int(0).map_err(WriteError)?; // skillid
        self.write_byte(0).map_err(WriteError)?; // direction
        Ok(self)
    }

    pub fn build_set_ap_packet(&mut self, ap: i16) -> Result<&mut Self, PacketBuildError> {
        let op = SendOpcode::ChangeStats as i16;
        self.write_short(op).map_err(WriteError)?;
        self.write_byte(0i16).map_err(WriteError)?; // itemreaction
        self.write_int(0x4000i32).map_err(WriteError)?; // updatemask: HP
        self.write_short(ap).map_err(WriteError)?;
        Ok(self)
    }

    // 0x1       SKIN short
    // 0x2       FACE none
    // 0x4       HAIR int
    // 0x10      LEVEL byte
    // 0x20      JOB short
    // 0x40      STR short
    // 0x80      DEX short
    // 0x100     INT short
    // 0x200     LUK short
    // 0x400     HP short
    // 0x800     MAXHP short
    // 0x1000    MP short
    // 0x2000    MAXMP short
    // 0x4000    AP short
    // 0x8000    SP short
    // 0x10000   EXP int
    // 0x20000   FAME short
    // 0x40000   MESO int
    // 0x180008  PET short
    // 0x200000  GACHAEXP short
}
