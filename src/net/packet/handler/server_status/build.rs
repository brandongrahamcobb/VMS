use crate::net::error::NetworkError;
use crate::net::packet::error::PacketError;
use crate::net::packet::io::error::IOError::WriteError;
use crate::net::packet::packet::Packet;
use crate::op::send::SendOpcode;
use crate::prelude::*;

impl Packet {
    pub fn build_server_status_handler_packet(
        &mut self,
        status: i8,
    ) -> Result<&mut Self, NetworkError> {
        let op = SendOpcode::ServerStatus as i16;
        self.write_short(op)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        self.write_short(status as i16) // Highly populated status!
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        Ok(self)
    }
}
