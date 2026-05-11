use crate::models::character::model::Character;
use crate::net::error::NetworkError;

use crate::net::packet::io::error::IOError::WriteError;
use crate::net::packet::model::Packet;
use crate::op::send::SendOpcode;
use crate::prelude::*;

impl Packet {
    pub fn build_delete_char_handler_packet(
        &mut self,
        char: Character,
        status: bool,
    ) -> Result<&mut Self, NetworkError> {
        let op = SendOpcode::DeleteCharacter as i16;
        self.write_short(op).map_err(WriteError)?;
        self.write_int(char.model.get_id()?).map_err(WriteError)?;
        let status = status as i16;
        self.write_byte(status).map_err(WriteError)?;
        Ok(self)
    }
}
