use crate::net::action::Action;
use crate::net::error::NetworkError;
use crate::net::packet::handler::close_attack::reader::CloseAttackReader;
use crate::net::packet::handler::close_attack::store::CloseAttackStore;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::model::Packet;
use crate::runtime::scope::{MapScope, Scope};
use crate::runtime::session::model::Session;
use crate::runtime::state::SharedState;

pub struct CloseAttackHandler;

impl CloseAttackHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        &self,
        state: &SharedState,
        session: Session,
        packet: &Packet,
    ) -> Result<HandlerResult, NetworkError> {
        let reader: CloseAttackReader = CloseAttackReader::read_close_attack_packet(packet)?;
        let store: CloseAttackStore =
            CloseAttackStore::store_close_attack(state, session.clone(), reader.clone()).await?;
        let result = self.build_close_attack_result(store.clone())?;
        Ok(result)
    }

    fn build_close_attack_result(
        &self,
        store: CloseAttackStore,
    ) -> Result<HandlerResult, NetworkError> {
        let mut result: HandlerResult = HandlerResult::new();
        let packet = Packet::new_empty()
            .build_close_attack_handler_packet(
                store.char.model.get_id()?,
                store.count,
                store.skill.model.level as i16,
                store.skill.model.wz_id,
                store.display,
                store.toleft,
                store.stance,
                store.speed,
                store.mob_damages.clone(),
            )?
            .finish();
        result.add_action(Action::Send {
            packet: packet.clone(),
            scope: Scope::Local,
        })?;
        result.add_action(Action::Send {
            packet: packet.clone(),
            scope: Scope::Map(MapScope::SameChannelSameWorld),
        })?;
        Ok(result)
    }
}
