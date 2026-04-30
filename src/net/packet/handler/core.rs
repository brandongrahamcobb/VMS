use crate::net::error::NetworkError;
use crate::net::packet::core::Packet;
use crate::net::packet::handler::action::ChannelAction;
use crate::net::packet::handler::action::LoginAction;
use crate::net::packet::handler::cc;
use crate::net::packet::handler::char_select;
use crate::net::packet::handler::check_char_name;
use crate::net::packet::handler::create_char;
use crate::net::packet::handler::credentials;
use crate::net::packet::handler::delete_char;
use crate::net::packet::handler::enter_cash_shop;
use crate::net::packet::handler::list_chars;
use crate::net::packet::handler::list_worlds;
use crate::net::packet::handler::login_start;
use crate::net::packet::handler::move_player;
use crate::net::packet::handler::party_search;
use crate::net::packet::handler::pic;
use crate::net::packet::handler::play;
use crate::net::packet::handler::player_map_transfer;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::handler::server_status;
use crate::net::packet::handler::spw;
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
            LoginHandler::RegisterPic(h) => h.handle(state, session, packet).await,
            LoginHandler::CharSelectWithPic(h) => h.handle(state, session, packet).await,
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
    RegisterPic(spw::SpwHandler),
    CharSelectWithPic(pic::PicHandler),
}

impl ChannelHandler {
    pub async fn handle(
        self: &mut Self,
        state: SharedState,
        session: Session,
        packet: Packet,
    ) -> Result<HandlerResult<ChannelAction>, NetworkError> {
        match self {
            ChannelHandler::ChangeChannel(h) => h.handle(state, session, packet).await,
            ChannelHandler::PlayerLoggedIn(h) => h.handle(state, session, packet).await,
            ChannelHandler::PartySearch(h) => h.handle(state, session, packet).await,
            ChannelHandler::PlayerMapTransfer(h) => h.handle(state, session, packet).await,
            ChannelHandler::MovePlayer(h) => h.handle(state, session, packet).await,
            ChannelHandler::EnterCashShop(h) => h.handle(state, session, packet).await,
        }
    }
}

pub enum ChannelHandler {
    ChangeChannel(cc::ChangeChannelHandler),
    PlayerLoggedIn(play::PlayerLoggedInHandler),
    PartySearch(party_search::PartySearchHandler),
    PlayerMapTransfer(player_map_transfer::PlayerMapTransferHandler),
    MovePlayer(move_player::MovePlayerHandler),
    EnterCashShop(enter_cash_shop::EnterCashShopHandler),
}
