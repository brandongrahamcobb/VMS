use crate::net::packet::core::Packet;

pub enum LoginAction {
    Simple,
    SendPacket { packet: Packet },
    CloseConnection,
}

pub enum WorldAction {
    FieldMove { movement_bytes: Vec<u8> },
    Simple,
    SendPacket { packet: Packet },
}
