use crate::net::packet::packet::Packet;

pub enum LoginAction {
    Simple,
    SendPacket { packet: Packet },
    CloseConnection,
}

pub enum ChannelAction {
    FieldMove { movement_bytes: Vec<u8> },
    Simple,
    SendPacket { packet: Packet },
}
