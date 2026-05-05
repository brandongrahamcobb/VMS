use crate::net::error::NetworkError;
use crate::net::packet::io::error::IOError::WriteError;
use crate::net::packet::model::Packet;
use crate::op::send::SendOpcode;
use crate::prelude::*;

impl Packet {
    pub fn build_select_char_handler_packet(
        &mut self,
        char_id: &i32,
        octets: &[u8; 4],
        port: &i16,
    ) -> Result<&mut Self, NetworkError> {
        let op = SendOpcode::ServerIp as i16;
        self.write_short(&op).map_err(WriteError)?;
        self.write_short(&0).map_err(WriteError)?;
        self.write_bytes(&octets.to_vec()).map_err(WriteError)?;
        self.write_short(port).map_err(WriteError)?;
        self.write_int(char_id).map_err(WriteError)?;
        self.write_bytes(&vec![0u8; 5]).map_err(WriteError)?;
        Ok(self)
    }
}
