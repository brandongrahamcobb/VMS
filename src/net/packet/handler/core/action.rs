use crate::db::models::account::core::Account;
use crate::db::models::character::core::Character;
//use crate::runtime::session::SessionState;

pub enum RejectLoginReason {
    Banned,
    PendingTOS,
    Playing,
    InvalidCredentials,
}

pub enum CoreAction {
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
        channel_id: i8,
        pic_status: i8,
        world_id: i8,
    },
    ListWorlds,
    // CreateSession {
    //     acc: Account,
    //     hwid: String,
    // },
    // SessionSelectWorld {
    //     session_state: SessionState,
    //     world_id: u8,
    // },
    // SessionSelectChannel {
    //     session_state: SessionState,
    //     channel_id: u8,
    // },
}
