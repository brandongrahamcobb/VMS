use crate::models::character::equipment_set::model::{CashEquipmentSet, RegularEquipmentSet};
use crate::models::character::model::Character;
use crate::net::error::NetworkError;

use crate::net::packet::io::error::IOError::WriteError;
use crate::net::packet::packet::Packet;
use crate::op::send::SendOpcode;
use crate::prelude::*;
use crate::runtime::state::SharedState;

impl Packet {
    pub async fn build_create_char_handler_packet(
        &mut self,
        state: SharedState,
        char: &Character,
        regular_equips: &RegularEquipmentSet,
        cash_equips: &CashEquipmentSet,
    ) -> Result<&mut Self, NetworkError> {
        let op = SendOpcode::NewCharacter as i16;
        self.write_short(op).map_err(WriteError)?;
        self.write_byte(0).map_err(WriteError)?;
        self.build_new_character_look_part_packet(
            state.clone(),
            &char,
            &regular_equips,
            cash_equips,
        )
        .await?;
        Ok(self)
    }

    async fn build_new_character_look_part_packet(
        &mut self,
        state: SharedState,
        char: &Character,
        regular_equips: &RegularEquipmentSet,
        cash_equips: &CashEquipmentSet,
    ) -> Result<&mut Self, NetworkError> {
        self.build_list_char_meta_part_packet(char)?;
        self.build_new_character_look_meta_part_packet(
            state.clone(),
            char,
            regular_equips,
            cash_equips,
        )
        .await?;
        self.write_byte(0).map_err(WriteError)?;
        // Disable rank.
        self.write_byte(0).map_err(WriteError)?;
        Ok(self)
    }

    async fn build_new_character_look_meta_part_packet(
        &mut self,
        state: SharedState,
        char: &Character,
        regular_equips: &RegularEquipmentSet,
        cash_equips: &CashEquipmentSet,
    ) -> Result<&mut Self, NetworkError> {
        self.write_byte(char.gender_id as u8).map_err(WriteError)?;
        self.write_byte(char.skin_id as u8).map_err(WriteError)?;
        self.write_int(char.face_id).map_err(WriteError)?;
        self.write_byte(0) // megaphone
            .map_err(WriteError)?;
        self.write_int(char.hair_id).map_err(WriteError)?;
        self.build_look_regular_equipment_part_packet(state.clone(), regular_equips)
            .await?;
        self.write_byte(0xFF).map_err(WriteError)?;
        self.build_look_cash_equipment_part_packet(state.clone(), cash_equips)
            .await?;
        self.write_byte(0xFF).map_err(WriteError)?;
        self.write_int(0) //maskedequips -111
            .map_err(WriteError)?;
        // Pet stuff...
        self.write_int(0).map_err(WriteError)?;
        self.write_int(0).map_err(WriteError)?;
        self.write_int(0).map_err(WriteError)?;
        println!("{}", self.bytes.len());
        Ok(self)
    }
}
