use crate::db::models::account::core::Account;
use crate::db::models::character::core::{Character, NewCharacter};
use crate::net::packet::core::Packet;

#[derive(Clone)]
pub enum BroadcastScope {
    Map(i32),
    MapExcludeSelf(i32),
    World,
    WorldExcludeSelf,
    Party(i32),
    Guild(i32),
    Nearby(i32, i16, i16),
}

pub enum RejectLoginReason {
    Banned,
    PendingTOS,
    Playing,
    InvalidCredentials,
}

pub enum LoginAction {
    RejectLogin {
        acc: Option<Account>,
        reason: RejectLoginReason,
    },
    AcceptLogin {
        acc: Account,
        hwid: String,
    },
    ServerStatus {
        status: i8,
    },
    ListChars {
        chars: Vec<Character>,
        char_max: i32,
        channel_id: i16,
        pic_status: i8,
        world_id: i16,
    },
    ListWorlds,
    CreateCharacter {
        character: NewCharacter,
    },
    CheckCharName {
        exists: bool,
        ign: String,
    },
}
