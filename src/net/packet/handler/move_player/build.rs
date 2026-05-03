use crate::net::error::NetworkError;
use crate::net::packet::packet::Packet;
use crate::net::packet::error::PacketError;
use crate::net::packet::io::error::IOError::WriteError;
use crate::op::send::SendOpcode;
use crate::prelude::*;

impl Packet {
    pub fn build_player_move_handler_packet(
        &mut self,
        char_id: i32,
        movement_bytes: &[u8],
    ) -> Result<&mut Self, NetworkError> {
        self.write_short(SendOpcode::MovePlayer as i16)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        self.write_int(char_id)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        self.write_int(0)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        self.write_bytes(movement_bytes)
            .map_err(WriteError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        Ok(self)
    }
}
