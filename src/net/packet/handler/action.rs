use crate::net::packet::core::Packet;

pub enum Action {
    SendPacket { packet: Packet },
}
