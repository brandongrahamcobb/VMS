/* list_chars/store.rs
 * The purpose of this module is to resolve relevant variables for character listing.
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

use crate::component::account::InAccount;
use crate::component::account::MapleAccount;
use crate::component::channel::{InChannel, MapleChannel};
use crate::component::character::MapleCharacter;
use crate::component::inventory::{MapleEquippedTab, MapleInventory};
use crate::component::item::MapleItem;
use crate::component::map::{InMap, MapleMap};
use crate::component::slot::MapleFilledItemSlot;
use crate::component::world::{InWorld, MapleWorld};
use crate::message::packet::list_chars::ListCharsSuccessMessage;
use crate::message::packet::list_chars::ReadListCharsRequestMessage;
use crate::message::result::HandlerResult;
use crate::resource::custom_resource::{ClientMap, CustomSender};
use crate::system::packet::build::list_chars;
use crate::system::packet::handler::codec::spawn_char;
use action::model::{Action, SessionAction};
use action::scope::SessionScope;
use bevy::ecs::entity::Entity;
use bevy::ecs::hierarchy::ChildOf;
use bevy::ecs::message::{MessageReader, MessageWriter};
use bevy::ecs::system::{Commands, Query, Res};
use config::settings;
use ipc::asyncronous::command::AsyncCommand;
use ipc::asyncronous::db_command::DatabaseCommand;

pub enum PicStatus {
    Disabled = 2,
    AlreadyRegistered = 1,
    NeedsToRegister = 0,
}

pub fn handle_load_char_slots(
    commands: &mut Commands,
    command_tx: CustomSender,
    client_map: Res<ClientMap>,
    worlds: Query<(Entity, &MapleWorld)>,
    channels: Query<(Entity, &MapleChannel, &ChildOf)>,
    accounts: Query<&MapleAccount>,
    in_accounts: Query<(Entity, &InAccount)>,
    mut messages: MessageReader<ReadListCharsRequestMessage>,
) -> () {
    for msg in messages.read() {
        let Some(&client_entity) = client_map.0.get(&msg.client_id) else {
            continue;
        };
        let Ok((in_acc_entity, _)) = in_accounts.get(client_entity) else {
            continue;
        };
        let Ok(acc) = accounts.get(in_acc_entity) else {
            continue;
        };
        let Some((world_entity, _)) = worlds.iter().find(|(_, w)| w.id == msg.world_id) else {
            continue;
        };
        commands.spawn(InWorld(world_entity));
        let Some((channel_entity, _, _)) = channels
            .iter()
            .find(|(_, c, parent)| c.id == msg.channel_id && parent.0 == world_entity)
        else {
            continue;
        };
        commands.spawn(InChannel(channel_entity));

        command_tx
            .0
            .lock()
            .unwrap()
            .send(AsyncCommand::DatabaseOperation(
                DatabaseCommand::ListCharsRequest {
                    client_id: msg.client_id,
                    acc_id: acc.id,
                    channel_id: msg.channel_id,
                    world_id: msg.world_id,
                },
            ))
            .unwrap();
    }
}

pub fn handle_list_chars(
    commands: &mut Commands,
    client_map: Res<ClientMap>,
    accounts: Query<(Entity, &MapleAccount)>,
    in_accounts: Query<(Entity, &InAccount)>,
    chars: Query<(Entity, &MapleCharacter, &ChildOf)>,
    maps: Query<&MapleMap>,
    in_maps: Query<(Entity, &InMap)>,
    items: Query<(&MapleItem, &ChildOf)>,
    inventories: Query<(Entity, &MapleInventory)>,
    equipped_tabs: Query<(Entity, &MapleEquippedTab)>,
    filled_slots: Query<(Entity, &MapleFilledItemSlot)>,
    mut messages: MessageReader<ListCharsSuccessMessage>,
    mut results: MessageWriter<HandlerResult>,
) -> () {
    for msg in messages.read() {
        let Some(&client_entity) = client_map.0.get(&msg.client_id) else {
            continue;
        };
        let Ok((in_acc_entity, _)) = in_accounts.get(client_entity) else {
            continue;
        };
        let Ok((acc_entity, acc)) = accounts.get(in_acc_entity) else {
            continue;
        };
        for char_model in msg.char_models {
            let Some(char_id) = char_model.id else {
                continue;
            };
            let char: MapleCharacter = MapleCharacter::from(char_model);
            commands.spawn((char, ChildOf(acc_entity)));
        }
        let chars: Vec<_> = chars
            .iter()
            .filter(|(_, _, parent)| parent.0 == acc_entity)
            .collect();
        spawn_char::spawn_char(
            commands,
            chars,
            &msg.equipped_item_model_map,
            &msg.equip_item_model_map,
            &msg.use_item_model_map,
            &msg.etc_item_model_map,
            &msg.setup_item_model_map,
            &msg.cash_item_model_map,
            &msg.equip_tab_inv_capacity_map,
            &msg.use_tab_inv_capacity_map,
            &msg.etc_tab_inv_capacity_map,
            &msg.setup_tab_inv_capacity_map,
            &msg.cash_tab_inv_capacity_map,
        );

        let mut pic_status: i16 = PicStatus::Disabled as i16;
        let Ok(use_pic) = settings::get_pic_required() else {
            continue;
        };
        if acc.pic.clone().is_some() {
            if use_pic {
                pic_status = PicStatus::AlreadyRegistered as i16;
            }
        } else {
            pic_status = PicStatus::NeedsToRegister as i16;
        };

        let Ok(mut list_chars_packet) = list_chars::build_list_chars_packet(
            chars,
            items,
            inventories,
            equipped_tabs,
            filled_slots,
            maps,
            msg.channel_id,
            msg.slots,
            pic_status,
            msg.world_id,
        ) else {
            continue;
        };
        results.write(HandlerResult {
            client_id: msg.client_id,
            actions: vec![Action::Session(SessionAction::Send {
                packet: list_chars_packet.finish(),
                scope: SessionScope::Local,
            })],
        });
    }
}
