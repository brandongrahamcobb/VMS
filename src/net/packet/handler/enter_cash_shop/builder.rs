use crate::models::account::model::AccountModel;
use crate::models::character::model::Character;
use crate::net::error::NetworkError;
use crate::net::packet::io::error::IOError::WriteError;
use crate::net::packet::model::Packet;
use crate::op::send::SendOpcode;
use crate::prelude::*;

impl Packet {
    pub fn build_enter_cash_shop_handler_packet(
        &mut self,
        acc_model: AccountModel,
        char: Character,
    ) -> Result<&mut Self, NetworkError> {
        let op = SendOpcode::SetCashShop as i16;
        self.write_short(op).map_err(WriteError)?;
        // Timestamp / Session Dummy value
        self.write_long(0).map_err(WriteError)?;
        // Flag
        self.write_byte(0).map_err(WriteError)?;
        self.build_player_logged_in_meta_part_packet(char.clone());
        self.build_cash_shop_meta(acc_model.clone())?;
        Ok(self)
    }

    fn build_cash_shop_meta(&mut self, acc_model: AccountModel) -> Result<&mut Self, NetworkError> {
        // Dummy values
        // Not MTS
        self.write_byte(0).map_err(WriteError)?;
        // Account name
        self.write_str(acc_model.username.clone())
            .map_err(WriteError)?;
        self.write_int(0).map_err(WriteError)?;
        // Special cash items
        self.write_short(0).map_err(WriteError)?;
        for _ in 0..121 {
            self.write_byte(0).map_err(WriteError)?;
        }
        for _ in 0..240 {
            self.write_int(0).map_err(WriteError)?;
        }
        self.write_int(0).map_err(WriteError)?;
        self.write_short(0).map_err(WriteError)?;
        self.write_byte(0).map_err(WriteError)?;
        self.write_int(0).map_err(WriteError)?;
        Ok(self)
    }
}
