/* app/src/system/packet/build/codec/item/builder.rs
 * The purpose of this module is to build generic item packet bytes.
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

use crate::component::item::MapleItem;
use crate::system::packet::build::error::PacketBuildError;
use base::inventory::{InventoryMod, InventoryModMode, InventoryTab};
use base::map::Point;
use net::packet::io::error::IOError::WriteError;
use net::packet::io::prelude::*;
use net::packet::model::Packet;
use op::send::SendOpcode;

pub fn build_drop_loot_packet(
    mode: u8, // animation 0 fade, 1 drop mob, 2 spawn in
    id: u32,
    is_meso: bool,
    wz_or_meso_amount: i32,
    owner: i32,     // char id or 0
    can_pickup: u8, // 0 everyone 1 owner, 2 party
    drop_to: Point,
    drop_from: Point,
    player_drop: bool,
) -> Result<Packet, PacketBuildError> {
    let mut packet: Packet = Packet::new_empty();
    let op = SendOpcode::DropLoot as i16;
    packet.write_short(op).map_err(WriteError)?;
    packet.write_byte(mode as i16).map_err(WriteError)?;
    packet.write_int(id as i32).map_err(WriteError)?;
    packet.write_byte(is_meso as i16).map_err(WriteError)?;
    packet.write_int(wz_or_meso_amount).map_err(WriteError)?;
    packet.write_int(owner).map_err(WriteError)?;
    packet.write_byte(can_pickup as i16).map_err(WriteError)?;
    packet.write_short(drop_to.x).map_err(WriteError)?;
    packet.write_short(drop_to.y).map_err(WriteError)?;
    let skip: Vec<u8> = vec![0; 4];
    packet.write_bytes(skip.clone()).map_err(WriteError)?;
    if mode != 2 {
        packet.write_short(drop_from.x).map_err(WriteError)?;
        packet.write_short(drop_from.y).map_err(WriteError)?;
        let skip: Vec<u8> = vec![0; 2];
        packet.write_bytes(skip.clone()).map_err(WriteError)?;
    }
    if !is_meso {
        let skip: Vec<u8> = vec![0; 8];
        packet.write_bytes(skip.clone()).map_err(WriteError)?;
    }
    packet.write_byte(player_drop as i16).map_err(WriteError)?;
    Ok(packet)
}

pub fn build_remove_loot_packet(item_id: i32) -> Result<Packet, PacketBuildError> {
    let mut packet: Packet = Packet::new_empty();
    let op = SendOpcode::RemoveLoot as i16;
    packet.write_short(op).map_err(WriteError)?;
    let mode: u8 = 0;
    packet.write_byte(mode as i16).map_err(WriteError)?;
    packet.write_int(item_id).map_err(WriteError)?;
    Ok(packet)
}

pub fn build_add_to_inventory_packet(
    inv_mod: InventoryMod,
    item: &MapleItem,
) -> Result<Packet, PacketBuildError> {
    let mut packet: Packet = Packet::new_empty();
    packet
        .write_short(SendOpcode::ModifyInventory as i16)
        .map_err(WriteError)?;
    packet.write_byte(true as i16).map_err(WriteError)?; // updatetick
    let mods_len: i16 = 1;
    packet.write_byte(mods_len).map_err(WriteError)?;
    packet
        .write_byte(inv_mod.mode.clone() as i16)
        .map_err(WriteError)?;
    packet
        .write_byte(item.base.itab as i16)
        .map_err(WriteError)?;
    packet.write_short(inv_mod.ipos).map_err(WriteError)?;
    match inv_mod.mode {
        InventoryModMode::Add => {
            build_item_data(&mut packet, item, inv_mod.char_name.clone(), inv_mod.count)?;
        }
        InventoryModMode::ChangeCount => {
            packet.write_short(inv_mod.count).map_err(WriteError)?;
        }
        _ => {}
    }
    Ok(packet)
}

pub fn build_item_data(
    packet: &mut Packet,
    item: &MapleItem,
    char_name: String,
    count: i16,
) -> Result<(), PacketBuildError> {
    match item.base.itab {
        x if x == InventoryTab::Equip as i8 => {
            let item_type: u8 = 1;
            packet.write_byte(item_type as i16).map_err(WriteError)?; // type byte
            packet.write_int(item.base.wz).map_err(WriteError)?;
            packet
                .write_byte(item.base.cash as i16)
                .map_err(WriteError)?;
            if item.base.cash {
                packet.write_bytes(vec![0u8; 8]).map_err(WriteError)?; // unique id
            }
            packet.write_long(item.expire).map_err(WriteError)?;
            packet.write_byte(item.slots as i16).map_err(WriteError)?;
            packet.write_byte(item.level).map_err(WriteError)?;
            // stats - order matters, must match EquipStat enum order
            packet.write_short(item.strength).map_err(WriteError)?;
            packet.write_short(item.dexterity).map_err(WriteError)?;
            packet.write_short(item.intelligence).map_err(WriteError)?;
            packet.write_short(item.luck).map_err(WriteError)?;
            packet.write_short(item.hp).map_err(WriteError)?;
            packet.write_short(item.mp).map_err(WriteError)?;
            packet.write_short(item.attack).map_err(WriteError)?;
            packet.write_short(item.magic).map_err(WriteError)?;
            packet
                .write_short(item.weapon_defense)
                .map_err(WriteError)?;
            packet.write_short(item.magic_defense).map_err(WriteError)?;
            packet.write_short(item.accuracy).map_err(WriteError)?;
            packet.write_short(item.avoid).map_err(WriteError)?;
            packet.write_short(item.hands).map_err(WriteError)?;
            packet.write_short(item.speed).map_err(WriteError)?;
            packet.write_short(item.jump).map_err(WriteError)?;
            packet
                .write_str_with_length(char_name)
                .map_err(WriteError)?;
            packet.write_short(item.flag).map_err(WriteError)?;
            if item.base.cash {
                packet.write_bytes(vec![0u8; 10]).map_err(WriteError)?;
            } else {
                packet.write_byte(0).map_err(WriteError)?;
                packet.write_byte(item.item_level).map_err(WriteError)?;
                packet.write_short(0).map_err(WriteError)?;
                packet.write_short(item.item_exp).map_err(WriteError)?;
                packet.write_int(item.vicious).map_err(WriteError)?;
                packet.write_long(0).map_err(WriteError)?;
            }
            packet.write_bytes(vec![0u8; 12]).map_err(WriteError)?;
        }
        _ => {
            let item_type: u8 = 2;
            packet.write_byte(item_type as i16).map_err(WriteError)?; // type byte
            packet.write_int(item.base.wz).map_err(WriteError)?;
            packet
                .write_byte(item.base.cash as i16)
                .map_err(WriteError)?;
            if item.base.cash {
                packet.write_bytes(vec![0u8; 8]).map_err(WriteError)?;
            }
            packet.write_long(item.expire).map_err(WriteError)?;
            packet.write_short(count).map_err(WriteError)?;
            packet
                .write_str_with_length(char_name)
                .map_err(WriteError)?;
            packet.write_short(item.flag).map_err(WriteError)?;
            if (item.base.wz / 10000 == 233) || (item.base.wz / 10000 == 207) {
                packet.write_bytes(vec![0u8; 8]).map_err(WriteError)?;
            }
        }
    }
    Ok(())
}
