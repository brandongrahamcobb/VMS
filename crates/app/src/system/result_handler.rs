/* app/src/system/result_handler.rs
 * The purpose of this module is to provide a system for sending packets to scoped clients.
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

use crate::message::result::HandlerResult;
use crate::resource::custom_resource::{ClientMap, CustomSender};
use crate::system::system_params::{InParams, LocationParams};

use action::model::Action;
use action::scope::{ActionScope, ChannelScope, MapScope};
use bevy::ecs::message::MessageReader;
use bevy::ecs::system::Res;
use ipc::command::AsyncCommand;

pub fn result_handler_system(
    command_tx: Res<CustomSender>,
    client_map: Res<ClientMap>,
    loc_params: LocationParams,
    in_params: InParams,
    mut messages: MessageReader<HandlerResult>,
) {
    for msg in messages.read() {
        let Some(&client_entity) = client_map.0.get(&msg.client_id) else {
            continue;
        };
        for action in msg.actions.iter() {
            match action {
                Action::HandlerAction { packet, scope } => match scope {
                    ActionScope::Global => {
                        for (client_id, _) in client_map.0.iter() {
                            command_tx
                                .0
                                .send(AsyncCommand::SendPacket {
                                    client_id: *client_id,
                                    packet: packet.clone(),
                                })
                                .unwrap();
                        }
                    }
                    ActionScope::World => {
                        let Ok(in_world) = in_params.in_worlds.get(client_entity) else {
                            continue;
                        };
                        let client_ids = client_map
                            .0
                            .iter()
                            .filter(|(_, entity)| {
                                in_params
                                    .in_worlds
                                    .get(**entity)
                                    .map(|iw| iw.0 == in_world.0)
                                    .unwrap_or(false)
                            })
                            .map(|(&client_id, _)| client_id);
                        for client_id in client_ids {
                            command_tx
                                .0
                                .send(AsyncCommand::SendPacket {
                                    client_id,
                                    packet: packet.clone(),
                                })
                                .unwrap();
                        }
                    }
                    ActionScope::Channel(channel_scope) => {
                        let Ok(in_channel) = in_params.in_channels.get(client_entity) else {
                            continue;
                        };
                        let Ok((_, _, channel_child_of)) = loc_params.channels.get(in_channel.0)
                        else {
                            continue;
                        };
                        let channel_world_entity = channel_child_of.parent();
                        match channel_scope {
                            ChannelScope::SameWorld => {
                                let client_ids: Vec<_> = client_map
                                    .0
                                    .iter()
                                    .filter(|(_, entity)| {
                                        in_params
                                            .in_channels
                                            .get(**entity)
                                            .and_then(|ic| loc_params.channels.get(ic.0))
                                            .map(|(_, _, child_of)| {
                                                child_of.parent() == channel_world_entity
                                            })
                                            .unwrap_or(false)
                                    })
                                    .map(|(&client_id, _)| client_id)
                                    .collect();
                                for client_id in client_ids {
                                    command_tx
                                        .0
                                        .send(AsyncCommand::SendPacket {
                                            client_id,
                                            packet: packet.clone(),
                                        })
                                        .unwrap();
                                }
                            }
                            ChannelScope::AllWorlds => {
                                let client_ids = client_map
                                    .0
                                    .iter()
                                    .filter(|(_, entity)| {
                                        in_params
                                            .in_channels
                                            .get(**entity)
                                            .map(|ic| ic.0 == in_channel.0)
                                            .unwrap_or(false)
                                    })
                                    .map(|(&client_id, _)| client_id);
                                for client_id in client_ids {
                                    command_tx
                                        .0
                                        .send(AsyncCommand::SendPacket {
                                            client_id,
                                            packet: packet.clone(),
                                        })
                                        .unwrap();
                                }
                            }
                        }
                    }
                    ActionScope::Map(map_scope) => {
                        let Ok(in_map) = in_params.in_maps.get(client_entity) else {
                            continue;
                        };
                        let Ok((_, _, map_child_of)) = loc_params.maps.get(in_map.0) else {
                            continue;
                        };
                        let map_channel_entity = map_child_of.parent();
                        let Ok((_, _, channel_child_of)) =
                            loc_params.channels.get(map_channel_entity)
                        else {
                            continue;
                        };
                        let map_world_entity = channel_child_of.parent();
                        match map_scope {
                            MapScope::SameChannelSameWorld => {
                                let client_ids: Vec<_> = client_map
                                    .0
                                    .iter()
                                    .filter(|(_, entity)| {
                                        in_params
                                            .in_maps
                                            .get(**entity)
                                            .map(|im| im.0 == in_map.0)
                                            .unwrap_or(false)
                                    })
                                    .map(|(&client_id, _)| client_id)
                                    .collect();
                                for client_id in client_ids {
                                    command_tx
                                        .0
                                        .send(AsyncCommand::SendPacket {
                                            client_id,
                                            packet: packet.clone(),
                                        })
                                        .unwrap();
                                }
                            }
                            MapScope::AllChannelsSameWorld => {
                                let client_ids: Vec<_> = client_map
                                    .0
                                    .iter()
                                    .filter(|(_, entity)| {
                                        in_params
                                            .in_maps
                                            .get(**entity)
                                            .and_then(|im| loc_params.maps.get(im.0))
                                            .map(|(_, _, child_of)| {
                                                child_of.parent() == map_channel_entity
                                            })
                                            .unwrap_or(false)
                                    })
                                    .map(|(&client_id, _)| client_id)
                                    .collect();
                                for client_id in client_ids {
                                    command_tx
                                        .0
                                        .send(AsyncCommand::SendPacket {
                                            client_id,
                                            packet: packet.clone(),
                                        })
                                        .unwrap();
                                }
                            }
                            MapScope::AllChannelsAllWorlds => {
                                let client_ids: Vec<_> = client_map
                                    .0
                                    .iter()
                                    .filter(|(_, entity)| {
                                        in_params
                                            .in_maps
                                            .get(**entity)
                                            .and_then(|im| loc_params.maps.get(im.0))
                                            .and_then(|(_, _, child_of)| {
                                                loc_params.channels.get(child_of.parent())
                                            })
                                            .map(|(_, _, child_of)| {
                                                child_of.parent() == map_world_entity
                                            })
                                            .unwrap_or(false)
                                    })
                                    .map(|(&client_id, _)| client_id)
                                    .collect();
                                for client_id in client_ids {
                                    command_tx
                                        .0
                                        .send(AsyncCommand::SendPacket {
                                            client_id,
                                            packet: packet.clone(),
                                        })
                                        .unwrap();
                                }
                            }
                        }
                    }
                    ActionScope::Local => {
                        command_tx
                            .0
                            .send(AsyncCommand::SendPacket {
                                client_id: msg.client_id,
                                packet: packet.clone(),
                            })
                            .unwrap();
                    }
                },
                _ => {}
            }
        }
    }
}
