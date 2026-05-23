/* packet/model.rs
 * The purpose of this module is to provide a packet model.
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

use crate::constants::{INVALID_OPCODE, MAX_PACKET_LENGTH};
use std::io::Result;
use std::io::Write;

#[derive(Clone, Debug, Default)]
pub struct Packet {
    pub bytes: Vec<u8>,
}

impl Packet {
    pub fn new(buffer: &Vec<u8>) -> Packet {
        if buffer.len() > MAX_PACKET_LENGTH as usize {
            panic!(
                "Packet with length {} exceeded max packet length {}",
                buffer.len(),
                MAX_PACKET_LENGTH
            );
        }
        Packet {
            bytes: buffer.clone(),
        }
    }

    pub fn new_empty() -> Packet {
        let bytes = vec![];
        Packet { bytes }
    }

    pub fn opcode(&self) -> i16 {
        if self.bytes.len() > 1 {
            let opcode: i16 = (self.bytes[0] as i16) | ((self.bytes[1] as i16) << 8);
            if opcode >= 0 { opcode } else { INVALID_OPCODE }
        } else {
            INVALID_OPCODE
        }
    }

    pub fn len(&self) -> i16 {
        (self.bytes.len() - 2) as i16
    }

    pub fn finish(&mut self) -> Packet {
        std::mem::take(self)
    }
}

impl Write for Packet {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.bytes.write(buf)
    }

    fn flush(&mut self) -> Result<()> {
        self.bytes.flush()
    }
}
