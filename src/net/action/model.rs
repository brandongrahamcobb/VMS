use crate::net::packet::model::Packet;
use crate::runtime::session::Session;

pub enum LoginAction {
    Simple,
    SendLocalPacket { packet: Packet },
    CreateSession { acc_id: i32, hwid: String },
}

pub enum PlayerAction {
    FieldMove { session: Session, packet: Packet },
    Simple,
    SendLocalPacket { packet: Packet },
    EnterMap { session: Session, packet: Packet },
    ExitMap { session: Session, packet: Packet },
    Connect { session_id: i32 },
}
