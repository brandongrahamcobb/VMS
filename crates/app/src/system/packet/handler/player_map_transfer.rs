/* player_map_transfer/store.rs
 * The purpose of this module is to resolve relevant variables for player map transfers.
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

use std::collections::HashMap;
use std::time::Instant;

use crate::component::item::MapleItem;
use crate::component::map::{InMap, MapleMap};
use crate::component::mob::{MapleMob, MobIndex};
use crate::component::portal::MaplePortal;
use crate::component::position::{MapleCurrentPosition, MapleLastPosition};
use crate::component::session::Transitioning;
use crate::message::packet::player_map_transferred::{
    PlayerMapTransferResponseMessage, ReadPlayerMapTransferRequestMessage,
};
use crate::message::result::HandlerResult;
use crate::resource::custom_resource::{ClientMap, CustomSender};
use crate::system::packet::build::codec;
use crate::system::system_params::{InParams, InventoryParams, LocationParams, SessionParams};
use action::model::Action;
use action::scope::{ActionScope, MapScope};
use bevy::ecs::hierarchy::ChildOf;
use bevy::ecs::message::{MessageReader, MessageWriter};
use bevy::ecs::system::{Commands, Query, Res};
use ipc::command::AsyncCommand;
use ipc::db_command::DatabaseCommand;

pub fn handle_player_map_transfer_request(
    command_tx: Res<CustomSender>,
    client_map: Res<ClientMap>,
    in_params: InParams,
    session_params: SessionParams,
    transitioning: Query<&Transitioning>,
    mut messages: MessageReader<ReadPlayerMapTransferRequestMessage>,
) -> () {
    for msg in messages.read() {
        let Some(&client_entity) = client_map.0.get(&msg.client_id) else {
            continue;
        };
        let Ok(in_char) = in_params.in_chars.get(client_entity) else {
            continue;
        };
        let Ok((_, char, _)) = session_params.chars.get(in_char.0) else {
            continue;
        };
        let Ok(in_session) = in_params.in_sessions.get(client_entity) else {
            continue;
        };
        let Ok(transitioning) = transitioning.get(in_session.0) else {
            continue;
        };

        command_tx
            .0
            .send(AsyncCommand::DatabaseOperation(
                DatabaseCommand::UpdateMapRequest {
                    client_id: msg.client_id,
                    char_id: char.id,
                    map_wz: transitioning.map_wz,
                },
            ))
            .unwrap();
    }
}

pub fn handle_player_map_transfer_response(
    mut commands: Commands,
    client_map: Res<ClientMap>,
    loc_params: LocationParams,
    in_params: InParams,
    session_params: SessionParams,
    inv_params: InventoryParams,
    mobs: Query<(&mut MapleMob, &ChildOf)>,
    items: Query<(&MapleItem, &ChildOf)>,
    mut messages: MessageReader<PlayerMapTransferResponseMessage>,
    mut results: MessageWriter<HandlerResult>,
) -> () {
    for msg in messages.read() {
        let Some(&client_entity) = client_map.0.get(&msg.client_id) else {
            continue;
        };
        let Ok(in_session) = in_params.in_sessions.get(client_entity) else {
            continue;
        };
        commands.entity(in_session.0).remove::<Transitioning>();
        let Ok(in_channel) = in_params.in_channels.get(client_entity) else {
            continue;
        };
        let stance: i8 = 0; //placeholder
        let effect: i8 = 0; //placeholder
        let team: i8 = -1; //placeholder
        let mode: u8 = 1; //placeholder
        let mut mobs_vec: Vec<MapleMob> = Vec::new();
        let map_entity = if let Some((map_entity, _, _)) = loc_params
            .maps
            .iter()
            .find(|(_, m, parent)| parent.0 == in_channel.0 && m.base.wz == msg.base_map.wz)
        {
            for (mob, _) in mobs.iter().filter(|(_, parent)| parent.0 == map_entity) {
                mobs_vec.push(mob.clone());
            }
            map_entity
        } else {
            let map: MapleMap = MapleMap {
                vacant: false,
                base: msg.base_map.clone(),
            };
            let mut mob_index = MobIndex::default();
            let map_entity = commands
                .spawn((map.clone(), mob_index.clone(), ChildOf(in_channel.0)))
                .id();
            for base_portal in msg.base_portals.clone() {
                let portal: MaplePortal = MaplePortal { base: base_portal };
                commands.spawn((portal, ChildOf(map_entity)));
            }
            for base_mob in msg.base_mobs.clone() {
                let mob: MapleMob = MapleMob {
                    id: mob_index.next_id(),
                    new_state: 0,
                    died_at: Instant::now(),
                    dead: false,
                    base: base_mob,
                };
                mobs_vec.push(mob.clone());
                let mob_entity = commands.spawn((mob, ChildOf(map_entity))).id();
                let curr_pos = MapleCurrentPosition {
                    x: 0,
                    y: 0,
                    fh: None,
                };
                commands.spawn((curr_pos, ChildOf(mob_entity)));
                let last_pos = MapleLastPosition { x: 0, y: 0 };
                commands.spawn((last_pos, ChildOf(mob_entity)));
            }
            for mob in mobs_vec.iter() {
                let Ok(mut spawn_mob_controller_packet) =
                    codec::mob::builder::build_spawn_mob_controller_packet(
                        mob, mode, stance, effect, team,
                    )
                else {
                    continue;
                };
                results.write(HandlerResult {
                    client_id: msg.client_id,
                    actions: vec![Action::HandlerAction {
                        packet: spawn_mob_controller_packet.finish(),
                        scope: ActionScope::Local,
                    }],
                });
            }
            map_entity
        };
        // TODO: NPC spawning
        for mob in mobs_vec.iter() {
            let Ok(mut spawn_mob_packet) =
                codec::mob::builder::build_spawn_mob_packet(mob, stance, effect, team)
            else {
                continue;
            };
            results.write(HandlerResult {
                client_id: msg.client_id,
                actions: vec![Action::HandlerAction {
                    packet: spawn_mob_packet.finish(),
                    scope: ActionScope::Local,
                }],
            });
        }
        commands.entity(client_entity).insert(InMap(map_entity));

        let Ok(in_char) = in_params.in_chars.get(client_entity) else {
            continue;
        };
        let Ok((char_entity, char, _)) = session_params.chars.get(in_char.0) else {
            continue;
        };
        let Some((inv_entity, _, _)) = inv_params
            .inventories
            .iter()
            .find(|(_, _, parent)| parent.0 == char_entity)
        else {
            continue;
        };
        let Some((equipped_tab_entity, _, _)) = inv_params
            .equipped_tabs
            .iter()
            .find(|(_, _, parent)| parent.0 == inv_entity)
        else {
            continue;
        };
        let filled_item_slots: Vec<_> = inv_params
            .filled_slots
            .iter()
            .filter(|(_, _, parent)| parent.0 == equipped_tab_entity)
            .collect();
        let mut equips_map: HashMap<i32, Vec<MapleItem>> = HashMap::new();
        let mut equips: Vec<MapleItem> = Vec::new();
        for (filled_item_slot_entity, _, _) in filled_item_slots {
            let Some((equip, _)) = items
                .iter()
                .find(|(_, parent)| parent.0 == filled_item_slot_entity)
            else {
                continue;
            };
            equips.push(equip.clone());
        }
        equips_map.insert(char.id, equips);

        let Ok(mut spawn_player_packet) =
            codec::player::spawn::build_spawn_player_packet(&char, &equips_map)
        else {
            continue;
        };
        results.write(HandlerResult {
            client_id: msg.client_id,
            actions: vec![Action::HandlerAction {
                packet: spawn_player_packet.finish(),
                scope: ActionScope::Map(MapScope::SameChannelSameWorld),
            }],
        });
        let other_clients: Vec<_> = client_map
            .0
            .iter()
            .filter(|(_, entity)| {
                in_params
                    .in_maps
                    .get(**entity)
                    .map(|im| im.0 == map_entity)
                    .unwrap_or(false)
            })
            .map(|(_, entity)| *entity)
            .collect();
        for other_client_entity in other_clients {
            let Ok(in_char) = in_params.in_chars.get(other_client_entity) else {
                continue;
            };
            let Ok((other_char_entity, other_char, _)) = session_params.chars.get(in_char.0) else {
                continue;
            };
            if other_char.id == char.id {
                continue;
            }
            let Some((inv_entity, _, _)) = inv_params
                .inventories
                .iter()
                .find(|(_, _, parent)| parent.0 == other_char_entity)
            else {
                continue;
            };
            let Some((equipped_tab_entity, _, _)) = inv_params
                .equipped_tabs
                .iter()
                .find(|(_, _, parent)| parent.0 == inv_entity)
            else {
                continue;
            };
            let filled_item_slots: Vec<_> = inv_params
                .filled_slots
                .iter()
                .filter(|(_, _, parent)| parent.0 == equipped_tab_entity)
                .collect();
            let mut equips_map: HashMap<i32, Vec<MapleItem>> = HashMap::new();
            let mut equips: Vec<MapleItem> = Vec::new();
            for (filled_item_slot_entity, _, _) in filled_item_slots {
                let Some((equip, _)) = items
                    .iter()
                    .find(|(_, parent)| parent.0 == filled_item_slot_entity)
                else {
                    continue;
                };
                equips.push(equip.clone());
            }
            equips_map.insert(other_char.id, equips);
            let Ok(mut spawn_player_packet) =
                codec::player::spawn::build_spawn_player_packet(&other_char, &equips_map)
            else {
                continue;
            };
            results.write(HandlerResult {
                client_id: msg.client_id,
                actions: vec![Action::HandlerAction {
                    packet: spawn_player_packet.finish(),
                    scope: ActionScope::Local,
                }],
            });
        }
    }
}
