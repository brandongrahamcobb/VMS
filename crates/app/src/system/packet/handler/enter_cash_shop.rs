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

use crate::component::item::MapleItem;
use crate::component::map::InMap;
use crate::message::packet::enter_cash_shop::ReadEnterCashShopRequestMessage;
use crate::message::result::HandlerResult;
use crate::resource::custom_resource::ClientMap;
use crate::system::packet::build::{codec, enter_cash_shop};
use crate::system::packet::handler::constants::CASH_SHOP_MAP_WZ;
use crate::system::system_params::{InParams, InventoryParams, LocationParams, SessionParams};
use action::model::Action;
use action::scope::ActionScope;
use bevy::ecs::hierarchy::ChildOf;
use bevy::ecs::message::{MessageReader, MessageWriter};
use bevy::ecs::system::{Commands, Query, Res};

pub fn handle_enter_cash_shop(
    mut commands: Commands,
    client_map: Res<ClientMap>,
    loc_params: LocationParams,
    in_params: InParams,
    mut session_params: SessionParams,
    inv_params: InventoryParams,
    items: Query<(&MapleItem, &ChildOf)>,
    mut messages: MessageReader<ReadEnterCashShopRequestMessage>,
    mut results: MessageWriter<HandlerResult>,
) -> () {
    for msg in messages.read() {
        let Some(&client_entity) = client_map.0.get(&msg.client_id) else {
            continue;
        };
        let Ok((_, mut session)) = session_params.sessions.get_mut(client_entity) else {
            continue;
        };
        let Ok((in_channel_entity, _)) = in_params.in_channels.get(client_entity) else {
            continue;
        };
        let Ok((channel_entity, _, _)) = loc_params.channels.get(in_channel_entity) else {
            continue;
        };
        let Ok((in_acc_entity, _)) = in_params.in_accounts.get(client_entity) else {
            continue;
        };
        let Ok((_, acc, _)) = session_params.accounts.get(in_acc_entity) else {
            continue;
        };
        let Ok((in_char_entity, _)) = in_params.in_chars.get(client_entity) else {
            continue;
        };
        let Ok((char_entity, char, _)) = session_params.chars.get(in_char_entity) else {
            continue;
        };
        let Ok((inv_entity, _)) = inv_params.inventories.get(char_entity) else {
            continue;
        };
        let Ok((equipped_tab_entity, _)) = inv_params.equipped_tabs.get(inv_entity) else {
            continue;
        };
        let Ok((filled_slot_entity, _)) = inv_params.filled_slots.get(equipped_tab_entity) else {
            continue;
        };
        let equips: Vec<_> = items
            .iter()
            .filter(|(_, parent)| parent.0 == filled_slot_entity)
            .collect();

        session.transitioning = true;

        commands.entity(client_entity).remove::<InMap>();
        let Some((map_entity, map, _)) = loc_params
            .maps
            .iter()
            .find(|(_, m, parent)| m.base.wz == CASH_SHOP_MAP_WZ && parent.0 == channel_entity)
        else {
            continue;
        };
        commands.entity(client_entity).insert(InMap(map_entity));

        let Ok(mut despawn_packet) = codec::player::builder::build_despawn_player_packet(char.id)
        else {
            continue;
        };
        let Ok(mut enter_cash_shop_packet) =
            enter_cash_shop::build_enter_cash_shop_packet(acc.username.clone(), char, equips, map)
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
