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

use crate::component::account::MapleAccount;
use crate::component::character::MapleCharacter;
use crate::component::map::{InMap, MapleMap};
use crate::component::session::MapleSession;
use crate::message::packet::enter_cash_shop::EnterCashShopMessage;
use crate::resource::custom_resource::{ClientMap, Sessions};
use crate::system::packet::build::enter_cash_shop;
use crate::system::packet::handler::constants::CASH_SHOP_MAP_WZ;
use crate::system::packet::handler::result::HandlerResult;
use bevy::ecs::entity::{Entity, Res};
use bevy::ecs::hierarchy::ChildOf;
use bevy::ecs::message::{MessageReader, MessageWriter};
use bevy::ecs::system::Commands;
use net::packet::model::Packet;

pub async fn handle_enter_cash_shop(
    mut commands: Commands,
    client_map: Res<ClientMap>,
    mut messages: MessageReader<EnterCashShopMessage>,
    mut results: MessageWriter<HandlerResult>,
    accounts: Query<&MapleAccount>,
    chars: Query<&MapleCharacter>,
    maps: Query<(Entity, &MapleMap, &ChildOf)>,
) -> () {
    for msg in messages.read() {
        let Some(&client_entity) = clients.0.get(&msg.client_id) else {
            continue;
        };
        let Ok(acc) = accounts.get(client_entity) else {
            continue;
        };
        let Ok(char) = chars.get(client_entity) else {
            continue;
        };
        commands.entity(client_entity).remove::<InMap>();
        let Some((map_entity, map, _)) = maps
            .iter()
            .find(|(_, m, parent)| m.wz == CASH_SHOP_MAP_WZ && parent.0 == in_channel.0)
        else {
            continue;
        };
        commands.entity(client_entity).insert(map_entity);

        let Ok(despawn_packet) = codec::player::builder::build_despawn_player_packet(char.id)
        else {
            continue;
        };
        let Ok(enter_cash_shop_packet) =
            enter_cash_shop::build_enter_cash_shop_packet(acc.username, char)
        else {
            continue;
        };

        results.write(HandlerResult {
            client_id: msg.client_id,
            actions: vec![
                Action::Session(SessionAction::Send {
                    packet: despawn_packet.finish(),
                    scope: SessionScope::Local,
                }),
                Action::Session(SessionAction::Send {
                    packet: enter_cash_shop_packet.finish(),
                    scope: SessionScope::Local,
                }),
            ],
        });
    }
}
