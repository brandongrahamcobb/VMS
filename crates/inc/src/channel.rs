/* helpers.rs
 * The purpose of this module is to provide project-wide functions.
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

use config::settings;

pub fn get_channel_ports(base_port: i16) -> Vec<i16> {
    let mut ports: Vec<i16> = Vec::new();
    let first_port: i16 = base_port + 1;
    let count: u8 = settings::get_channel_count().unwrap_or(3);
    for offset in 0..count {
        let port: i16 = (first_port + offset as i16) as i16;
        ports.push(port)
    }
    ports
}
