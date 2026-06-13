/* enter_cash_shop/store.rs
 * The purpose of this module is to resolve relevant variables for entering the cash shop.
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

use std::time::Instant;

use crate::component::item::MapleItem;
use crate::component::map::InMap;
use crate::component::session::Transitioning;
use crate::message::packet::enter_cash_shop::ReadEnterCashShopRequestMessage;
use crate::message::result::HandlerResult;
use crate::resource::custom_resource::ClientMap;
use crate::system::packet::build::{codec, enter_cash_shop};
use crate::system::packet::handler::codec::load_char;
use crate::system::packet::handler::constants::CASH_SHOP_MAP_WZ;
use crate::system::system_params::{InParams, InventoryParams, SessionParams};
use action::model::Action;
use action::scope::ActionScope;
use bevy::ecs::hierarchy::ChildOf;
use bevy::ecs::message::{MessageReader, MessageWriter};
use bevy::ecs::system::{Commands, Query, Res};

pub fn handle_enter_cash_shop(
    mut commands: Commands,
    client_map: Res<ClientMap>,
    in_params: InParams,
    session_params: SessionParams,
    inv_params: InventoryParams,
    items: Query<(&MapleItem, &ChildOf)>,
    mut messages: MessageReader<ReadEnterCashShopRequestMessage>,
    mut results: MessageWriter<HandlerResult>,
) -> () {
    for msg in messages.read() {
        let Some(&client_entity) = client_map.0.get(&msg.client_id) else {
            continue;
        };
        let Ok(in_session) = in_params.in_sessions.get(client_entity) else {
            continue;
        };
        let Ok(in_acc) = in_params.in_accounts.get(client_entity) else {
            continue;
        };
        let Ok((_, acc, _)) = session_params.accounts.get(in_acc.0) else {
            continue;
        };
        let char_map = load_char::load_char_with_equips(
            vec![client_entity],
            &in_params,
            &session_params,
            &inv_params,
            items,
        );

        commands.entity(client_entity).remove::<InMap>();
        commands.entity(in_session.0).insert(Transitioning {
            map_wz: CASH_SHOP_MAP_WZ,
            started_at: Instant::now(),
        });

        for (char, equips) in char_map.iter() {
            let Ok(mut despawn_packet) = codec::player::spawn::build_despawn_player_packet(char.id)
            else {
                continue;
            };
            let Ok(mut enter_cash_shop_packet) =
                enter_cash_shop::build_enter_cash_shop_packet(acc.username.clone(), &char, &equips)
            else {
                continue;
            };
            results.write(HandlerResult {
                client_id: msg.client_id,
                actions: vec![
                    Action::HandlerAction {
                        packet: despawn_packet.finish(),
                        scope: ActionScope::Local,
                    },
                    Action::HandlerAction {
                        packet: enter_cash_shop_packet.finish(),
                        scope: ActionScope::Local,
                    },
                ],
            });
        }
    }
}
