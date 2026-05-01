use crate::models::character::error::CharacterError;
use crate::models::character::model::{CashEquipment, Character, CharacterEquipment};
use crate::models::error::ModelError;
use crate::net::packet::core::Packet;
use crate::net::packet::error::PacketError;
use crate::net::packet::io::error::IOError::WriteError;
use crate::prelude::*;
use crate::runtime::state::SharedState;

fn begin_inventory(packet: &mut Packet, char: &Character) -> Result<(), ModelError> {
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
    Ok(())
}

fn end_inventory(packet: &mut Packet) -> Result<(), ModelError> {
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

// NONE = 0,
// HAT = 1,
// FACE = 2,
// EYEACC = 3,
// EARACC = 4,
// TOP = 5,
// BOTTOM = 6,
// SHOES = 7,
// GLOVES = 8,
// CAPE = 9,
// SHIELD = 10, // TODO: Where is this now?
// WEAPON = 11,
// RING1 = 12,
// RING2 = 13,
// RING3 = 15,
// RING4 = 16,
// PENDANT1 = 17,
// TAMEDMOB = 18,	// TODO: Where is this now?
// SADDLE = 19,	// TODO: Where is this now?
// MEDAL = 49,
// BELT = 50,
// POCKET = 51
// BOOK = 52
// PENDANT2 = 53
// SHOULDER = 54
// ANDROID = 55
// EMBLEM = 56
// BADGE = 57
// SUBWEAPON = 58
// HEART = 59
// LENGTH = 60
pub fn write_char_equips(
    packet: &mut Packet,
    _char: &Character,
    char_equips: &CharacterEquipment,
) -> Result<(), ModelError> {
    if let Some(id) = char_equips.hat {
        packet
            .write_short(1)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_byte(1)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_int(id)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        write_char_item_meta(packet, id)?;
    }
    if let Some(id) = char_equips.face_acc {
        packet
            .write_short(2)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_byte(1)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_int(id)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        write_char_item_meta(packet, id)?;
    }
    if let Some(id) = char_equips.eye_acc {
        packet
            .write_short(3)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_byte(1)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_int(id)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        write_char_item_meta(packet, id)?;
    }
    if let Some(id) = char_equips.ear_acc {
        packet
            .write_short(4)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_byte(1)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_int(id)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        write_char_item_meta(packet, id)?;
    }
    if let Some(id) = char_equips.top {
        packet
            .write_short(5)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_byte(1)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_int(id)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        write_char_item_meta(packet, id)?;
    }
    if let Some(id) = char_equips.bottom {
        packet
            .write_short(6)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_byte(1)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_int(id)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        write_char_item_meta(packet, id)?;
    }
    if let Some(id) = char_equips.shoes {
        packet
            .write_short(7)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_byte(1)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_int(id)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        write_char_item_meta(packet, id)?;
    }
    if let Some(id) = char_equips.gloves {
        packet
            .write_short(8)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_byte(1)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_int(id)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        write_char_item_meta(packet, id)?;
    }
    if let Some(id) = char_equips.cape {
        packet
            .write_short(9)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_byte(1)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_int(id)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        write_char_item_meta(packet, id)?;
    }
    if let Some(id) = char_equips.shield {
        packet
            .write_short(10)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_byte(1)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_int(id)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        write_char_item_meta(packet, id)?;
    }
    if let Some(id) = char_equips.weapon {
        packet
            .write_short(11)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_byte(1)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_int(id)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        write_char_item_meta(packet, id)?;
    }
    if let Some(id) = char_equips.ring_one {
        packet
            .write_short(12)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_byte(1)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_int(id)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        write_char_item_meta(packet, id)?;
    }
    if let Some(id) = char_equips.ring_two {
        packet
            .write_short(13)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_byte(1)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_int(id)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        write_char_item_meta(packet, id)?;
    }
    if let Some(id) = char_equips.ring_three {
        packet
            .write_short(15)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_byte(1)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_int(id)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        write_char_item_meta(packet, id)?;
    }
    if let Some(id) = char_equips.ring_four {
        packet
            .write_short(16)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_byte(1)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_int(id)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        write_char_item_meta(packet, id)?;
    }
    if let Some(id) = char_equips.pendant_one {
        packet
            .write_short(17)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_byte(1)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_int(id)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        write_char_item_meta(packet, id)?;
    }
    if let Some(id) = char_equips.saddle {
        packet
            .write_short(18)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_byte(1)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_int(id)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        write_char_item_meta(packet, id)?;
    }
    if let Some(id) = char_equips.belt {
        packet
            .write_short(50)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_byte(1)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_int(id)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        write_char_item_meta(packet, id)?;
    }
    if let Some(id) = char_equips.pocket {
        packet
            .write_short(51)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_byte(1)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_int(id)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        write_char_item_meta(packet, id)?;
    }
    if let Some(id) = char_equips.pendant_two {
        packet
            .write_short(52)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_byte(1)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_int(id)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        write_char_item_meta(packet, id)?;
    }
    if let Some(id) = char_equips.shoulder {
        packet
            .write_short(54)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_byte(1)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_int(id)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        write_char_item_meta(packet, id)?;
    }
    if let Some(id) = char_equips.android {
        packet
            .write_short(55)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_byte(1)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_int(id)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        write_char_item_meta(packet, id)?;
    }
    if let Some(id) = char_equips.emblem {
        packet
            .write_short(56)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_byte(1)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_int(id)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        write_char_item_meta(packet, id)?;
    }
    if let Some(id) = char_equips.badge {
        packet
            .write_short(57)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_byte(1)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_int(id)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        write_char_item_meta(packet, id)?;
    }
    if let Some(id) = char_equips.sub_weapon {
        packet
            .write_short(58)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_byte(1)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_int(id)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        write_char_item_meta(packet, id)?;
    }
    if let Some(id) = char_equips.heart {
        packet
            .write_short(59)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_byte(1)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_int(id)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        write_char_item_meta(packet, id)?;
    }
    Ok(())
}

fn write_char_item_meta(packet: &mut Packet, id: i32) -> Result<(), ModelError> {
    const NUM_EQUIP_STATS: i8 = 15;
    packet
        .write_byte(false as u8)
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
        .write_byte(0)
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
    for _ in 0..NUM_EQUIP_STATS {
        packet
            .write_short(0)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
    }
    packet
        .write_str_with_length("")
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
        .write_byte(0)
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
        .write_int(0)
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
        .write_bytes(&[0u8; 12])
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)
        .map_err(ModelError::from)?;
    Ok(())
}

