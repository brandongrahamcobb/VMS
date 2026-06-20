/* app/src/system/handler/result/drop_items_and_mesos_result.rs
 * The purpose of this module is to write the kill mob packet result.
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
use base::map::Point;
use bevy::ecs::message::MessageWriter;
use config::settings;

use crate::component::item::MesoIndex;
use crate::component::mob::MapleMob;
use crate::message::result::HandlerResult;
use crate::system::packet::build::codec;

pub fn write_result(
    client_id: i32,
    mut meso_index: MesoIndex,
    mob: &MapleMob,
    drop_to_point: Point,
    drop_from_point: Point,
    results: &mut MessageWriter<HandlerResult>,
) -> () {
    let mut actions: Vec<Action> = Vec::new();
    let mode: u8 = 1; // animation 0 fade, 1 drop mob, 2 spawn in
    let owner: i32 = 0; // char id or 0
    let can_pickup: u8 = 0; // 0 everyone 1 owner, 2 party
    let player_drop: bool = false;
    let Ok(meso_rate) = settings::get_meso_drop_rate() else {
        return;
    };
    let mesos: i32 = inc::item::calculate_rand_meso_amount(meso_rate, mob.base.level);
    if mesos > 0 {
        let id = meso_index.next_id();
        let Ok(mut meso_packet) = codec::item::builder::build_drop_loot_packet(
            mode,
            id,
            true,
            mesos,
            owner,
            can_pickup,
            drop_to_point.clone(),
            drop_from_point.clone(),
            player_drop,
        ) else {
            return;
        };
        actions.push(Action::HandlerAction {
            packet: meso_packet.finish(),
            scope: ActionScope::Map(MapScope::SameChannelSameWorld),
        });
        results.write(HandlerResult { client_id, actions });
    }
}
