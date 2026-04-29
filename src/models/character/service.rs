use crate::models::character::error::CharacterError;
use crate::models::character::model::Character;
use crate::models::error::ModelError;
use crate::net::packet::core::Packet;
use crate::net::packet::error::PacketError;
use crate::net::packet::io::error::IOError::WriteError;
use crate::prelude::*;

pub fn write_char_meta(packet: &mut Packet, char: &Character) -> Result<(), ModelError> {
    packet
        .write_int(char.id)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)
        .map_err(ModelError::from)?;
    packet
        .write_str(&char.ign)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)
        .map_err(ModelError::from)?;
    packet
        .write_bytes(&vec![0u8; 13 - char.ign.len()])
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)
        .map_err(ModelError::from)?;
    packet
        .write_byte(char.gender as u8)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)
        .map_err(ModelError::from)?;
    packet
        .write_byte(char.skin as u8)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)
        .map_err(ModelError::from)?;
    packet
        .write_int(char.face)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)
        .map_err(ModelError::from)?;
    packet
        .write_int(char.hair)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)
        .map_err(ModelError::from)?;
    // Pets... Not implemented yet
    packet
        .write_long(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)
        .map_err(ModelError::from)?;
    packet
        .write_long(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)
        .map_err(ModelError::from)?;
    packet
        .write_long(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)
        .map_err(ModelError::from)?;
    packet
        .write_byte(char.level.ok_or(CharacterError::MissingField)? as u8)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)
        .map_err(ModelError::from)?;
    packet
        .write_short(char.job)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)
        .map_err(ModelError::from)?;
    packet
        .write_short(char.strength.ok_or(CharacterError::MissingField)?)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)
        .map_err(ModelError::from)?;
    packet
        .write_short(char.dexterity.ok_or(CharacterError::MissingField)?)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)
        .map_err(ModelError::from)?;
    packet
        .write_short(char.intelligence.ok_or(CharacterError::MissingField)?)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)
        .map_err(ModelError::from)?;
    packet
        .write_short(char.luck.ok_or(CharacterError::MissingField)?)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)
        .map_err(ModelError::from)?;
    packet
        .write_short(char.hp.ok_or(CharacterError::MissingField)?)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)
        .map_err(ModelError::from)?;
    packet
        .write_short(char.max_hp.ok_or(CharacterError::MissingField)?)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)
        .map_err(ModelError::from)?;
    packet
        .write_short(char.mp.ok_or(CharacterError::MissingField)?)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)
        .map_err(ModelError::from)?;
    packet
        .write_short(char.max_mp.ok_or(CharacterError::MissingField)?)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)
        .map_err(ModelError::from)?;
    packet
        .write_short(char.ap.ok_or(CharacterError::MissingField)?)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)
        .map_err(ModelError::from)?;
    // SP
    packet
        .write_short(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)
        .map_err(ModelError::from)?;
    packet
        .write_int(char.exp.ok_or(CharacterError::MissingField)?)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)
        .map_err(ModelError::from)?;
    packet
        .write_short(char.fame.ok_or(CharacterError::MissingField)?)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)
        .map_err(ModelError::from)?;
    // Gach xp?
    packet
        .write_int(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)
        .map_err(ModelError::from)?;
    packet
        .write_int(char.map)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)
        .map_err(ModelError::from)?;
    packet
        .write_byte(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)
        .map_err(ModelError::from)?;
    packet
        .write_int(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)
        .map_err(ModelError::from)?;
    Ok(())
}

fn write_inventory(packet: &mut Packet, char: &Character) -> Result<(), ModelError> {
    packet
        .write_int(char.meso.unwrap())
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)
        .map_err(ModelError::from)?;
    // Inventory slot Capacities
    packet
        .write_bytes(&vec![0u8; 5])
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)
        .map_err(ModelError::from)?;
    // Time?
    packet
        .write_long(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)
        .map_err(ModelError::from)?;

    // 8 bytes skipped
    packet
        .write_short(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)
        .map_err(ModelError::from)?;
    packet
        .write_short(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)
        .map_err(ModelError::from)?;
    packet
        .write_short(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)
        .map_err(ModelError::from)?;

    // Equiped items go here
    // Skip 2 bytes after equips
    packet
        .write_short(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)
        .map_err(ModelError::from)?;
    // Start of USE
    packet
        .write_byte(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)
        .map_err(ModelError::from)?;
    // Start of SETUP
    packet
        .write_byte(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)
        .map_err(ModelError::from)?;
    // Start of ETC
    packet
        .write_byte(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)
        .map_err(ModelError::from)?;
    // Start of CASH
    packet
        .write_byte(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)
        .map_err(ModelError::from)?;
    Ok(())
}

fn write_skills(packet: &mut Packet, _char: &Character) -> Result<(), ModelError> {
    // No skills!
    packet
        .write_short(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)
        .map_err(ModelError::from)?;
    // No no cooldowns!
    packet
        .write_short(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)
        .map_err(ModelError::from)?;
    Ok(())
}

