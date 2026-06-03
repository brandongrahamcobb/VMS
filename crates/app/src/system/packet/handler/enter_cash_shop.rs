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

use crate::component::account::{InAccount, MapleAccount};
use crate::component::channel::{InChannel, MapleChannel};
use crate::component::character::{InChar, MapleCharacter};
use crate::component::inventory::{MapleEquippedTab, MapleInventory};
use crate::component::item::MapleItem;
use crate::component::map::{InMap, MapleMap};
use crate::component::session::{InSession, MapleSession};
use crate::component::slot::MapleFilledItemSlot;
use crate::message::packet::enter_cash_shop::ReadEnterCashShopRequestMessage;
use crate::message::result::HandlerResult;
use crate::resource::custom_resource::ClientMap;
use crate::system::packet::build::{codec, enter_cash_shop};
use crate::system::packet::handler::constants::CASH_SHOP_MAP_WZ;
use action::model::{Action, SessionAction};
use action::scope::SessionScope;
use bevy::ecs::entity::Entity;
use bevy::ecs::hierarchy::ChildOf;
use bevy::ecs::message::{MessageReader, MessageWriter};
use bevy::ecs::system::{Commands, Query, Res};

pub fn handle_enter_cash_shop(
    commands: &mut Commands,
    client_map: Res<ClientMap>,
    mut sessions: Query<&mut MapleSession>,
    in_sessions: Query<(Entity, &InSession)>,
    channels: Query<(Entity, &MapleChannel)>,
    in_channels: Query<(Entity, &InChannel)>,
    maps: Query<(Entity, &MapleMap, &ChildOf)>,
    in_maps: Query<(Entity, &InMap)>,
    accounts: Query<&MapleAccount>,
    in_accounts: Query<(Entity, &InAccount)>,
    chars: Query<(Entity, &MapleCharacter)>,
    in_chars: Query<(Entity, &InChar)>,
    items: Query<(&MapleItem, &ChildOf)>,
    inventories: Query<(Entity, &MapleInventory)>,
    equipped_tabs: Query<(Entity, &MapleEquippedTab)>,
    filled_slots: Query<(Entity, &MapleFilledItemSlot)>,
    mut messages: MessageReader<ReadEnterCashShopRequestMessage>,
    mut results: MessageWriter<HandlerResult>,
) -> () {
    for msg in messages.read() {
        let Some(&client_entity) = client_map.0.get(&msg.client_id) else {
            continue;
        };
        let Ok((in_session_entity, _)) = in_sessions.get(client_entity) else {
            continue;
        };
        let Ok(mut session) = sessions.get_mut(in_session_entity) else {
            continue;
        };
        let Ok((in_channel_entity, _)) = in_channels.get(client_entity) else {
            continue;
        };
        let Ok((channel_entity, _)) = channels.get(in_channel_entity) else {
            continue;
        };
        let Ok((in_map_entity, _)) = in_maps.get(client_entity) else {
            continue;
        };
        let Ok((_, map, _)) = maps.get(in_map_entity) else {
            continue;
        };
        let Ok((in_acc_entity, _)) = in_accounts.get(client_entity) else {
            continue;
        };
        let Ok(acc) = accounts.get(in_acc_entity) else {
            continue;
        };
        let Ok((in_char_entity, _)) = in_chars.get(client_entity) else {
            continue;
        };
        let Ok((char_entity, char)) = chars.get(in_char_entity) else {
            continue;
        };
        let Ok((inv_entity, _)) = inventories.get(char_entity) else {
            continue;
        };
        let Ok((equipped_tab_entity, _)) = equipped_tabs.get(inv_entity) else {
            continue;
        };
        let Ok((filled_slot_entity, _)) = filled_slots.get(equipped_tab_entity) else {
            continue;
        };
        let equips: Vec<_> = items
            .iter()
            .filter(|(_, parent)| parent.0 == filled_slot_entity)
            .collect();

        session.transitioning = true;

        commands.entity(client_entity).remove::<InMap>();
        let Some((map_entity, _, _)) = maps
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
