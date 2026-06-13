/* app/src/system/packet/handler/mob_moved.rs
 * The purpose of this module is to process mob moved system messages.
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

use crate::component::mob::MapleMob;
use crate::message::packet::mob_moved::ReadMobMovedRequestMessage;
use crate::message::result::HandlerResult;
use crate::resource::custom_resource::ClientMap;
use crate::system::packet::build::codec;
use crate::system::system_params::{InParams, PositionParams};
use action::model::Action;
use action::scope::{ActionScope, MapScope};
use base::mob::MobMovement;
use bevy::ecs::entity::Entity;
use bevy::ecs::hierarchy::ChildOf;
use bevy::ecs::message::{MessageReader, MessageWriter};
use bevy::ecs::system::{Query, Res};

pub fn handle_mob_moved(
    client_map: Res<ClientMap>,
    in_params: InParams,
    mut pos_params: PositionParams,
    mut mobs: Query<(Entity, &mut MapleMob, &ChildOf)>,
    mut messages: MessageReader<ReadMobMovedRequestMessage>,
    mut results: MessageWriter<HandlerResult>,
) -> () {
    for msg in messages.read() {
        let Some(&client_entity) = client_map.0.get(&msg.client_id) else {
            continue;
        };
        let Ok(in_map) = in_params.in_maps.get(client_entity) else {
            continue;
        };
        let Some((mob_entity, _, _)) = mobs
            .iter_mut()
            .find(|(_, m, parent)| m.id == msg.mob_id && parent.0 == in_map.0)
        else {
            continue;
        };
        let Some((mut curr_pos, _)) = pos_params
            .curr_positions
            .iter_mut()
            .find(|(_, parent)| parent.0 == mob_entity)
        else {
            continue;
        };
        let Some((mut last_pos, _)) = pos_params
            .last_positions
            .iter_mut()
            .find(|(_, parent)| parent.0 == mob_entity)
        else {
            continue;
        };

        curr_pos.x = msg.origin_x;
        curr_pos.y = msg.origin_y;
        curr_pos.fh = Some(msg.fh);
        last_pos.x = msg.last_x;
        last_pos.y = msg.last_y;

        let Ok(mut mob_moved_packet) = codec::mob::builder::build_mob_move_packet(
            msg.mob_id,
            msg.skill0,
            msg.skill1,
            msg.skill2,
            msg.skill3,
            msg.skill4,
            msg.skillb,
            msg.origin_x,
            msg.origin_y,
            vec![MobMovement {
                command: msg.command,
                x: curr_pos.x,
                y: curr_pos.y,
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
            actions: vec![Action::HandlerAction {
                packet: mob_moved_packet.finish(),
                scope: ActionScope::Map(MapScope::SameChannelSameWorld),
            }],
        });
    }
}
