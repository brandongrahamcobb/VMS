/* close_attack/service.rs
 * The purpose of this module is to provide assisting functions and implementations for close attacks.
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

pub fn get_max_number_of_meso_explosion_hits(skill_level: i16, number_of_meso_chunks: i16) -> i16 {
    let skill_max: i16 = match skill_level {
        1 => 5,
        2 | 3 => 6,
        4 | 5 => 7,
        6 | 7 => 8,
        8 | 9 => 9,
        10 | 11 => 10,
        12 | 13 => 11,
        14 | 15 => 12,
        16 | 17 => 13,
        18 | 19 => 14,
        20 => 15,
        _ => 0,
    };
    if number_of_meso_chunks < skill_max {
        return number_of_meso_chunks;
    } else {
        return skill_max;
    }
}
