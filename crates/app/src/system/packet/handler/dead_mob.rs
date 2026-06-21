/* app/src/system/handler/dead_mob.rs
 * The purpose of this module is to handle dead mob system messages.
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

use crate::component::item::{Lootable, MapleItem};
use crate::component::meso::MesoIndex;
use crate::component::mob::MapleMob;
use crate::message::packet::attack_close::DeadMobResponseMessage;
use crate::message::result::HandlerResult;
use crate::resource::custom_resource::{ClientMap, CustomSender};
use crate::system::packet::handler::constants::EXP_TABLE;
use crate::system::packet::handler::result::{
    drop_items_result, drop_mesos_result, kill_mob_result, level_up_result, set_exp_result,
};
use crate::system::system_params::{InParams, PositionParams, SessionParams};
use base::character::StatsUpdate;
use base::map::Point;
use bevy::ecs::entity::Entity;
use bevy::ecs::hierarchy::ChildOf;
use bevy::ecs::message::{MessageReader, MessageWriter};
use bevy::ecs::system::{Commands, Query, Res};
use config::settings;
use ipc::command::AsyncCommand;
use ipc::db_command::DatabaseCommand;
use rand::RngExt;
use std::time::Instant;

pub fn handle_dead_mob(
    mut commands: Commands,
    command_tx: Res<CustomSender>,
    client_map: Res<ClientMap>,
    in_params: InParams,
    mut session_params: SessionParams,
    pos_params: PositionParams,
    mut meso_indexes: Query<&MesoIndex>,
    mobs: Query<(Entity, &MapleMob, &ChildOf)>,
    mut messages: MessageReader<DeadMobResponseMessage>,
    mut results: MessageWriter<HandlerResult>,
) -> () {
    for msg in messages.read() {
        let mut stats_updates: Vec<StatsUpdate> = Vec::new();

        let Some(&client_entity) = client_map.0.get(&msg.client_id) else {
            continue;
        };
        let Ok(in_char) = in_params.in_chars.get(client_entity) else {
            continue;
        };
        let Ok((_, mut char, _)) = session_params.chars.get_mut(in_char.0) else {
            continue;
        };
        let Ok(in_map) = in_params.in_maps.get(client_entity) else {
            continue;
        };
        let Some((mob_entity, mob, _)) = mobs
            .iter()
            .find(|(_, m, parent)| parent.0 == in_map.0 && m.id == msg.mob_id)
        else {
            continue;
        };
        let Ok(meso_index) = meso_indexes.get_mut(in_map.0) else {
            continue;
        };
        kill_mob_result::write_result(msg.client_id, mob, &mut results);

        char.exp += mob.base.exp;
        if char.exp >= EXP_TABLE[char.level as usize] as i32 {
            char.exp = 0;
            char.level += 1;
            stats_updates.push(StatsUpdate::Level { level: char.level });
            level_up_result::write_result(msg.client_id, &char.clone(), &mut results);
        } else {
            set_exp_result::write_result(msg.client_id, &char.clone(), &mut results);
        }
        command_tx
            .0
            .send(AsyncCommand::DatabaseOperation(
                DatabaseCommand::UpdateStats {
                    client_id: msg.client_id,
                    char_id: char.id,
                    updates: vec![StatsUpdate::Exp { exp: char.exp }],
                },
            ))
            .unwrap();

        let Some((drop_from_pos, _)) = pos_params
            .curr_positions
            .iter()
            .find(|(_, parent)| parent.0 == mob_entity)
        else {
            continue;
        };
        let drop_from_point: Point = Point {
            x: drop_from_pos.x,
            y: drop_from_pos.y,
        };
        let offset_x = rand::rng().random_range(-50..=50);
        let drop_to_point: Point = Point {
            x: drop_from_pos.x + offset_x,
            y: drop_from_pos.y,
        };
        let mut item_vec: Vec<MapleItem> = Vec::new();
        for (base_item, item_model) in msg.items_map.clone() {
            let item = MapleItem::from((base_item.clone(), item_model));
            let item_enitity = commands.spawn((item.clone(), ChildOf(in_map.0))).id();
            let lootable: Lootable = Lootable {
                dropped_at: Instant::now(),
            };
            commands.spawn((lootable, ChildOf(item_enitity)));
            item_vec.push(item);
        }
        drop_items_result::write_result(
            msg.client_id,
            &item_vec,
            drop_to_point.clone(),
            drop_from_point.clone(),
            &mut results,
        );
        let Ok(meso_rate) = settings::get_meso_drop_rate() else {
            continue;
        };
        let mesos: i32 = inc::item::calculate_rand_meso_amount(meso_rate, mob.base.level);
        if mesos > 0 {
            drop_mesos_result::write_result(
                msg.client_id,
                meso_index.clone(),
                mob,
                drop_to_point,
                drop_from_point,
                &mut results,
            );
        }
    }
}
