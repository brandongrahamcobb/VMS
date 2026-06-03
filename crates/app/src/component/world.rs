/* app/src/component/world.rs
 * The purpose of this module is to provide a world component.
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
use bevy::ecs::{component::Component, entity::Entity};

#[derive(Component, Clone)]
pub struct MapleWorld {
    pub id: i16,
    pub name: &'static str,
    pub flag: i16,
    pub event_message: &'static str,
    pub base_port: i16,
}

#[derive(Component)]
pub struct InWorld(pub Entity);

const EVENT_MESSAGE: &str = "";
const FLAG: i16 = 0;

pub const WORLDS: &[MapleWorld] = &[
    MapleWorld {
        id: 0,
        name: "Scania",
        event_message: EVENT_MESSAGE,
        flag: FLAG,
        base_port: 8585,
    },
    MapleWorld {
        id: 1,
        name: "Bera",
        event_message: EVENT_MESSAGE,
        flag: FLAG,
        base_port: 8686,
    },
    MapleWorld {
        id: 2,
        name: "Broa",
        event_message: EVENT_MESSAGE,
        flag: FLAG,
        base_port: 8787,
    },
    MapleWorld {
        id: 3,
        name: "Windia",
        event_message: EVENT_MESSAGE,
        flag: FLAG,
        base_port: 8888,
    },
    MapleWorld {
        id: 4,
        name: "Khaini",
        event_message: EVENT_MESSAGE,
        flag: FLAG,
        base_port: 8989,
    },
    MapleWorld {
        id: 5,
        name: "Mardia",
        event_message: EVENT_MESSAGE,
        flag: FLAG,
        base_port: 9090,
    },
    MapleWorld {
        id: 6,
        name: "Yellonde",
        event_message: EVENT_MESSAGE,
        flag: FLAG,
        base_port: 9191,
    },
    MapleWorld {
        id: 7,
        name: "Bellocan",
        event_message: EVENT_MESSAGE,
        flag: FLAG,
        base_port: 9292,
    },
];
