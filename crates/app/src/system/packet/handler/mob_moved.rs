/* mob_ai/store.rs
 * The purpose of this module is to resolve relevant variables for mob AI.
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

use crate::component::channel::MapleChannel;
use crate::component::map::MapleMap;
use crate::component::world::MapleWorld;
use crate::message::packet::mob_moved::MobMovedMessage;
use crate::resource::custom_resource::ClientMap;
use crate::system::packet::build::codec;
use crate::system::packet::handler::result::HandlerResult;
use bevy::ecs::entity::Entity;
use bevy::ecs::hierarchy::ChildOf;
use bevy::ecs::message::{MessageReader, MessageWriter};
use bevy::ecs::system::{Query, Res};
use entity::map::model::Point;

pub async fn handle_mob_moved(
    client_map: Res<ClientMap>,
    mut messages: MessageReader<MobMovedMessage>,
    mut results: MessageWriter<HandlerResult>,
    positions: Query<(&mut MaplePosition, &ChildOf)>,
    in_map: Query<&InMap>,
    mobs: Query<(Entity, &mut MapleMob, &ChildOf)>,
) -> () {
    for msg in messages.read() {
        let Some(&client_entity) = client_map.0.get(&msg.client_id) else {
            continue;
        };
        let Ok(in_map) = in_map.get(client_entity) else {
            continue;
        };
        let Some((mob_entity, mob, _)) = mobs
            .iter_mut()
            .find(|(_, m, parent)| m.id == msg.mob_id && parent.0 == in_map.0);

        let mob_pos = Point {
            x: msg.origin_x,
            y: msg.origin_y,
        };
        mob.pos = mob_pos;
        mob.fh = msg.fh;
        let last_pos = Point {
            x: msg.last_x,
            y: msg.last_y,
        };
        mob.last_pos = last_pos;

        let Ok(mob_moved_packet) = codec::mob::builder::build_mob_move_packet(
            msg.mob_id,
            msg.skill0,
            msg.skill1,
            msg.skill2,
            msg.skill3,
            msg.skill4,
            msg.skillb,
            msg.origin.x,
            msg.origin.y,
            vec![MobMovement {
                command: msg.command,
                x: mob_pos.x,
                y: mob_pos.y,
                last_x: last_pos.x,
                last_y: last_pos.y,
                fh: msg.fh,
                new_state: msg.new_state,
                duration: msg.duration,
            }],
        ) else {
            continue;
        };

        results.write(HandlerResult {
            client_id: msg.client_id,
            actions: vec![Action::Broadcast(BroadcastAction::Send {
                packet: mob_moved_packet.finish(),
                scope: BroadcastScope::Map,
            })],
        });
    }
}
