use crate::net::error::NetworkError;

use crate::net::packet::io::error::IOError::WriteError;
use crate::net::packet::packet::Packet;
use crate::prelude::*;

impl Packet {
    pub async fn build_handshake_packet(
        &mut self,
        recv_iv: &Vec<u8>,
        send_iv: &Vec<u8>,
        version: &i16,
    ) -> Result<&mut Self, NetworkError> {
        self.write_short(0x0E).map_err(WriteError)?;
        self.write_short(*version).map_err(WriteError)?;
        // Not sure what this part is meant to represent...
        // HeavenClient doesn't seem to care for these values but the
        // official clients do...
        self.write_short(0).map_err(WriteError)?;
        self.write_byte(0).map_err(WriteError)?;
        self.write_bytes(&recv_iv).map_err(WriteError)?;
        self.write_bytes(&send_iv).map_err(WriteError)?;
        self.write_byte(8) // Locale byte
            .map_err(WriteError)?;
        Ok(self)
    }
}
