use crate::models::character;
use crate::net::error::NetworkError;
use crate::net::packet::handler::action::ChannelAction;
use crate::net::packet::handler::close_attack;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::packet::Packet;
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
        state: SharedState,
        session: Session,
        packet: Packet,
    ) -> Result<HandlerResult<ChannelAction>, NetworkError> {
        let read = close_attack::read::read_close_attack_packet(packet)?;
        let char_id = session
            .char_id
            .ok_or(SessionError::NoCharacterSelected(session.id))?;
        let skill_level: u8 = character::skill::query::get_skill_level_by_character_id_and_skill_id(
            state.clone(),
            char_id,
            read.skill_id,
        )
        .await
        .unwrap_or(0) as u8;
        let result = complete_close_attack(
            session,
            &char_id,
            &read.count,
            &skill_level,
            &read.skill_id,
            &read.display,
            &read.toleft,
            &read.stance,
            &read.speed,
            read.mob_damages.clone(),
        )?;
        Ok(result)
    }
}

fn complete_close_attack(
    session: Session,
    char_id: &i32,
    count: &u8,
    skill_level: &u8,
    skill_id: &i32,
    display: &u8,
    toleft: &u8,
    stance: &u8,
    speed: &u8,
    mob_damages: HashMap<i32, Vec<i32>>,
) -> Result<HandlerResult<ChannelAction>, NetworkError> {
    let mut result: HandlerResult<ChannelAction> = HandlerResult::new();
    let packet = Packet::new_empty()
        .build_close_attack_handler_packet(
            char_id,
            count,
            skill_level,
            skill_id,
            display,
            toleft,
            stance,
            speed,
            mob_damages.clone(),
        )?
        .finish();
    result.add_action(ChannelAction::BroadcastPacket { session, packet });
    Ok(result)
}
