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
        let skip: Vec<u8> = vec![0; 1];
        self.write_bytes(skip.clone()).map_err(WriteError)?;
        self.write_byte(skill_level).map_err(WriteError)?;
        self.write_int(skill_id).map_err(WriteError)?;
        self.write_byte(display).map_err(WriteError)?;
        self.write_byte(toleft).map_err(WriteError)?;
        self.write_byte(stance).map_err(WriteError)?;
        self.write_byte(speed).map_err(WriteError)?;
        self.write_bytes(skip.clone()).map_err(WriteError)?;
        for (mob_id, damages) in mob_damages {
            self.write_int(mob_id).map_err(WriteError)?;
            self.write_bytes(skip.clone()).map_err(WriteError)?;
            let meso_explosion_skill_wz_id: i32 = 4211006;
            if skill_id == meso_explosion_skill_wz_id {
                let placeholder_number_of_mesos: i16 = 15;
                let max_hits: i16 =
                    get_max_number_of_meso_explosion_hits(skill_level, placeholder_number_of_mesos);
                self.write_byte(max_hits).map_err(WriteError)?;
            }
            for dmg in damages {
                self.write_int(dmg).map_err(WriteError)?;
            }
        }
        Ok(self)
    }
}

fn get_max_number_of_meso_explosion_hits(skill_level: i16, number_of_meso_chunks: i16) -> i16 {
    let skill_max: i16 = match skill_level {
        1 => 5,
        2 | 3 => 6,
        4 | 5 => 7,
        6 | 7 => 8,
        8 | 9 => 9,
        10 | 11 => 10,
        12 | 13 => 11,
        14 | 15 => 12,
        16 | 17 => 13,
        18 | 19 => 14,
        20 => 15,
        _ => 0,
    };
    if number_of_meso_chunks < skill_max {
        return number_of_meso_chunks;
    } else {
        return skill_max;
    }
}
