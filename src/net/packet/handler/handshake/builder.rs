use crate::net::error::NetworkError;

use crate::net::packet::io::error::IOError::WriteError;
use crate::net::packet::model::Packet;
use crate::prelude::*;

impl Packet {
    pub async fn build_handshake_packet(
        &mut self,
        recv_iv: &[u8; 4],
        send_iv: &[u8; 4],
    ) -> Result<&mut Self, NetworkError> {
        let version = settings::get_version()?;
        self.write_short(&0x0E).map_err(WriteError)?;
        self.write_short(version).map_err(WriteError)?;
        // Not sure what this part is meant to represent...
        // HeavenClient doesn't seem to care for these values but the
        // official clients do...
        self.write_short(&0).map_err(WriteError)?;
        self.write_byte(&0).map_err(WriteError)?;
        self.write_bytes(&recv_iv.to_vec()).map_err(WriteError)?;
        self.write_bytes(&send_iv.to_vec()).map_err(WriteError)?;
        self.write_byte(&8) // Locale byte
            .map_err(WriteError)?;
        Ok(self)
    }
}
