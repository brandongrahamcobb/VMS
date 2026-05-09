use crate::config::settings;
use crate::models::account::model::Account;
use crate::net::error::NetworkError;
use crate::net::packet::io::error::IOError::WriteError;
use crate::net::packet::model::Packet;
use crate::op::send::SendOpcode;
use crate::prelude::*;
use std::time::UNIX_EPOCH;

impl Packet {
    pub fn build_credentials_handler_failed_login_packet(
        &mut self,
        status: i16,
    ) -> Result<&mut Self, NetworkError> {
        let op = SendOpcode::AccountStatus as i16;
        self.write_short(op).map_err(WriteError)?;
        self.write_byte(status).map_err(WriteError)?;
        self.write_byte(0).map_err(WriteError)?;
        self.write_int(0).map_err(WriteError)?;
        Ok(self)
    }

    pub fn build_credentials_handler_successful_login_packet(
        &mut self,
        acc: Account,
    ) -> Result<&mut Self, NetworkError> {
        let pin_required = settings::get_pin_required()? as i16;
        let opcode = SendOpcode::AccountStatus as i16;
        let acc_id = acc.model.get_id()? as i32;
        let gender_id = acc.model.gender_id as i16;
        let account_name = acc.model.username.clone();
        let created_at: i64 = acc
            .model
            .get_created_at()?
            .duration_since(UNIX_EPOCH)?
            .as_secs()
            .try_into()?;
        self.write_short(opcode).map_err(WriteError)?;
        self.write_int(0).map_err(WriteError)?;
        self.write_short(0).map_err(WriteError)?;
        self.write_int(acc_id).map_err(WriteError)?;
        self.write_byte(gender_id).map_err(WriteError)?;
        self.write_byte(0).map_err(WriteError)?;
        self.write_byte(0).map_err(WriteError)?;
        self.write_byte(0).map_err(WriteError)?;
        self.write_str_with_length(account_name).map_err(WriteError)?;
        self.write_byte(0).map_err(WriteError)?;
        self.write_byte(0).map_err(WriteError)?;
        self.write_long(0).map_err(WriteError)?;
        self.write_long(created_at).map_err(WriteError)?;
        self.write_int(1).map_err(WriteError)?;
        self.write_byte(pin_required).map_err(WriteError)?;
        self.write_byte(1).map_err(WriteError)?;
        Ok(self)
    }
}
