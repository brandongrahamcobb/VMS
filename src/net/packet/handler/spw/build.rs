use crate::net::error::NetworkError;
use crate::net::packet::packet::Packet;
use crate::net::packet::error::PacketError;
use crate::net::packet::io::error::IOError::WriteError;
use crate::op::send::SendOpcode;
use crate::prelude::*;

impl Packet {
    pub fn build_spw_handler_packet(&mut self) -> Result<&mut Self, NetworkError> {
        let op = SendOpcode::CheckSpwResult as i16;
        self.write_short(op)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        self.write_byte(0) // failure
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        Ok(self)
    }
}
