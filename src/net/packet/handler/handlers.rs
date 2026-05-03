use crate::net::error::NetworkError;
use crate::net::packet::handler::action::ChannelAction;
use crate::net::packet::handler::action::LoginAction;
use crate::net::packet::handler::cc;
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
use crate::net::packet::handler::play;
use crate::net::packet::handler::player_map_transfer;
use crate::net::packet::handler::register_pic;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::handler::select_char;
use crate::net::packet::handler::select_char_with_pic;
use crate::net::packet::handler::server_status;
use crate::net::packet::handler::tos;
use crate::net::packet::packet::Packet;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;

impl LoginHandler {
    pub async fn handle(
        &mut self,
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
    Credentials(credentials::handler::CredentialsHandler),
    TOS(tos::handler::TOSHandler),
    LoginStarted(login_start::handler::LoginStartHandler),
    ListWorlds(list_worlds::handler::WorldListHandler),
    ServerStatus(server_status::handler::ServerStatusHandler),
    ListChars(list_chars::handler::CharListHandler),
    CreateChar(create_char::handler::CreateCharacterHandler),
    CheckCharName(check_char_name::handler::CheckCharNameHandler),
    DeleteChar(delete_char::handler::DeleteCharacterHandler),
    CharSelect(select_char::handler::CharacterSelectHandler),
    RegisterPic(register_pic::handler::RegisterPicHandler),
    CharSelectWithPic(select_char_with_pic::handler::SelectCharWithPicHandler),
}

impl ChannelHandler {
    pub async fn handle(
        &mut self,
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
    ChangeChannel(cc::handler::ChangeChannelHandler),
    PlayerLoggedIn(play::handler::PlayerLoggedInHandler),
    PartySearch(party_search::handler::PartySearchHandler),
    PlayerMapTransfer(player_map_transfer::handler::PlayerMapTransferHandler),
    MovePlayer(move_player::handler::MovePlayerHandler),
    EnterCashShop(enter_cash_shop::handler::EnterCashShopHandler),
}
