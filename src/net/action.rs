use crate::models::account::model::Account;
use crate::models::channel::model::Channel;
use crate::models::character::model::Character;
use crate::models::map::model::Map;
use crate::models::world::model::World;
use crate::net::packet::model::Packet;
use crate::runtime::scope::Scope;

pub enum Action {
    Set(SetAction),
    Send { packet: Packet, scope: Scope },
}

pub enum SetAction {
    SetMap { map: Map, scope: Scope },
    SetChannel { channel: Channel, scope: Scope },
    SetWorld { world: World, scope: Scope },
    SetAccount { acc: Account },
    SetChar { char: Character },
}
