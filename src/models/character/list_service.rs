use crate::inc::helpers;
use crate::models::character::error::CharacterError;
use crate::models::character::model::{CashEquipment, Character, CharacterEquipment};
use crate::models::character::service::write_char_meta;
use crate::models::error::ModelError;
use crate::net::packet::core::Packet;
use crate::net::packet::error::PacketError;
use crate::net::packet::io::error::IOError::WriteError;
use crate::prelude::*;

pub fn finish_equips(packet: &mut Packet) -> Result<(), ModelError> {
    packet
        .write_int(0) //maskedequips -111
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

pub fn write_list_char(
    packet: &mut Packet,
    char: &Character,
    char_equips: &CharacterEquipment,
    cash_equips: &CashEquipment,
) -> Result<(), ModelError> {
    write_char_meta(packet, char)?;
    write_char_look(packet, char, char_equips, cash_equips)?;
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

pub fn write_char_look(
    packet: &mut Packet,
    char: &Character,
    char_equips: &CharacterEquipment,
    cash_equips: &CashEquipment,
) -> Result<(), ModelError> {
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
        .write_byte(0) // megaphone
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
    write_char_equips(packet, char, char_equips)?;
    packet
        .write_byte(0xFF)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)
        .map_err(ModelError::from)?;
    write_cash_equips(packet, char, cash_equips)?;
    packet
        .write_byte(0xFF)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(CharacterError::from)
        .map_err(ModelError::from)?;
    finish_equips(packet)?;
    Ok(())
}

pub fn write_char_equips(
    packet: &mut Packet,
    _char: &Character,
    char_equips: &CharacterEquipment,
) -> Result<(), ModelError> {
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
    // Begin equipped
    if char_equips.hat.is_some() {
        packet
            .write_byte(1)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_int(char_equips.hat.unwrap())
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
    }
    if char_equips.face_acc.is_some() {
        packet
            .write_byte(2)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_int(char_equips.face_acc.unwrap())
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
    }
    if char_equips.eye_acc.is_some() {
        packet
            .write_byte(3)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_int(char_equips.eye_acc.unwrap())
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
    }
    if char_equips.ear_acc.is_some() {
        packet
            .write_byte(4)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_int(char_equips.ear_acc.unwrap())
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
    }
    if char_equips.top.is_some() {
        packet
            .write_byte(5)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_int(char_equips.top.unwrap())
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
    }
    if char_equips.bottom.is_some() {
        packet
            .write_byte(6)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_int(char_equips.bottom.unwrap())
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
    }
    if char_equips.shoes.is_some() {
        packet
            .write_byte(7)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_int(char_equips.shoes.unwrap())
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
    }
    if char_equips.gloves.is_some() {
        packet
            .write_byte(8)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_int(char_equips.gloves.unwrap())
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
    }
    if char_equips.cape.is_some() {
        packet
            .write_byte(9)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_int(char_equips.cape.unwrap())
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
    }
    if char_equips.shield.is_some() {
        packet
            .write_byte(10)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_int(char_equips.shield.unwrap())
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
    }
    if char_equips.weapon.is_some() {
        packet
            .write_byte(11)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_int(char_equips.weapon.unwrap())
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
    }
    if char_equips.ring_one.is_some() {
        packet
            .write_byte(12)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_int(char_equips.ring_one.unwrap())
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
    }
    if char_equips.ring_two.is_some() {
        packet
            .write_byte(13)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_int(char_equips.ring_two.unwrap())
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
    }
    if char_equips.ring_three.is_some() {
        packet
            .write_byte(15)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_int(char_equips.ring_three.unwrap())
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
    }
    if char_equips.ring_four.is_some() {
        packet
            .write_byte(16)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_int(char_equips.ring_four.unwrap())
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
    }
    if char_equips.pendant_one.is_some() {
        packet
            .write_byte(17)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_int(char_equips.pendant_one.unwrap())
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
    }
    if char_equips.saddle.is_some() {
        packet
            .write_byte(18)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_int(char_equips.saddle.unwrap())
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
    }
    if char_equips.belt.is_some() {
        packet
            .write_byte(50)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_int(char_equips.belt.unwrap())
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
    }
    if char_equips.pocket.is_some() {
        packet
            .write_byte(51)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_int(char_equips.pocket.unwrap())
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
    }
    if char_equips.pendant_two.is_some() {
        packet
            .write_byte(52)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_int(char_equips.pendant_two.unwrap())
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
    }
    if char_equips.shoulder.is_some() {
        packet
            .write_byte(54)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_int(char_equips.shoulder.unwrap())
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
    }
    if char_equips.android.is_some() {
        packet
            .write_byte(55)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_int(char_equips.android.unwrap())
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
    }
    if char_equips.emblem.is_some() {
        packet
            .write_byte(56)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_int(char_equips.emblem.unwrap())
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
    }
    if char_equips.badge.is_some() {
        packet
            .write_byte(57)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_int(char_equips.badge.unwrap())
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
    }
    if char_equips.sub_weapon.is_some() {
        packet
            .write_byte(58)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_int(char_equips.sub_weapon.unwrap())
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
    }
    if char_equips.heart.is_some() {
        packet
            .write_byte(59)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_int(char_equips.heart.unwrap())
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
    }
    Ok(())
}

pub fn write_cash_equips(
    packet: &mut Packet,
    _char: &Character,
    cash_equips: &CashEquipment,
) -> Result<(), ModelError> {
    if cash_equips.hat.is_some() {
        packet
            .write_byte(1 + 100)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_int(cash_equips.hat.unwrap())
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
    }
    if cash_equips.face_acc.is_some() {
        packet
            .write_byte(2 + 100)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_int(cash_equips.face_acc.unwrap())
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
    }
    if cash_equips.eye_acc.is_some() {
        packet
            .write_byte(3 + 100)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_int(cash_equips.eye_acc.unwrap())
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
    }
    if cash_equips.ear_acc.is_some() {
        packet
            .write_byte(4 + 100)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_int(cash_equips.ear_acc.unwrap())
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
    }
    if cash_equips.top.is_some() {
        packet
            .write_byte(5 + 100)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_int(cash_equips.top.unwrap())
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
    }
    if cash_equips.bottom.is_some() {
        packet
            .write_byte(6 + 100)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_int(cash_equips.bottom.unwrap())
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
    }
    if cash_equips.shoes.is_some() {
        packet
            .write_byte(7 + 100)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_int(cash_equips.shoes.unwrap())
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
    }
    if cash_equips.gloves.is_some() {
        packet
            .write_byte(8 + 100)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_int(cash_equips.gloves.unwrap())
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
    }
    if cash_equips.cape.is_some() {
        packet
            .write_byte(9 + 100)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_int(cash_equips.cape.unwrap())
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
    }
    if cash_equips.weapon.is_some() {
        packet
            .write_byte(11 + 100)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_int(cash_equips.weapon.unwrap())
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
    }
    if cash_equips.ring_one.is_some() {
        packet
            .write_byte(12 + 100)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_int(cash_equips.ring_one.unwrap())
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
    }
    if cash_equips.ring_two.is_some() {
        packet
            .write_byte(13 + 100)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_int(cash_equips.ring_two.unwrap())
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
    }
    if cash_equips.ring_three.is_some() {
        packet
            .write_byte(15 + 100)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_int(cash_equips.ring_three.unwrap())
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
    }
    if cash_equips.ring_four.is_some() {
        packet
            .write_byte(16 + 100)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
        packet
            .write_int(cash_equips.ring_four.unwrap())
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(CharacterError::from)
            .map_err(ModelError::from)?;
    }
    Ok(())
}
