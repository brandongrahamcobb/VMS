use crate::net::error::NetworkError;

use crate::net::packet::io::error::IOError::WriteError;
use crate::net::packet::model::Packet;
use crate::op::send::SendOpcode;
use crate::prelude::*;

impl Packet {
    pub fn build_select_char_handler_failed_pic_packet(
        &mut self,
    ) -> Result<&mut Self, NetworkError> {
        let op = SendOpcode::CheckSpwResult as i16;
        self.write_short(op).map_err(WriteError)?;
        self.write_byte(0) // 0 for failure, anything else for success
            .map_err(WriteError)?;
        Ok(self)
    }
}