fn write_cash_item_meta(packet: &mut Packet, id: i32) -> Result<(), ModelError> {
    const NUM_EQUIP_STATS: i8 = 15;
    packet
        .write_byte(true as u8)
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
        .write_long(0)
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
        .write_byte(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)
        .map_err(ModelError::from)?;

    for _ in 0..NUM_EQUIP_STATS {
        packet
            .write_short(0)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
    }
    packet
        .write_str_with_length("")
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
        .write_bytes(&[0u8; 10])
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)
        .map_err(ModelError::from)?;
    Ok(())
}

pub fn write_cash_equips(
    packet: &mut Packet,
    _char: &Character,
    cash_equips: &CashEquipment,
) -> Result<(), ModelError> {
    if let Some(id) = cash_equips.hat {
        packet
            .write_short(101)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_byte(5)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_int(id)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        write_cash_item_meta(packet, id)?;
    }
    if let Some(id) = cash_equips.face_acc {
        packet
            .write_short(102)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_byte(5)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_int(id)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        write_cash_item_meta(packet, id)?;
    }
    if let Some(id) = cash_equips.eye_acc {
        packet
            .write_short(103)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_byte(5)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_int(id)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        write_cash_item_meta(packet, id)?;
    }
    if let Some(id) = cash_equips.ear_acc {
        packet
            .write_short(104)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_byte(5)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_int(id)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        write_cash_item_meta(packet, id)?;
    }
    if let Some(id) = cash_equips.top {
        packet
            .write_short(105)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_byte(5)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_int(id)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        write_cash_item_meta(packet, id)?;
    }
    if let Some(id) = cash_equips.bottom {
        packet
            .write_short(106)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_byte(5)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_int(id)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        write_cash_item_meta(packet, id)?;
    }
    if let Some(id) = cash_equips.shoes {
        packet
            .write_short(107)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_byte(5)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_int(id)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        write_cash_item_meta(packet, id)?;
    }
    if let Some(id) = cash_equips.gloves {
        packet
            .write_short(108)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_byte(5)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_int(id)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        write_cash_item_meta(packet, id)?;
    }
    if let Some(id) = cash_equips.cape {
        packet
            .write_short(109)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_byte(5)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_int(id)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        write_cash_item_meta(packet, id)?;
    }
    if let Some(id) = cash_equips.weapon {
        packet
            .write_short(111)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_byte(5)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_int(id)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        write_cash_item_meta(packet, id)?;
    }
    if let Some(id) = cash_equips.ring_one {
        packet
            .write_short(112)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_byte(5)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_int(id)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        write_cash_item_meta(packet, id)?;
    }
    if let Some(id) = cash_equips.ring_two {
        packet
            .write_short(113)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_byte(5)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_int(id)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        write_cash_item_meta(packet, id)?;
    }
    if let Some(id) = cash_equips.ring_three {
        packet
            .write_short(115)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_byte(5)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_int(id)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        write_cash_item_meta(packet, id)?;
    }
    if let Some(id) = cash_equips.ring_four {
        packet
            .write_short(116)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_byte(5)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_int(id)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        write_cash_item_meta(packet, id)?;
    }
    Ok(())
}

pub fn write_game_char(
    _state: SharedState,
    packet: &mut Packet,
    char: &Character,
    char_equips: &CharacterEquipment,
    cash_equips: &CashEquipment,
) -> Result<(), ModelError> {
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
    begin_inventory(packet, char)?;
    write_char_equips(packet, char, char_equips)?;
    write_cash_equips(packet, char, cash_equips)?;
    // End of equipment equipped (all id's) MUST BE ENDED WITH A SHORT 0
    packet
        .write_short(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)
        .map_err(ModelError::from)?;
    // Start of equipment inventory (negative id's) MUST BE ENDED WITH A SHORT 0
    packet
        .write_short(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)
        .map_err(ModelError::from)?;
    // Start of equipment inventory (postive id's)  MUST BE ENDED WITH A SHORT 0
    packet
        .write_short(0)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)
        .map_err(ModelError::from)?;
    end_inventory(packet)?;
    write_skills(packet, char)?;
    write_quests(packet, char)?;
    write_minigames(packet, char)?;
    write_rings(packet, char)?;
    write_teleport(packet, char)?;
    write_codex(packet, char)?;
    write_new_year_cards(packet, char)?;
    write_area_info(packet, char)?;
    Ok(())
}
