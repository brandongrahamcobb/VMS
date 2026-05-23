/* assembly/src/world/constants.rs
 * The purpose of this module is to provide constants for world assembly.
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

use entity::world::model::WorldModel;

const EVENT_MESSAGE: &str = "";
const FLAG: i16 = 0;

pub const WORLDS: &[WorldModel] = &[
    WorldModel {
        name: "Scania",
        event_message: EVENT_MESSAGE,
        flag: FLAG,
        base_port: 8585,
    },
    WorldModel {
        name: "Bera",
        event_message: EVENT_MESSAGE,
        flag: FLAG,
        base_port: 8686,
    },
    WorldModel {
        name: "Broa",
        event_message: EVENT_MESSAGE,
        flag: FLAG,
        base_port: 8787,
    },
    WorldModel {
        name: "Windia",
        event_message: EVENT_MESSAGE,
        flag: FLAG,
        base_port: 8888,
    },
    WorldModel {
        name: "Khaini",
        event_message: EVENT_MESSAGE,
        flag: FLAG,
        base_port: 8989,
    },
    WorldModel {
        name: "Mardia",
        event_message: EVENT_MESSAGE,
        flag: FLAG,
        base_port: 9090,
    },
    WorldModel {
        name: "Yellonde",
        event_message: EVENT_MESSAGE,
        flag: FLAG,
        base_port: 9191,
    },
    WorldModel {
        name: "Bellocan",
        event_message: EVENT_MESSAGE,
        flag: FLAG,
        base_port: 9292,
    },
];
