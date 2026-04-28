use crate::net::error::NetworkError;
use crate::net::packet::core::Packet;
use crate::net::packet::handler::action::LoginAction;
use crate::net::packet::handler::action::WorldAction;
use crate::net::packet::handler::cc;
use crate::net::packet::handler::char_select;
use crate::net::packet::handler::check_char_name;
use crate::net::packet::handler::create_char;
use crate::net::packet::handler::credentials;
use crate::net::packet::handler::delete_char;
use crate::net::packet::handler::list_chars;
use crate::net::packet::handler::list_worlds;
use crate::net::packet::handler::login_start;
use crate::net::packet::handler::move_player;
use crate::net::packet::handler::party_search;
use crate::net::packet::handler::play;
use crate::net::packet::handler::player_map_transfer;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::handler::server_status;
use crate::net::packet::handler::tos;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;

impl LoginHandler {
    pub async fn handle(
        self: &mut Self,
        state: SharedState,
        session: Session,
        packet: Packet,
    ) -> Result<HandlerResult<LoginAction>, NetworkError> {
        match self {
            LoginHandler::Credentials(h) => h.handle(state, session, packet).await,
            LoginHandler::TOS(h) => h.handle(state, session, packet).await,
            LoginHandler::LoginStarted(h) => h.handle(state, session, packet).await,
            LoginHandler::ServerStatus(h) => h.handle(state, session, packet).await,
            LoginHandler::ListWorlds(h) => h.handle(state, session, packet).await,
            LoginHandler::ListChars(h) => h.handle(state, session, packet).await,
            LoginHandler::CreateChar(h) => h.handle(state, session, packet).await,
            LoginHandler::CheckCharName(h) => h.handle(state, session, packet).await,
            LoginHandler::DeleteChar(h) => h.handle(state, session, packet).await,
            LoginHandler::CharSelect(h) => h.handle(state, session, packet).await,
        }
    }
}

pub enum LoginHandler {
    Credentials(credentials::CredentialsHandler),
    TOS(tos::TOSHandler),
    LoginStarted(login_start::LoginStartHandler),
    ListWorlds(list_worlds::WorldListHandler),
    ServerStatus(server_status::ServerStatusHandler),
    ListChars(list_chars::CharListHandler),
    CreateChar(create_char::CreateCharacterHandler),
    CheckCharName(check_char_name::CheckCharNameHandler),
    DeleteChar(delete_char::DeleteCharacterHandler),
    CharSelect(char_select::CharacterSelectHandler),
}

impl WorldHandler {
    pub async fn handle(
        self: &mut Self,
        state: SharedState,
        session: Session,
        packet: Packet,
    ) -> Result<HandlerResult<WorldAction>, NetworkError> {
        match self {
            WorldHandler::ChangeChannel(h) => h.handle(state, session, packet).await,
            WorldHandler::PlayerLoggedIn(h) => h.handle(state, session, packet).await,
            WorldHandler::PartySearch(h) => h.handle(state, session, packet).await,
            WorldHandler::PlayerMapTransfer(h) => h.handle(state, session, packet).await,
            WorldHandler::MovePlayer(h) => h.handle(state, session, packet).await,
        }
    }
}

pub enum WorldHandler {
    ChangeChannel(cc::ChangeChannelHandler),
    PlayerLoggedIn(play::PlayerLoggedInHandler),
    PartySearch(party_search::PartySearchHandler),
    PlayerMapTransfer(player_map_transfer::PlayerMapTransferHandler),
    MovePlayer(move_player::MovePlayerHandler),
}
