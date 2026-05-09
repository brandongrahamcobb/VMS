use crate::models::character::model::Character;
use crate::net::error::NetworkError;
use crate::net::packet::io::error::IOError::WriteError;
use crate::net::packet::model::Packet;
use crate::op::send::SendOpcode;
use crate::prelude::*;

impl Packet {
    pub fn build_create_char_handler_packet(
        &mut self,
        char: Character,
    ) -> Result<&mut Self, NetworkError> {
        let op = SendOpcode::New as i16;
        self.write_short(op).map_err(WriteError)?;
        self.write_byte(0).map_err(WriteError)?;
        self.build_new_character_look_part_packet(char.clone())?;
        Ok(self)
    }

    fn build_new_character_look_part_packet(
        &mut self,
        char: Character,
    ) -> Result<&mut Self, NetworkError> {
        self.build_list_char_meta_part_packet(char.clone())?;
        self.build_new_character_look_meta_part_packet(char.clone())?;
        self.write_byte(0).map_err(WriteError)?;
        // Disable rank.
        self.write_byte(0).map_err(WriteError)?;
        Ok(self)
    }

    fn build_new_character_look_meta_part_packet(
        &mut self,
        char: Character,
    ) -> Result<&mut Self, NetworkError> {
        let gender_id = char.model.gender_id as i16;
        self.write_byte(gender_id).map_err(WriteError)?;
        let skin_id = char.model.skin_id as i16;
        self.write_byte(skin_id).map_err(WriteError)?;
        self.write_int(char.model.face_id).map_err(WriteError)?;
        self.write_byte(0) // megaphone
            .map_err(WriteError)?;
        self.write_int(char.model.hair_id).map_err(WriteError)?;
        self.build_look_regular_equipment_part_packet(char.clone())?;
        self.write_byte(0xFF).map_err(WriteError)?;
        self.build_look_cash_equipment_part_packet(char.clone())?;
        self.write_byte(0xFF).map_err(WriteError)?;
        self.write_int(0) //maskedequips -111
            .map_err(WriteError)?;
        // Pet stuff...
        self.write_int(0).map_err(WriteError)?;
        self.write_int(0).map_err(WriteError)?;
        self.write_int(0).map_err(WriteError)?;
        Ok(self)
    }
}
