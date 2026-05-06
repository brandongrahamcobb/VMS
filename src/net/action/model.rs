use crate::models::channel::model::Channel;
use crate::models::map::model::Map;
use crate::models::world::model::World;
use crate::net::packet::model::Packet;
use crate::runtime::session::Session;

pub enum Action {
    Simple,
    Local { packet: Packet },
    Login(LoginAction),
    Player(PlayerAction),
}

pub enum LoginAction {
    CreateSession { acc_id: i32, hwid: String },
}

pub enum PlayerAction {
    FieldMove {
        session: Session,
        packet: Packet,
    },
    EnterMap {
        session: Session,
        packet: Packet,
        target_world: World,
        target_channel: Channel,
        target_map: Map,
    },
    ExitMap {
        session: Session,
        packet: Packet,
        source_world: World,
        source_channel: Channel,
        source_map: Map,
    },
    Connect {
        session: Session,
    },
    CleanSession {
        session: Session,
    },
}
