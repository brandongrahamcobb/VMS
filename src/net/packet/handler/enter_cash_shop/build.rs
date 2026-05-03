use crate::models::account::model::Account;
use crate::models::character::equipment_set::model::{CashEquipmentSet, RegularEquipmentSet};
use crate::models::character::model::Character;
use crate::net::error::NetworkError;
use crate::net::packet::error::PacketError;
use crate::net::packet::io::error::IOError::WriteError;
use crate::net::packet::packet::Packet;
use crate::op::send::SendOpcode;
use crate::prelude::*;
use crate::runtime::state::SharedState;

impl Packet {
    pub async fn build_enter_cash_shop_handler_packet(
        &mut self,
        state: SharedState,
        channel_id: i16,
        acc: &Account,
        char: &Character,
        regular_equips: &RegularEquipmentSet,
        cash_equips: &CashEquipmentSet,
    ) -> Result<&mut Self, NetworkError> {
        let op = SendOpcode::SetCashShop as i16;
        self.write_short(op)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        // Timestamp / Session Dummy value
        self.write_long(0)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        // Flag
        self.write_byte(0)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        self.build_play_handler_char_packet(
            state.clone(),
            char,
            channel_id,
            regular_equips,
            cash_equips,
        )
        .await?;
        self.build_cash_shop_meta(acc)?;
        Ok(self)
    }

    fn build_cash_shop_meta(&mut self, acc: &Account) -> Result<&mut Self, NetworkError> {
        // Dummy values
        // Not MTS
        self.write_byte(0)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        // Account name
        self.write_str_with_length(&acc.username)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        self.write_int(0)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        // Special cash items
        self.write_short(0)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        for _ in 0..121 {
            self.write_byte(0)
                .map_err(WriteError)
                .map_err(PacketError::from)
                .map_err(NetworkError::from)?;
        }
        for _ in 0..240 {
            self.write_int(0)
                .map_err(WriteError)
                .map_err(PacketError::from)
                .map_err(NetworkError::from)?;
        }
        self.write_int(0)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        self.write_short(0)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        self.write_byte(0)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        self.write_int(0)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        Ok(self)
    }
}
