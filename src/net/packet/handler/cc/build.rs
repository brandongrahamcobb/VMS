use crate::config::settings;
use crate::inc::helpers;
use crate::models::channel::model::Channel;
use crate::net::error::NetworkError;
use crate::net::packet::error::PacketError;
use crate::net::packet::io::error::IOError::WriteError;
use crate::net::packet::packet::Packet;
use crate::op::send::SendOpcode;
use crate::prelude::*;

impl Packet {
    pub fn build_channel_change_handler_packet(
        &mut self,
        channel: &Channel,
    ) -> Result<&mut Self, NetworkError> {
        let addr = settings::get_address()?;
        let octets = helpers::convert_to_ip_array(addr);
        let op = SendOpcode::ChangeChannel as i16;
        self.write_short(op)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        self.write_byte(1)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        self.write_bytes(&octets)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        self.write_short(channel.port)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        Ok(self)
    }
}
