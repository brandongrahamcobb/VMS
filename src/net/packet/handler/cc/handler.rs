use crate::constants::WORLDS;
use crate::models::channel::model::Channel;
use crate::models::world::model::World;
use crate::models::{channel, character, world};
use crate::net::action::model::{Action, PlayerAction};
use crate::net::error::NetworkError;
use crate::net::packet::handler::cc::read::ChangeChannelRead;
use crate::net::packet::handler::cc::store::ChangeChannelStore;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::model::Packet;
use crate::runtime::error::SessionError;
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
        let read = ChangeChannelRead::new().read_change_channel_packet(packet)?;
        let store = ChangeChannelStore::new()
            .store_change_channel(state, session, &read)
            .await?;
        let result = build_change_channel_result(state, session, &store).await?;
        Ok(result)
    }

    async fn build_change_channel_result(
        &self,
        _state: &SharedState,
        session: &Session,
        store: &ChangeChannelStore,
    ) -> Result<HandlerResult, NetworkError> {
        let mut result: HandlerResult = HandlerResult::new();
        let packet: Packet = Packet::new_empty()
            .build_channel_change_handler_packet(&store.channel)?
            .finish();
        result.add_action(Action::Local {
            session: session.clone(),
            packet: packet.clone(),
        });
        let packet: Packet = Packet::new_empty()
            .build_despawn_player_handler_packet(&store.char)?
            .finish();
        result.add_action(Action::Player(PlayerAction::ExitMap {
            session: session.clone(),
            packet: packet.clone(),
            source_world: store.world.clone(),
            source_channel: store.source_channel.clone(),
            source_map: store.map.clone(),
        }));
        let packet: Packet = Packet::new_empty()
            .build_spawn_player_packet(
                state,
                &store.char,
                &store.regular_equips,
                &store.cash_equips,
                &store.android_equips,
                &store.pet_equips,
            )
            .await?
            .finish();
        let target_channel = store.channel;
        result.add_action(Action::Player(PlayerAction::EnterMap {
            session: session.clone(),
            packet: packet.clone(),
            target_world: store.world.clone(),
            target_channel: store.target_channel.clone(),
            target_map: store.map.clone(),
        }));
        Ok(result)
    }
}
