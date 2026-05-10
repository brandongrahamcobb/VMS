use crate::models::account::model::Account;
use crate::models::character::model::Character;
use crate::net::error::NetworkError;
use crate::net::packet::io::error::IOError::WriteError;
use crate::net::packet::model::Packet;
use crate::op::send::SendOpcode;
use crate::prelude::*;

impl Packet {
    pub fn build_chat_text_handler_packet(
        &mut self,
        acc: Account,
        char: Character,
        msg: String,
        show: i16,
    ) -> Result<&mut Self, NetworkError> {
        let op = SendOpcode::ChatText as i16;
        self.write_short(op).map_err(WriteError)?;
        self.write_int(char.model.get_id()?).map_err(WriteError)?;
        self.write_byte(acc.model.admin as i16)
            .map_err(WriteError)?;
        self.write_str_with_length(msg.clone())
            .map_err(WriteError)?;
        self.write_byte(show).map_err(WriteError)?;
        Ok(self)
    }
}
