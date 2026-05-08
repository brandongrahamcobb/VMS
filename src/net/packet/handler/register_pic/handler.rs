use crate::net::action::Action;
use crate::net::error::NetworkError;
use crate::net::packet::handler::register_pic::reader::RegisterPicReader;
use crate::net::packet::handler::register_pic::store::RegisterPicStore;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::model::Packet;
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
        session: Session,
        packet: &Packet,
    ) -> Result<HandlerResult, NetworkError> {
        let reader: RegisterPicReader = RegisterPicReader::read_register_pic_packet(packet)?;
        let store: RegisterPicStore =
            RegisterPicStore::store_register_pic(state, session.clone(), reader.clone()).await?;
        let result = self.build_register_pic_result(store.clone())?;
        Ok(result)
    }

    fn build_register_pic_result(
        &self,
        store: RegisterPicStore,
    ) -> Result<HandlerResult, NetworkError> {
        let mut result: HandlerResult = HandlerResult::new();
        let packet: Packet = Packet::new_empty()
            .build_select_char_handler_packet(
                store.char.model.id,
                store.octets,
                store.channel.model.port,
            )?
            .finish();
        result.add_action(Action::Send {
            packet: packet.clone(),
            scope: Scope::Local,
        })?;
        Ok(result)
    }
}
