// /* app/src/system/transition.rs
//  * The purpose of this module is to provide a system for dispatching packets.
//  *
//  * Copyright (C) 2026  https://github.com/brandongrahamcobb/VMS.git
//  *
//  * This program is free software: you can redistribute it and/or modify
//  * it under the terms of the GNU Affero General Public License as published by
//  * the Free Software Foundation, either version 3 of the License, or
//  * (at your option) any later version.
//  *
//  * This program is distributed in the hope that it will be useful,
//  * but WITHOUT ANY WARRANTY; without even the implied warranty of
//  * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//  * GNU Affero General Public License for more details.
//  *
//  * You should have received a copy of the GNU Affero General Public License
//  * along with this program.  If not, see <https://www.gnu.org/licenses/>.
//  */
// use crate::component::channel::MapleChannel;
// use crate::component::world::MapleWorld;
// use bevy::ecs::entity::Entity;
// use bevy::ecs::system::Query;
// use ipc::channel::{AsyncCommand, AsyncEvent};
//
// pub fn handle_transition_system(
//     receiver: Res<CustomReceiver>,
//     command_tx: Res<CustomSender>,
//     worlds: Query<(Entity, &MapleWorld)>,
//     channels: Query<(&MapleChannel, &ChildOf)>,
// ) {
//     while let Ok(event) = receiver.0.lock().unwrap().try_recv() {
//         let AsyncEvent::ClientTransitioning {
//             client_id,
//             channel_id,
//             world_id,
//         } = event
//         else {
//             continue;
//         };
//         let Some((world_entity, _)) = worlds.iter().find(|(_, w)| w.world_id == world_id) else {
//             continue;
//         };
//         let Some((channel, _)) = channels
//             .iter()
//             .find(|(c, p)| p.0 == world_entity && c.channel_id == channel_id)
//         else {
//             continue;
//         };
//         command_tx
//             .0
//             .send(AsyncCommand::AcceptTransition {
//                 client_id,
//                 port: channel.port,
//             })
//             .unwrap();
//     }
// }
