/* mob_ai/handler.rs
 * The purpose of this module is to handle mob AI.
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

use action::scope::BroadcastScope;
use action::model::{Action, BroadcastAction};
use crate::packet::handler::mob_ai::error::MobAiError;
use crate::packet::handler::mob_ai::reader::MobAiReader;
use crate::packet::handler::mob_ai::store::MobAiStore;
use crate::packet::handler::result::HandlerResult;
use packet::model::Packet;
use entity::mob::model::MobMovement;
use session::model::Session;
use state::model::SharedState;

pub struct MobAiHandler;

impl MobAiHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        &self,
        state: &SharedState,
        session: &Session,
        packet: &Packet,
    ) -> Result<HandlerResult, MobAiError> {
        let reader: MobAiReader = MobAiReader::read_mob_ai_packet(packet)?;
        let store: MobAiStore = MobAiStore::store_mob_ai(state, session, &reader).await?;
        let result: HandlerResult = self.build_mob_ai_result(&store)?;
        Ok(result)
    }

    fn build_mob_ai_result(&self, store: &MobAiStore) -> Result<HandlerResult, MobAiError> {
        let mut result = HandlerResult::new();
        let packet: Packet = Packet::new_empty()
            .build_mob_move_packet(
                store.mob_id,
                store.skill0,
                store.skill1,
                store.skill2,
                store.skill3,
                store.skill4,
                store.skillb,
                store.origin.x,
                store.origin.y,
                vec![MobMovement {
                    command: store.command,
                    x: store.next.x,
                    y: store.next.y,
                    last_x: store.last.x,
                    last_y: store.last.y,
                    fh: store.fh,
                    new_state: store.new_state,
                    duration: store.duration,
                }],
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
        Ok(result)
    }
}
