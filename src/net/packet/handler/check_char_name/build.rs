use crate::net::error::NetworkError;

use crate::net::packet::io::error::IOError::WriteError;
use crate::net::packet::packet::Packet;
use crate::op::send::SendOpcode;
use crate::prelude::*;

impl Packet {
    pub fn build_check_char_name_handler_packet(
        &mut self,
        exists: &bool,
        ign: &str,
    ) -> Result<&mut Self, NetworkError> {
        let op = SendOpcode::CharNameResponse as i16;
        self.write_short(op).map_err(WriteError)?;
        self.write_str_with_length(ign).map_err(WriteError)?;
        self.write_byte(*exists as u8).map_err(WriteError)?;
        Ok(self)
    }
}
