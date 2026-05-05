use crate::net::error::NetworkError;

use crate::net::packet::io::error::IOError::ReadError;
use crate::net::packet::model::Packet;
use crate::prelude::*;
use std::collections::HashMap;
use std::io::Cursor;

pub struct CloseAttackRead {
    pub count: u8,
    pub skill_id: i32,
    pub charge: i32,
    pub display: u8,
    pub toleft: u8,
    pub stance: u8,
    pub speed: u8,
    pub mob_damages: HashMap<i32, Vec<i32>>,
}

pub fn read_close_attack_packet(packet: &Packet) -> Result<CloseAttackRead, NetworkError> {
    let mut pkt_reader = Cursor::new(&packet.bytes);
    let skip = 1;
    pkt_reader.read_bytes(skip).map_err(ReadError)?;
    let count = pkt_reader.read_byte().map_err(ReadError)? as u8;
    let mobcount = count >> 4;
    let hitcount = count & 0x0F;
    let skill_id = pkt_reader.read_int().map_err(ReadError)?;
    let charge = pkt_reader.read_int().map_err(ReadError)?;
    if charge != 0 {
        pkt_reader.read_int().map_err(ReadError)?;
    }
    let display = pkt_reader.read_byte().map_err(ReadError)? as u8;
    let toleft = pkt_reader.read_byte().map_err(ReadError)? as u8;
    let stance = pkt_reader.read_byte().map_err(ReadError)? as u8;
    let skip = 1;
    pkt_reader.read_bytes(skip).map_err(ReadError)?;
    let speed = pkt_reader.read_byte().map_err(ReadError)? as u8;
    pkt_reader.read_byte().map_err(ReadError)? as u8;
    let to_left_ranged = pkt_reader.read_byte().map_err(ReadError)? as u8;
    if to_left_ranged != 0 {
        let skip = 7;
        pkt_reader.read_bytes(skip).map_err(ReadError)?;
        if skill_id == 3121004 || skill_id == 3221001 || skill_id == 5221004 {
            // 3121004 = Hurricane, 3221001 = Piercing
            // arrow, 5221004 = Rapid fire
            let skip = 4;
            pkt_reader.read_bytes(skip).map_err(ReadError)?;
        }
    } else {
        let skip = 4;
        pkt_reader.read_bytes(skip).map_err(ReadError)?;
    }
    let mut mob_damages = HashMap::new();
    for _ in 0..mobcount {
        let mob_id = pkt_reader.read_int().map_err(ReadError)?;
        let skip = 14;
        pkt_reader.read_bytes(skip).map_err(ReadError)?;
        let mut damages: Vec<i32> = Vec::new();
        for _ in 0..hitcount {
            let dmg = pkt_reader.read_int().map_err(ReadError)?;
            damages.push(dmg);
        }
        if skill_id != 5221004 {
            // Skip 4
            let skip = 4;
            pkt_reader.read_bytes(skip).map_err(ReadError)?;
        }
        mob_damages.insert(mob_id, damages);
    }
    Ok(CloseAttackRead {
        count,
        skill_id,
        charge,
        display,
        toleft,
        stance,
        speed,
        mob_damages,
    })
}
