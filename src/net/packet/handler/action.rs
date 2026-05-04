use crate::models::account::model::Account;
use crate::net::packet::packet::Packet;
use crate::runtime::session::Session;

pub enum LoginAction {
    Simple,
    SendPacket { packet: Packet },
    CloseConnection,
    CreateSession { acc: Account, hwid: String },
}

pub enum ChannelAction {
    FieldMove { movement_bytes: Vec<u8> },
    Simple,
    SendPacket { packet: Packet },
    BroadcastPacket { session: Session, packet: Packet },
    Connect { session_id: i32 },
}
