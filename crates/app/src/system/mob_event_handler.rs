/* app/src/system/mob_event_handler.rs
 * The purpose of this module is to provide a system for handling raw mob events.
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

use crate::message::packet::attack_close::{CloseAttackResponseMessage, DeadMobResponseMessage};
use crate::system::event::RawEvent;
use bevy::ecs::message::{MessageReader, MessageWriter};
use ipc::event::AsyncEvent;

pub fn handle_mob_events_system(
    mut messages: MessageReader<RawEvent>,
    mut close_attack_success_writer: MessageWriter<CloseAttackResponseMessage>,
    mut dead_mob_success_writer: MessageWriter<DeadMobResponseMessage>,
) {
    for msg in messages.read() {
        match msg {
            RawEvent::CloseAttackSuccess(event) => match event {
                AsyncEvent::CloseAttackSuccess {
                    client_id,
                    count,
                    skill_model,
                    base_skill,
                    display,
                    toleft,
                    stance,
                    speed,
                    mob_damages,
                } => {
                    close_attack_success_writer.write(CloseAttackResponseMessage {
                        client_id: *client_id,
                        count: *count,
                        skill_model: skill_model.clone(),
                        base_skill: base_skill.clone(),
                        display: *display,
                        toleft: *toleft,
                        stance: *stance,
                        speed: *speed,
                        mob_damages: mob_damages.clone(),
                    });
                }
                _ => {}
            },
            RawEvent::DeadMobSuccess(event) => match event {
                AsyncEvent::DeadMobSuccess {
                    client_id,
                    mob_id,
                    items_map,
                } => {
                    dead_mob_success_writer.write(DeadMobResponseMessage {
                        client_id: *client_id,
                        mob_id: *mob_id,
                        items_map: items_map.clone(),
                    });
                }
                _ => {}
            },
            _ => {}
        }
    }
}
