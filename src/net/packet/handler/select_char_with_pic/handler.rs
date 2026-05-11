use crate::net::action::{Action, SetAction};
use crate::net::error::NetworkError;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::handler::select_char_with_pic::reader::SelectCharWithPicReader;
use crate::net::packet::handler::select_char_with_pic::store::SelectCharWithPicStore;
use crate::net::packet::model::Packet;
use crate::runtime::relay::scope::Scope;
use crate::runtime::session::model::Session;
use crate::runtime::state::SharedState;

pub struct SelectCharWithPicHandler;

impl SelectCharWithPicHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        &self,
        state: &SharedState,
        session: Session,
        packet: &Packet,
    ) -> Result<HandlerResult, NetworkError> {
        let reader: SelectCharWithPicReader =
            SelectCharWithPicReader::read_select_char_with_pic_packet(packet)?;
        let store: SelectCharWithPicStore = SelectCharWithPicStore::store_select_char_with_pic(
            state,
            session.clone(),
            reader.clone(),
        )
        .await?;
        let result: HandlerResult = self.build_select_char_with_pic_result(store.clone())?;
        Ok(result)
    }

    fn build_select_char_with_pic_result(
        &self,
        store: SelectCharWithPicStore,
    ) -> Result<HandlerResult, NetworkError> {
        let mut result: HandlerResult = HandlerResult::new();
        let packet: Packet = if store.pic_status {
            result.add_action(Action::Set(SetAction::SetChar {
                char: store.char.clone(),
            }))?;
            Packet::new_empty()
                .build_select_char_handler_packet(
                    store.channel.clone(),
                    store.char.clone(),
                    store.octets,
                )?
                .finish()
        } else {
            Packet::new_empty()
                .build_select_char_handler_failed_pic_packet()?
                .finish()
        };
        result.add_action(Action::Break {
            packet: packet.clone(),
            scope: Scope::Local,
        })?;
        Ok(result)
    }
}
