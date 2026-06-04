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
    let mode: i8 = 1;
    let stance: i8 = 0;
    let effect: i8 = 0;
    let team: i8 = -1;
    for (mob_entity, mut mob, map_entity) in mobs.iter_mut().find(|(_, m, _)| m.dead == true) {
        if mob.died_at.elapsed().as_secs() >= mob.base.mob_time {
            mob.dead = false;
            let Ok(mut hp) = healths.get_mut(mob_entity) else {
                continue;
            };
            hp.amount = mob.base.max_hp;
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
                        world_id: world.id,
                        channel_id: channel.id,
                        map_wz: map.base.wz,
                    },
                },
                Action::TickAction {
                    packet: spawn_mob_controller_packet.finish(),
                    scope: TickScope::Map {
                        world_id: world.id,
                        channel_id: channel.id,
                        map_wz: map.base.wz,
                    },
                },
            ],
        });
    }
}
