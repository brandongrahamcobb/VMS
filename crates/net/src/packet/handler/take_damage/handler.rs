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

use action::model::{Action, SessionAction, SetAction};
use action::scope::{MapScope, SessionScope};
use crate::packet::handler::result::HandlerResult;
use crate::packet::handler::take_damage::error::TakeDamageError;
use crate::packet::handler::take_damage::reader::TakeDamageReader;
use crate::packet::handler::take_damage::store::TakeDamageStore;
use packet::model::Packet;
use db::pool::DbPool;
use session::model::Session;

pub struct TakeDamageHandler;

impl TakeDamageHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        &self,
        pool: &DbPool,
        session: &Session,
        packet: &Packet,
    ) -> Result<HandlerResult, TakeDamageError> {
        let reader: TakeDamageReader = TakeDamageReader::read_take_damage_packet(packet)?;
        let store: TakeDamageStore =
            TakeDamageStore::store_take_damage(pool, session, &reader).await?;
        let result = self.build_take_damage_result(&store)?;
        Ok(result)
    }

    fn build_take_damage_result(
        &self,
        store: &TakeDamageStore,
    ) -> Result<HandlerResult, TakeDamageError> {
        let mut result: HandlerResult = HandlerResult::new();
        if store.hp != 0 {
            let packet = Packet::new_empty()
                .build_take_damage_packet(store.hp)?
                .finish();
            result.add_action(Action::Session(SessionAction::Send {
                packet: packet.clone(),
                scope: SessionScope::Local,
            }));
        } else {
            let packet = Packet::new_empty()
                .build_take_damage_packet(store.max_hp)?
                .finish();
            result.add_action(Action::Session(SessionAction::Send {
                packet: packet.clone(),
                scope: SessionScope::Local,
            }));
            let packet: Packet = Packet::new_empty()
                .build_despawn_player_packet(&store.char)?
                .finish();
            result.add_action(Action::Session(SessionAction::Send {
                packet: packet.clone(),
                scope: SessionScope::Map(MapScope::SameChannelSameWorld),
            }));
            let packet: Packet = Packet::new_empty()
                .build_set_field_change_map_packet(store.channel_id, store.return_map_wz, 0)?
                .finish();
            result.add_action(Action::Session(SessionAction::Send {
                packet: packet.clone(),
                scope: SessionScope::Local,
            }));
            result.add_action(Action::Session(SessionAction::Set(SetAction::SetMap {
                map_wz: store.return_map_wz,
                scope: SessionScope::Local,
            })));
        }
        Ok(result)
    }
}
