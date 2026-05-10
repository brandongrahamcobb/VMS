use crate::net::action::{Action, SetAction};
use crate::net::error::NetworkError;
use crate::net::packet::handler::cc::reader::ChangeChannelReader;
use crate::net::packet::handler::cc::store::ChangeChannelStore;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::model::Packet;
use crate::runtime::relay::scope::{MapScope, Scope};
use crate::runtime::session::model::Session;
use crate::runtime::state::SharedState;

pub struct ChangeChannelHandler;

impl ChangeChannelHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        &self,
        state: &SharedState,
        session: Session,
        packet: &Packet,
    ) -> Result<HandlerResult, NetworkError> {
        let reader: ChangeChannelReader = ChangeChannelReader::read_change_channel_packet(packet)?;
        let store: ChangeChannelStore =
            ChangeChannelStore::store_change_channel(state, session, reader.clone()).await?;
        let result: HandlerResult = self.build_change_channel_result(store.clone())?;
        Ok(result)
    }

    fn build_change_channel_result(
        &self,
        store: ChangeChannelStore,
    ) -> Result<HandlerResult, NetworkError> {
        let mut result: HandlerResult = HandlerResult::new();
        result.add_action(Action::Set(SetAction::SetChannel {
            channel: store.channel.clone(),
            scope: Scope::Local,
        }))?;
        let packet: Packet = Packet::new_empty()
            .build_despawn_player_handler_packet(store.char.clone())?
            .finish();
        result.add_action(Action::Send {
            packet: packet.clone(),
            scope: Scope::Map(MapScope::SameChannelSameWorld),
        })?;
        let packet: Packet = Packet::new_empty()
            .build_spawn_player_packet(store.char.clone())?
            .finish();
        result.add_action(Action::Send {
            packet: packet.clone(),
            scope: Scope::Map(MapScope::SameChannelSameWorld),
        })?;
        for session in store.sessions {
            let packet: Packet = Packet::new_empty()
                .build_spawn_player_packet(session.get_char()?.clone())?
                .finish();
            result.add_action(Action::Send {
                packet: packet.clone(),
                scope: Scope::Local,
            })?;
        }
        let packet: Packet = Packet::new_empty()
            .build_channel_change_handler_packet(store.channel.clone(), store.octets.clone())?
            .finish();
        result.add_action(Action::Break {
            packet: packet.clone(),
            scope: Scope::Local,
        })?;
        Ok(result)
    }
}
