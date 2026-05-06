use crate::constants::WORLDS;
use crate::models::channel::model::Channel;
use crate::models::world::model::World;
use crate::models::{channel, character, world};
use crate::net::action::{Action, PlayerAction};
use crate::net::error::NetworkError;
use crate::net::packet::handler::cc::reader::ChangeChannelReader;
use crate::net::packet::handler::cc::store::ChangeChannelStore;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::model::Packet;
use crate::runtime::error::SessionError;
use crate::runtime::scope::Scope;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;

pub struct ChangeChannelHandler;

impl ChangeChannelHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        &self,
        state: &SharedState,
        session: &Session,
        packet: &Packet,
    ) -> Result<HandlerResult, NetworkError> {
        let reader: ChangeChannelReader =
            ChangeChannelReader::new().read_change_channel_packet(packet)?;
        let store: ChangeChannelStore = ChangeChannelStore::new()
            .store_change_channel(state, session, &reader)
            .await?;
        let result: HandlerResult = self.build_change_channel_result(&store)?;
        Ok(result)
    }

    fn build_change_channel_result(
        &self,
        store: &ChangeChannelStore,
    ) -> Result<HandlerResult, NetworkError> {
        let mut result: HandlerResult = HandlerResult::new();
        result.add_action(Action::SetChannel {
            channel: store.channel.clone(),
            scope: Scope::Local,
        })?;
        let packet: Packet = Packet::new_empty()
            .build_despawn_player_handler_packet(&store.char)?
            .finish();
        result.add_action(Action::Send {
            packet: packet.clone(),
            scope: Scope::Map,
        })?;
        let packet: Packet = Packet::new_empty()
            .build_channel_change_handler_packet(&store.channel)?
            .finish();
        result.add_action(Action::Send {
            packet: packet.clone(),
            scope: Scope::Local,
        })?;
        let packet: Packet = Packet::new_empty()
            .build_spawn_player_packet(
                &store.char,
                &store.regular_equips,
                &store.cash_equips,
                &store.android_equips,
                &store.pet_equips,
            )
            .finish();
        result.add_action(Action::Send {
            packet: packet.clone(),
            scope: Scope::Map,
        })?;
        Ok(result)
    }
}
