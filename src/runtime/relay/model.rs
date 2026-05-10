use crate::net::packet::io::{read::PacketReader, write::PacketWriter};
use crate::net::packet::model::Packet;
use crate::runtime::relay::types::shared::RuntimeRelay;
use crate::runtime::state::SharedState;
use tokio::sync::mpsc::UnboundedReceiver;

pub struct Runtime<T: RuntimeRelay> {
    pub pkt_reader: PacketReader,
    pub pkt_writer: PacketWriter,
    pub state: SharedState,
    pub relay: T,
    pub rx: UnboundedReceiver<Packet>,
}

pub struct LoginRelay {
    pub session_id: i32,
}

pub struct PlayerRelay {
    pub session_id: i32,
}
