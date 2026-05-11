use crate::models::character::model::Character;
use crate::models::shroom::channel::model::Channel;
use crate::net::error::NetworkError;
use crate::net::packet::io::error::IOError::WriteError;
use crate::net::packet::model::Packet;
use crate::op::send::SendOpcode;
use crate::prelude::*;

impl Packet {
    pub fn build_set_field_change_map_packet(
        &mut self,
        channel: Channel,
        char: Character,
        pid: i16,
    ) -> Result<&mut Self, NetworkError> {
        let op = SendOpcode::SetField as i16;
        self.write_short(op).map_err(WriteError)?;
        let channel_id = channel.model.id as i32;
        self.write_int(channel_id).map_err(WriteError)?;
        self //mode 1
            .write_byte(0)
            .map_err(WriteError)?;
        self //mode 2
            .write_byte(0)
            .map_err(WriteError)?;
        let skip: Vec<u8> = vec![0; 3];
        self.write_bytes(skip).map_err(WriteError)?;
        self.write_int(char.map.model.wz_id).map_err(WriteError)?;
        self.write_byte(pid).map_err(WriteError)?;
        Ok(self)
    }
}
