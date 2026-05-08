use crate::models::channel::model::ChannelModel;
use crate::net::error::NetworkError;
use crate::net::packet::io::error::IOError::WriteError;
use crate::net::packet::model::Packet;
use crate::op::send::SendOpcode;
use crate::prelude::*;

impl Packet {
    pub fn build_channel_change_handler_packet(
        &mut self,
        channel: ChannelModel,
        octets: [u8; 4],
    ) -> Result<&mut Self, NetworkError> {
        let op = SendOpcode::ChangeChannel as i16;
        self.write_short(op).map_err(WriteError)?;
        self.write_byte(1).map_err(WriteError)?;
        self.write_bytes(octets.to_vec()).map_err(WriteError)?;
        self.write_short(channel.port as i16).map_err(WriteError)?;
        Ok(self)
    }
}
