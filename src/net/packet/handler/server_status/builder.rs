use crate::net::error::NetworkError;

use crate::net::packet::io::error::IOError::WriteError;
use crate::net::packet::model::Packet;
use crate::op::send::SendOpcode;
use crate::prelude::*;

impl Packet {
    pub fn build_server_status_handler_packet(
        &mut self,
        status: &i8,
    ) -> Result<&mut Self, NetworkError> {
        let op = SendOpcode::ServerStatus as i16;
        self.write_short(&op).map_err(WriteError)?;
        let status = *status as i16;
        self.write_short(&status) // Highly populated status!
            .map_err(WriteError)?;
        Ok(self)
    }
}
