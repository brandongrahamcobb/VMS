/* app/src/system/tick.rs
 * The purpose of this module is to provide a tick system.
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

use bevy::ecs::entity::Entity;
use bevy::ecs::hierarchy::ChildOf;
use bevy::ecs::message::MessageWriter;
use bevy::ecs::system::Query;

use crate::component::channel::MapleChannel;
use crate::component::hp::MapleHealth;
use crate::component::map::MapleMap;
use crate::component::mob::MapleMob;
use crate::component::world::MapleWorld;
use crate::message::result::TickResult;
use crate::system::packet::build::codec;
use action::model::Action;
use action::scope::TickScope;

// FixedUpdate

pub fn mob_respawn_system(
    worlds: Query<&MapleWorld>,
    channels: Query<(&MapleChannel, &ChildOf)>,
    maps: Query<(&MapleMap, &ChildOf)>,
    mut mobs: Query<(Entity, &mut MapleMob, &ChildOf)>,
    mut healths: Query<&mut MapleHealth>,
    mut results: MessageWriter<TickResult>,
) -> () {
    let mode: u8 = 1;
    let stance: i8 = 0;
    let effect: i8 = 0;
    let team: i8 = -1;
    while let Some((mob_entity, mut mob, map_entity)) =
        mobs.iter_mut().find(|(_, m, _)| m.dead == true)
    {
        if mob.died_at.elapsed().as_secs() >= mob.base.mob_time {
            mob.dead = false;
            let Ok(mut hp) = healths.get_mut(mob_entity) else {
                continue;
            };
            hp.amount = mob.base.max_hp as i32;
        } else {
            continue;
        }
        let Ok((map, channel_entity)) = maps.get(map_entity.0) else {
            continue;
        };
        let Ok((channel, world_entity)) = channels.get(channel_entity.0) else {
            continue;
        };
        let Ok(world) = worlds.get(world_entity.0) else {
            continue;
        };
        let Ok(mut spawn_mob_packet) =
            codec::mob::builder::build_spawn_mob_packet(&*mob, stance, effect, team)
        else {
            continue;
        };
        let Ok(mut spawn_mob_controller_packet) =
            codec::mob::builder::build_spawn_mob_controller_packet(
                &*mob, mode, stance, effect, team,
            )
        else {
            continue;
        };
        results.write(TickResult {
            actions: vec![
                Action::TickAction {
                    packet: spawn_mob_packet.finish(),
                    scope: TickScope::Map {
                        world_id: world.base.id,
                        channel_id: channel.id,
                        map_wz: map.base.wz,
                    },
                },
                Action::TickAction {
                    packet: spawn_mob_controller_packet.finish(),
                    scope: TickScope::Map {
                        world_id: world.base.id,
                        channel_id: channel.id,
                        map_wz: map.base.wz,
                    },
                },
            ],
        });
    }
}
