/* app/src/system/handler/change_map.rs
 * The purpose of this module is to handle changing map system messages.
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

use crate::component::map::InMap;
use crate::component::portal::MaplePortal;
use crate::component::session::Transitioning;
use crate::message::packet::change_map::ReadChangeMapRequestMessage;
use crate::message::result::HandlerResult;
use crate::resource::custom_resource::ClientMap;
use crate::system::packet::handler::result::change_map_result;
use crate::system::system_params::{InParams, LocationParams, SessionParams};
use bevy::ecs::hierarchy::ChildOf;
use bevy::ecs::message::{MessageReader, MessageWriter};
use bevy::ecs::system::{Commands, Query, Res};

pub fn handle_map_change(
    mut commands: Commands,
    client_map: Res<ClientMap>,
    loc_params: LocationParams,
    in_params: InParams,
    session_params: SessionParams,
    portals: Query<(&MaplePortal, &ChildOf)>,
    mut messages: MessageReader<ReadChangeMapRequestMessage>,
    mut results: MessageWriter<HandlerResult>,
) -> () {
    for msg in messages.read() {
        let Some(&client_entity) = client_map.0.get(&msg.client_id) else {
            continue;
        };
        let Ok(in_session) = in_params.in_sessions.get(client_entity) else {
            continue;
        };
        let Ok(in_map) = in_params.in_maps.get(client_entity) else {
            continue;
        };
        let Ok(in_channel) = in_params.in_channels.get(client_entity) else {
            continue;
        };
        let Ok((_, channel, _)) = loc_params.channels.get(in_channel.0) else {
            continue;
        };
        let Ok(in_char) = in_params.in_chars.get(client_entity) else {
            continue;
        };
        let Ok((_, char, _)) = session_params.chars.get(in_char.0) else {
            continue;
        };
        let Some((portal, _)) = portals
            .iter()
            .find(|(p, parent)| p.base.name == msg.portal_name && parent.0 == in_map.0)
        else {
            continue;
        };

        commands.entity(in_session.0).insert(Transitioning {
            map_wz: portal.base.target_map_wz,
            started_at: Instant::now(),
        });
        commands.entity(client_entity).remove::<InMap>();

        change_map_result::write_result(
            msg.client_id,
            &vec![char.clone()],
            &channel,
            &portal,
            &mut results,
        );
    }
}
