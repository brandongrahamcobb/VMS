use crate::net::action::{Action, SetAction};
use crate::net::error::NetworkError;
use crate::net::packet::handler::change_map::reader::ChangeMapReader;
use crate::net::packet::handler::change_map::store::ChangeMapStore;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::model::Packet;
use crate::runtime::relay::scope::{MapScope, Scope};
use crate::runtime::session::model::Session;
use crate::runtime::state::SharedState;

pub struct ChangeMapHandler;

impl ChangeMapHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        &self,
        state: &SharedState,
        session: Session,
        packet: &Packet,
    ) -> Result<HandlerResult, NetworkError> {
        let reader: ChangeMapReader = ChangeMapReader::read_change_map_packet(packet)?;
        let store: ChangeMapStore =
            ChangeMapStore::store_change_map(state, session.clone(), reader.clone()).await?;
        let result: HandlerResult = self.build_change_map(store.clone())?;
        Ok(result)
    }

    fn build_change_map(&self, store: ChangeMapStore) -> Result<HandlerResult, NetworkError> {
        let mut result: HandlerResult = HandlerResult::new();
        result.add_action(Action::Set(SetAction::SetChar {
            char: store.char.clone(),
        }))?;
        // let packet: Packet = Packet::new_empty()
        //     .build_player_logged_in_handler_keymap_packet(store.binds.clone())?
        //     .finish();
        // result.add_action(Action::Send {
        //     packet: packet.clone(),
        //     scope: Scope::Local,
        // })?;
        dbg!(store.char.model.map_id);
        let packet: Packet = Packet::new_empty()
            .build_set_field_change_map_packet(
                store.channel.clone(),
                store.char.clone(),
                store.portal.model.pid,
            )?
            .finish();
        result.add_action(Action::Send {
            packet: packet.clone(),
            scope: Scope::Local,
        })?;
        result.add_action(Action::Set(SetAction::SetMap {
            map: store.char.map.clone(),
            scope: Scope::Local,
        }))?;
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
        Ok(result)
    }
}
