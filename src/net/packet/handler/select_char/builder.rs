use crate::models::character::model::Character;
use crate::models::shroom::channel::model::Channel;
use crate::net::error::NetworkError;
use crate::net::packet::io::error::IOError::WriteError;
use crate::net::packet::model::Packet;
use crate::op::send::SendOpcode;
use crate::prelude::*;

impl Packet {
    pub fn build_select_char_handler_packet(
        &mut self,
        channel: Channel,
        char: Character,
        octets: [u8; 4],
    ) -> Result<&mut Self, NetworkError> {
        let op = SendOpcode::ServerIp as i16;
        self.write_short(op).map_err(WriteError)?;
        self.write_short(0).map_err(WriteError)?;
        self.write_bytes(octets.to_vec()).map_err(WriteError)?;
        self.write_short(channel.model.port).map_err(WriteError)?;
        self.write_int(char.model.get_id()?).map_err(WriteError)?;
        self.write_bytes(vec![0u8; 5]).map_err(WriteError)?;
        Ok(self)
    }
}
