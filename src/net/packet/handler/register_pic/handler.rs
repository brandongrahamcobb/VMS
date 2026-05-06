use crate::config::settings;
use crate::inc::helpers;
use crate::models::channel;
use crate::models::channel::model::Channel;
use crate::net::action::LoginAction;
use crate::net::error::NetworkError;
use crate::net::packet::handler::register_pic;
use crate::net::packet::handler::register_pic::reader::RegisterPicReader;
use crate::net::packet::handler::register_pic::store::RegisterPicStore;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::model::Packet;
use crate::runtime::error::SessionError;
use crate::runtime::scope::Scope;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;

pub struct RegisterPicHandler;

impl RegisterPicHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        &self,
        state: &SharedState,
        session: &Session,
        packet: &Packet,
    ) -> Result<HandlerResult, NetworkError> {
        let reader: RegisterPicReader =
            RegisterPicReader::new().read_register_pic_packet(packet)?;
        let store: RegisterPicStore =
            RegisterPicStore::new().store_register_pic(state, session, &reader)?;
        let result = self.build_register_pic_result(&store)?;
        Ok(result)
    }

    fn build_register_pic_result(
        &self,
        store: &RegisterPicStore,
    ) -> Result<HandlerResult, NetworkError> {
        let mut result: HandlerResult = HandlerResult::new();
        let packet: Packet = Packet::new_empty()
            .build_select_char_handler_packet(&store.char.id, &store.ip)?
            .finish();
        result.add_action(Action::Send {
            packet: packet.clone(),
            scope: Scope::Local,
        });
        Ok(result)
    }
}
