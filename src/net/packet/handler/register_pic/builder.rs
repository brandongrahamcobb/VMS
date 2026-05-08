use crate::net::error::NetworkError;
use crate::net::packet::io::error::IOError::WriteError;
use crate::net::packet::model::Packet;
use crate::op::send::SendOpcode;
use crate::prelude::*;

impl Packet {
    pub fn build_register_pic_handler_packet(&mut self) -> Result<&mut Self, NetworkError> {
        let op = SendOpcode::CheckSpwResult as i16;
        self.write_short(op).map_err(WriteError)?;
        self.write_byte(1) // 0 for failure
            .map_err(WriteError)?;
        Ok(self)
    }
}
