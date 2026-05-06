use crate::models::character;
use crate::net::action::model::{Action, PlayerAction};
use crate::net::error::NetworkError;
use crate::net::packet::handler::close_attack;
use crate::net::packet::handler::close_attack::read::CloseAttackRead;
use crate::net::packet::handler::close_attack::store::CloseAttackStore;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::model::Packet;
use crate::runtime::error::SessionError;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;
use std::collections::HashMap;

pub struct CloseAttackHandler;

impl CloseAttackHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        &self,
        state: &SharedState,
        session: &Session,
        packet: &Packet,
    ) -> Result<HandlerResult, NetworkError> {
        let read = CloseAttackRead::new().read_close_attack_packet(packet)?;
        let store = CloseAttackStore::new()
            .store_close_attack(state, &session, &read)
            .await?;
        let result = self.build_close_attack_result(state, session, &store)?;
        Ok(result)
    }

    fn build_close_attack_result(
        &self,
        _state: &SharedState,
        session: &Session,
        store: &CloseAttackStore,
    ) -> Result<HandlerResult, NetworkError> {
        let mut result: HandlerResult = HandlerResult::new();
        let packet = Packet::new_empty()
            .build_close_attack_handler_packet(
                &store.char.id,
                &store.count,
                &store.skill.level,
                &store.skill.wz_id,
                &store.display,
                &store.toleft,
                &store.stance,
                &store.speed,
                &store.mob_damages,
            )?
            .finish();
        result.add_action(Action::Local {
            session: session.clone(),
            packet: packet.clone(),
        });
        result.add_action(Action::Player(PlayerAction::FieldMove {
            session: session.clone(),
            packet: packet.clone(),
        }));
        Ok(result)
    }
}
