use crate::models::account::model::Account;
use crate::models::character::model::Character;
use crate::models::shroom::channel::model::Channel;
use crate::models::shroom::map::model::Map;
use crate::models::shroom::world::model::World;
use crate::net::packet::model::Packet;
use crate::runtime::relay::scope::Scope;

pub enum Action {
    Break { packet: Packet, scope: Scope },
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
