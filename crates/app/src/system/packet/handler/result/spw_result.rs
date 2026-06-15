/* app/src/system/handler/result/spw_result.rs
 * The purpose of this module is to write the pic validation packet result.
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

use action::model::Action;
use action::scope::{ActionScope, MapScope};
use bevy::ecs::message::MessageWriter;

use crate::message::result::HandlerResult;
use crate::system::packet::build::spw;

pub fn write_result(
    client_id: i32,
    pic_status: bool,
    results: &mut MessageWriter<HandlerResult>,
) -> () {
    let mut actions: Vec<Action> = Vec::new();
    let Ok(mut spw_packet) = spw::build_spw_packet(!pic_status) else {
        return;
    };
    actions.push(Action::HandlerAction {
        packet: spw_packet.finish(),
        scope: ActionScope::Map(MapScope::SameChannelSameWorld),
    });
    results.write(HandlerResult {
        client_id: client_id,
        actions,
    });
}