fn write_quests(packet: &mut Packet, _char: &Character) -> Result<(), ModelError> {
    let started_quests = 0;
    packet
        .write_short(started_quests)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)
        .map_err(ModelError::from)?;
    let completed_quests = 0;
    packet
        .write_short(completed_quests)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)
        .map_err(ModelError::from)?;
    Ok(())
}

fn write_minigames(packet: &mut Packet, _char: &Character) -> Result<(), ModelError> {
    packet
        .write_short(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)
        .map_err(ModelError::from)?;
    Ok(())
}

fn write_rings(packet: &mut Packet, _char: &Character) -> Result<(), ModelError> {
    let num_crush_rings = 0;
    let num_friendship_rings = 0;
    packet
        .write_short(num_crush_rings)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)
        .map_err(ModelError::from)?;
    packet
        .write_short(num_friendship_rings)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)
        .map_err(ModelError::from)?;
    // Not married
    packet
        .write_short(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)
        .map_err(ModelError::from)?;
    Ok(())
}

fn write_teleport(packet: &mut Packet, _char: &Character) -> Result<(), ModelError> {
    for _ in 0..5 {
        packet
            .write_int(0)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
    }
    for _ in 0..10 {
        packet
            .write_int(0)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
    }
    Ok(())
}

fn write_codex(packet: &mut Packet, _char: &Character) -> Result<(), ModelError> {
    let codex_cover = 1;
    let num_cards = 0;
    packet
        .write_int(codex_cover)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)
        .map_err(ModelError::from)?;
    packet
        .write_byte(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)
        .map_err(ModelError::from)?;

    packet
        .write_short(num_cards)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)
        .map_err(ModelError::from)?;
    Ok(())
}

fn write_new_year_cards(packet: &mut Packet, _char: &Character) -> Result<(), ModelError> {
    let num_cards = 0;
    packet
        .write_short(num_cards)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)
        .map_err(ModelError::from)?;
    Ok(())
}

fn write_area_info(packet: &mut Packet, _char: &Character) -> Result<(), ModelError> {
    let num_areas = 0;
    packet
        .write_short(num_areas)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)
        .map_err(ModelError::from)?;
    Ok(())
}

pub fn write_char_look(packet: &mut Packet, char: &Character) -> Result<(), ModelError> {
    packet
        .write_byte(char.gender as u8)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)
        .map_err(ModelError::from)?;
    packet
        .write_byte(char.skin as u8)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)
        .map_err(ModelError::from)?;
    packet
        .write_int(char.face)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)
        .map_err(ModelError::from)?;
    packet
        .write_byte(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)
        .map_err(ModelError::from)?;
    packet
        .write_int(char.hair)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)
        .map_err(ModelError::from)?;
    Ok(())
}
pub fn write_char_equips(packet: &mut Packet, _character: &Character) -> Result<(), ModelError> {
    // Overall (Top slot)
    packet
        .write_byte(5)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)
        .map_err(ModelError::from)?;
    packet
        .write_int(1052122)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)
        .map_err(ModelError::from)?;
    // Shoes
    packet
        .write_byte(7)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)
        .map_err(ModelError::from)?;
    packet
        .write_int(1072318)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)
        .map_err(ModelError::from)?;
    packet
        .write_byte(0xFF)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)
        .map_err(ModelError::from)?;
    // Cash shop equips
    packet
        .write_byte(0xFF)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)
        .map_err(ModelError::from)?;
    // Weapon
    packet
        .write_int(1302000)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)
        .map_err(ModelError::from)?;
    // Pet stuff...
    packet
        .write_int(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)
        .map_err(ModelError::from)?;
    packet
        .write_int(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)
        .map_err(ModelError::from)?;
    packet
        .write_int(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)
        .map_err(ModelError::from)?;
    Ok(())
}

pub fn write_list_char(packet: &mut Packet, char: &Character) -> Result<(), ModelError> {
    write_char_meta(packet, &char)?;
    write_char_look(packet, &char)?;
    write_char_equips(packet, char)?;
    packet
        .write_byte(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)
        .map_err(ModelError::from)?;
    // Disable rank.
    packet
        .write_byte(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)
        .map_err(ModelError::from)?;
    Ok(())
}

pub fn write_game_char(packet: &mut Packet, char: &Character) -> Result<(), ModelError> {
    let bl_capacity = 25;
    packet
        .write_byte(bl_capacity)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)
        .map_err(ModelError::from)?;
    packet
        .write_byte(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)
        .map_err(ModelError::from)?;
    write_inventory(packet, char)?;
    write_skills(packet, char)?;
    write_quests(packet, char)?;
    write_minigames(packet, char)?;
    write_rings(packet, char)?;
    write_teleport(packet, char)?;
    write_codex(packet, char)?;
    write_new_year_cards(packet, char)?;
    write_area_info(packet, char)?;
    packet
        .write_short(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)
        .map_err(ModelError::from)?;
    Ok(())
}
