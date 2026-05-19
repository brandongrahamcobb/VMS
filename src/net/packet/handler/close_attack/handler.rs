/* close_attack/handler.rs
 * The purpose of this module is to handle close attacks.
 *
 * Copyright (C) 2026  https://github.com/brandongrahamcobb/VMS.git
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use crate::net::action::{Action, BroadcastAction, SessionAction};
use crate::net::packet::handler::close_attack::error::CloseAttackError;
use crate::net::packet::handler::close_attack::reader::CloseAttackReader;
use crate::net::packet::handler::close_attack::store::CloseAttackStore;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::model::Packet;
use crate::runtime::relay::scope::{BroadcastScope, SessionScope};
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
        session: &Session,
        packet: &Packet,
    ) -> Result<HandlerResult, CloseAttackError> {
        let reader: CloseAttackReader = CloseAttackReader::read_close_attack_packet(packet)?;
        let store: CloseAttackStore =
            CloseAttackStore::store_close_attack(state, session, &reader).await?;
        let result = self.build_close_attack_result(&store)?;
        Ok(result)
    }

    fn build_close_attack_result(
        &self,
        store: &CloseAttackStore,
    ) -> Result<HandlerResult, CloseAttackError> {
        let mut result: HandlerResult = HandlerResult::new();
        let packet = Packet::new_empty()
            .build_close_attack_packet(
                store.char_id,
                store.count,
                store.skill_level,
                store.skill_wz,
                store.display,
                store.toleft,
                store.stance,
                store.speed,
                store.mob_damages.clone(),
            )?
            .finish();
        result.add_action(Action::Broadcast(BroadcastAction::Send {
            packet: packet.clone(),
            scope: BroadcastScope::Map {
                world_id: store.world_id,
                channel_id: store.channel_id,
                map_wz: store.map_wz,
            },
        }));
        for (mob_id, hp_percent) in store.hp_updates.clone() {
            let packet = Packet::new_empty()
                .build_mob_damage_show_hp_packet(mob_id, hp_percent)?
                .finish();
            result.add_action(Action::Session(SessionAction::Send {
                packet: packet.clone(),
                scope: SessionScope::Local,
            }));
        }
        for mob_id in store.dead_mobs.clone() {
            let packet = Packet::new_empty().build_kill_mob_packet(mob_id)?.finish();
            result.add_action(Action::Broadcast(BroadcastAction::Send {
                packet: packet.clone(),
                scope: BroadcastScope::Map {
                    world_id: store.world_id,
                    channel_id: store.channel_id,
                    map_wz: store.map_wz,
                },
            }));
        }
        Ok(result)
    }
}
