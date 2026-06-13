/* app/src/system/packet/build/codec/look/builder.rs
 * The purpose of this module is to build generic character look packet bytes.
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

use crate::component::character::MapleCharacter;
use crate::component::item::MapleItem;
use crate::system::packet::build::error::PacketBuildError;
use net::packet::io::error::IOError::WriteError;
use net::packet::io::prelude::*;
use net::packet::model::Packet;

pub fn build_cash_equipment_part_packet(
    packet: &mut Packet,
    equips: &Vec<MapleItem>,
) -> Result<(), PacketBuildError> {
    for equip in equips {
        if let Some(ipos) = equip.ipos {
            if equip.base.cash {
                packet.write_byte(ipos).map_err(WriteError)?;
                packet.write_int(equip.base.wz).map_err(WriteError)?;
            }
        } else {
            continue;
        };
    }
    Ok(())
}

pub fn build_look_regular_equipment_part_packet(
    packet: &mut Packet,
    equips: &Vec<MapleItem>,
) -> Result<(), PacketBuildError> {
    for equip in equips {
        if let Some(ipos) = equip.ipos {
            if !equip.base.cash {
                packet.write_byte(ipos).map_err(WriteError)?;
                packet.write_int(equip.base.wz).map_err(WriteError)?;
            }
        } else {
            continue;
        };
    }
    Ok(())
}

pub fn build_look_meta_part_packet(
    packet: &mut Packet,
    char: &MapleCharacter,
    equips: &Vec<MapleItem>,
) -> Result<(), PacketBuildError> {
    packet.write_byte(char.gender_wz).map_err(WriteError)?;
    packet.write_byte(char.skin_wz as i16).map_err(WriteError)?;
    packet.write_int(char.face_wz).map_err(WriteError)?;
    packet
        .write_byte(0) // megaphone
        .map_err(WriteError)?;
    packet.write_int(char.hair_wz).map_err(WriteError)?;
    build_look_regular_equipment_part_packet(packet, equips)?;
    packet.write_byte(0xFF).map_err(WriteError)?;
    build_cash_equipment_part_packet(packet, equips)?;
    packet.write_byte(0xFF).map_err(WriteError)?;
    packet
        .write_int(0) //maskedequips -111
        .map_err(WriteError)?;
    // Pet stuff...
    packet.write_int(0).map_err(WriteError)?;
    packet.write_int(0).map_err(WriteError)?;
    packet.write_int(0).map_err(WriteError)?;
    Ok(())
}
