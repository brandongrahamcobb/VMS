use crate::net::packet::core::Packet;

pub enum Action {
    Simple,
    SendPacket { packet: Packet },
}
