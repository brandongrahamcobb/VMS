use bevy::ecs::hierarchy::ChildOf;
use bevy::ecs::message::MessageWriter;
use bevy::time::Time;

use crate::component::channel::MapleChannel;
use crate::component::map::MapleMap;
use crate::component::mob::MapleMob;
use crate::component::position::MaplePosition;
use crate::component::world::MapleWorld;
use crate::system::packet::build::codec::mob;
use crate::system::packet::handler::result::HandlerResult;

// FixedUpdate

pub fn mob_respawn_system(
    time: Res<Time>,
    results: MessageWriter<HandlerResult>,
    mobs: Query<(&MapleMob, &MaplePosition, &ChildOf)>,
    worlds: Query<&MapleWorld, &ChildOf>,
    channel: Query<&MapleChannel, &ChildOf>,
    maps: Query<&MapleMap, &ChildOf>,
) -> () {
    let mode: i8 = 1;
    let stance: i8 = 0;
    let effect: i8 = 0;
    let team: i8 = -1;
    for (mob, pos, map) in mobs.iter_mut().find(|m, _, parent| m.dead == true) {
        if mob.death_time.elapsed().as_secs() >= mob.mob_time {
            mob.dead = false;
            mob.hp = mob.max_hp;
        } else {
            continue;
        }
        let Ok((map, channel)) = maps.get(map.0) else {
            continue;
        };
        let Ok((channel, world)) = channels.get(channel.0) else {
            continue;
        };
        let Ok(spawn_mob_packet) =
            mob::builder::build_spawn_mob_packet(mob, pos, stance, effect, team)
        else {
            continue;
        };
        let Ok(spawn_mob_controller_packet) =
            mob::builder::build_spawn_mob_controller_packet(mob, mode, stance, effect, team)
        else {
            continue;
        };
        results.write(TickResult {
            actions: vec![
                Action::Tick(TickAction::Send {
                    packet: spawn_mob_packet.finish(),
                    scope: TickScope::Map {
                        world_id: world.id,
                        channel_id: channel.id,
                        map_wz: map.wz,
                    },
                }),
                Action::Tick(TickAction::Send {
                    packet: spawn_mob_controller_packet.finish(),
                    scope: TickScope::Map {
                        world_id: world.id,
                        channel_id: channel.id,
                        map_wz: map.wz,
                    },
                }),
            ],
        });
    }
}
