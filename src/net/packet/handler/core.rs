use crate::net::error::NetworkError;
use crate::net::packet::core::Packet;
use crate::net::packet::handler::action::Action;
use crate::net::packet::handler::cc;
use crate::net::packet::handler::char_select;
use crate::net::packet::handler::check_char_name;
use crate::net::packet::handler::create_char;
use crate::net::packet::handler::credentials;
use crate::net::packet::handler::list_chars;
use crate::net::packet::handler::list_worlds;
use crate::net::packet::handler::login_start;
use crate::net::packet::handler::play;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::handler::server_status;
use crate::net::packet::handler::tos;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;

impl PacketHandler {
    pub async fn handle(
        self: &mut Self,
        state: SharedState,
        session: Session,
        packet: Packet,
    ) -> Result<HandlerResult<Action>, NetworkError> {
        match self {
            PacketHandler::Credentials(h) => h.handle(state, session, packet).await,
            PacketHandler::TOS(h) => h.handle(state, session, packet).await,
            PacketHandler::LoginStarted(h) => h.handle(state, session, packet).await,
            PacketHandler::ListWorlds(h) => h.handle(state, session, packet).await,
            PacketHandler::ServerStatus(h) => h.handle(state, session, packet).await,
            PacketHandler::ListChars(h) => h.handle(state, session, packet).await,
            PacketHandler::CreateChar(h) => h.handle(state, session, packet).await,
            PacketHandler::CheckCharName(h) => h.handle(state, session, packet).await,
            PacketHandler::CharSelect(h) => h.handle(state, session, packet).await,
            PacketHandler::ChangeChannel(h) => h.handle(state, session, packet).await,
            PacketHandler::PlayerLoggedIn(h) => h.handle(state, session, packet).await,
        }
    }
}

pub enum PacketHandler {
    Credentials(credentials::CredentialsHandler),
    TOS(tos::TOSHandler),
    LoginStarted(login_start::LoginStartHandler),
    ListWorlds(list_worlds::WorldListHandler),
    ServerStatus(server_status::ServerStatusHandler),
    ListChars(list_chars::CharListHandler),
    CreateChar(create_char::CreateCharacterHandler),
    CheckCharName(check_char_name::CheckCharNameHandler),
    CharSelect(char_select::CharacterSelectHandler),
    ChangeChannel(cc::ChangeChannelHandler),
    PlayerLoggedIn(play::PlayerLoggedInHandler),
}
