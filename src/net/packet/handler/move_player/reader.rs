use crate::net::error::NetworkError;
use crate::net::packet::model::Packet;

const MOVEMENT_HEADER_LEN: usize = 9;

#[derive(Clone)]
pub struct MovePlayerReader {
    pub movement_bytes: Vec<u8>,
    pub too_short: bool,
    pub empty: bool,
}

impl MovePlayerReader {
    pub fn read_move_player_packet(packet: &Packet) -> Result<Self, NetworkError> {
        let mut too_short: bool = false;
        if packet.bytes.len() <= 2 + MOVEMENT_HEADER_LEN {
            too_short = true;
        }
        let mut empty: bool = false;
        let movement_bytes = &packet.bytes[(2 + MOVEMENT_HEADER_LEN)..];
        if movement_bytes.is_empty() || movement_bytes[0] == 0 {
            empty = true;
        }
        Ok(Self {
            movement_bytes: movement_bytes.to_vec(),
            too_short,
            empty,
        })
    }
}
