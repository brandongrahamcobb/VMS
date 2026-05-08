use crate::net::error::NetworkError;

use crate::net::packet::io::error::IOError::WriteError;
use crate::net::packet::model::Packet;
use crate::op::send::SendOpcode;
use crate::prelude::*;
use std::collections::HashMap;

impl Packet {
    pub fn build_close_attack_handler_packet(
        &mut self,
        char_id: i32,
        count: i16,
        skill_level: i16,
        skill_id: i32,
        display: i16,
        toleft: i16,
        stance: i16,
        speed: i16,
        mob_damages: HashMap<i32, Vec<i32>>,
    ) -> Result<&mut Self, NetworkError> {
        let op = SendOpcode::AttackedClose as i16;
        self.write_short(op).map_err(WriteError)?;
        self.write_int(char_id).map_err(WriteError)?;
        self.write_byte(count).map_err(WriteError)?;
        self.write_byte(skill_level).map_err(WriteError)?;
        self.write_int(skill_id).map_err(WriteError)?;
        self.write_byte(display).map_err(WriteError)?;
        self.write_byte(toleft).map_err(WriteError)?;
        self.write_byte(stance).map_err(WriteError)?;
        self.write_byte(speed).map_err(WriteError)?;
        for (mob_id, damages) in mob_damages {
            self.write_int(mob_id).map_err(WriteError)?;
            for dmg in damages {
                self.write_int(dmg).map_err(WriteError)?;
            }
        }
        Ok(self)
    }
}
