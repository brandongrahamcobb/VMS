use crate::models::character::error::CharacterError;
use crate::models::character::model::Character;
use crate::net::packet::core::Packet;
use crate::net::packet::error::PacketError;
use crate::net::packet::io::error::IOError::WriteError;
use crate::prelude::*;

pub fn write_char_meta(packet: &mut Packet, character: &Character) -> Result<(), CharacterError> {
    packet
        .write_int(character.id)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)?;
    packet
        .write_str(&character.ign)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)?;
    packet
        .write_bytes(&vec![0u8; 13 - character.ign.len()])
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)?;
    packet
        .write_byte(character.gender as u8)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)?;
    packet
        .write_byte(character.skin as u8)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)?;
    packet
        .write_int(character.face)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)?;
    packet
        .write_int(character.hair)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)?;
    // Pets... Not implemented yet
    packet
        .write_long(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)?;
    packet
        .write_long(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)?;
    packet
        .write_long(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)?;
    packet
        .write_byte(character.level.ok_or(CharacterError::MissingField)? as u8)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)?;
    packet
        .write_short(character.job)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)?;
    packet
        .write_short(character.strength.ok_or(CharacterError::MissingField)?)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)?;
    packet
        .write_short(character.dexterity.ok_or(CharacterError::MissingField)?)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)?;
    packet
        .write_short(character.intelligence.ok_or(CharacterError::MissingField)?)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)?;
    packet
        .write_short(character.luck.ok_or(CharacterError::MissingField)?)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)?;
    packet
        .write_short(character.hp.ok_or(CharacterError::MissingField)?)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)?;
    packet
        .write_short(character.max_hp.ok_or(CharacterError::MissingField)?)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)?;
    packet
        .write_short(character.mp.ok_or(CharacterError::MissingField)?)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)?;
    packet
        .write_short(character.max_mp.ok_or(CharacterError::MissingField)?)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)?;
    packet
        .write_short(character.ap.ok_or(CharacterError::MissingField)?)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)?;
    // SP
    packet
        .write_short(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)?;
    packet
        .write_int(character.exp.ok_or(CharacterError::MissingField)?)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)?;
    packet
        .write_short(character.fame.ok_or(CharacterError::MissingField)?)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)?;
    // Gach xp?
    packet
        .write_int(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)?;
    packet
        .write_int(character.map.ok_or(CharacterError::MissingField)?)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)?;
    packet
        .write_byte(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)?;
    packet
        .write_int(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)?;
    Ok(())
}

fn write_inventory(packet: &mut Packet, character: &Character) -> Result<(), CharacterError> {
    packet
        .write_int(character.meso.unwrap())
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)?;
    // Inventory slot Capacities
    packet
        .write_bytes(&vec![0u8; 5])
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)?;
    // Time?
    packet
        .write_long(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)?;

    // 8 bytes skipped
    packet
        .write_short(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)?;
    packet
        .write_short(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)?;
    packet
        .write_short(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)?;

    // Equiped items go here
    // Skip 2 bytes after equips
    packet
        .write_short(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)?;
    // Start of USE
    packet
        .write_byte(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)?;
    // Start of SETUP
    packet
        .write_byte(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)?;
    // Start of ETC
    packet
        .write_byte(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)?;
    // Start of CASH
    packet
        .write_byte(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)?;
    Ok(())
}

fn write_skills(packet: &mut Packet, _character: &Character) -> Result<(), CharacterError> {
    // No skills!
    packet
        .write_short(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)?;
    // No no cooldowns!
    packet
        .write_short(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)?;
    Ok(())
}

fn write_quests(packet: &mut Packet, _character: &Character) -> Result<(), CharacterError> {
    let started_quests = 0;
    packet
        .write_short(started_quests)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)?;
    let completed_quests = 0;
    packet
        .write_short(completed_quests)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)?;
    Ok(())
}

fn write_minigames(packet: &mut Packet, _character: &Character) -> Result<(), CharacterError> {
    packet
        .write_short(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)?;
    Ok(())
}

fn write_rings(packet: &mut Packet, _character: &Character) -> Result<(), CharacterError> {
    let num_crush_rings = 0;
    let num_friendship_rings = 0;
    packet
        .write_short(num_crush_rings)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)?;
    packet
        .write_short(num_friendship_rings)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)?;
    // Not married
    packet
        .write_short(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)?;
    Ok(())
}

fn write_teleport(packet: &mut Packet, _character: &Character) -> Result<(), CharacterError> {
    for _ in 0..5 {
        packet
            .write_int(0)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)?;
    }
    for _ in 0..10 {
        packet
            .write_int(0)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)?;
    }
    Ok(())
}

fn write_codex(packet: &mut Packet, _character: &Character) -> Result<(), CharacterError> {
    let codex_cover = 1;
    let num_cards = 0;
    packet
        .write_int(codex_cover)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)?;
    packet
        .write_byte(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)?;

    packet
        .write_short(num_cards)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)?;
    Ok(())
}

fn write_new_year_cards(packet: &mut Packet, _character: &Character) -> Result<(), CharacterError> {
    let num_cards = 0;
    packet
        .write_short(num_cards)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)?;
    Ok(())
}

fn write_area_info(packet: &mut Packet, _character: &Character) -> Result<(), CharacterError> {
    let num_areas = 0;
    packet
        .write_short(num_areas)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)?;
    Ok(())
}

fn write_char_look(packet: &mut Packet, character: &Character) -> Result<(), CharacterError> {
    packet
        .write_byte(character.gender as u8)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)?;
    packet
        .write_byte(character.skin as u8)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)?;
    packet
        .write_int(character.face)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)?;
    packet
        .write_byte(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)?;
    packet
        .write_int(character.hair)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)?;
    Ok(())
}
fn write_char_equips(packet: &mut Packet, _character: &Character) -> Result<(), CharacterError> {
    // Overall (Top slot)
    packet
        .write_byte(5)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)?;
    packet
        .write_int(1052122)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)?;
    // Shoes
    packet
        .write_byte(7)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)?;
    packet
        .write_int(1072318)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)?;
    packet
        .write_byte(0xFF)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)?;
    // Cash shop equips
    packet
        .write_byte(0xFF)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)?;
    // Weapon
    packet
        .write_int(1302000)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)?;
    // Pet stuff...
    packet
        .write_int(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)?;
    packet
        .write_int(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)?;
    packet
        .write_int(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)?;
    Ok(())
}

pub fn write_list_char(packet: &mut Packet, character: &Character) -> Result<(), CharacterError> {
    write_char_meta(packet, &character)?;
    write_char_look(packet, &character)?;
    write_char_equips(packet, character)?;
    packet
        .write_byte(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)?;
    // Disable rank.
    packet
        .write_byte(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)?;
    Ok(())
}

pub fn write_game_char(packet: &mut Packet, character: &Character) -> Result<(), CharacterError> {
    write_char_meta(packet, character)?;
    let bl_capacity = 25;
    packet
        .write_byte(bl_capacity)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)?;
    packet
        .write_byte(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)?;
    write_inventory(packet, character)?;
    write_skills(packet, character)?;
    write_quests(packet, character)?;
    write_minigames(packet, character)?;
    write_rings(packet, character)?;
    write_teleport(packet, character)?;
    write_codex(packet, character)?;
    write_new_year_cards(packet, character)?;
    write_area_info(packet, character)?;
    packet
        .write_short(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)?;
    Ok(())
}
