use crate::net::action::Action;
use crate::net::error::NetworkError;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::handler::select_char_with_pic::reader::SelectCharWithPicReader;
use crate::net::packet::handler::select_char_with_pic::store::SelectCharWithPicStore;
use crate::net::packet::model::Packet;
use crate::runtime::error::SessionError;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;

pub struct SelectCharWithPicHandler;

impl SelectCharWithPicHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        &self,
        state: &SharedState,
        session: &Session,
        packet: &Packet,
    ) -> Result<HandlerResult, NetworkError> {
        let read: SelectCharWithPicReader =
            SelectCharWithPicReader::new().read_select_char_with_pic_packet(packet)?;
        let store: SelectCharWithPicStore =
            SelectCharWithPicStore::new().store_select_char_with_pic(state, session, &reader)?;
        let result: HandlerResult = self.build_select_char_with_pic_result(state, &store)?;
        Ok(result)
    }

    fn build_select_char_with_pic_result(
        &self,
        store: &SelectCharWithPicStore,
    ) -> Result<HandlerResult, NetworkError> {
        let mut result: HandlerResult = HandlerResult::new();
        let packet: Packet = if store.pic_status {
            result.add_action(Action::SetChar {
                char: store.char.clone(),
            })?;
            Packet::new_empty()
                .build_select_char_handler_packet(&store.char.id, &store.ip.addr, &store.ip.port)?
                .finish()
        } else {
            Packet::new_empty()
                .build_select_char_handler_failed_pic_packet()?
                .finish()
        };
        result.add_action(Action::Send {
            packet: packet.clone(),
            scope: Scope::Local,
        })?;
        Ok(result)
    }
}
