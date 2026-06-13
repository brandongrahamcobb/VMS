/* inc/src/map.rs
 * The purpose of this module is to provide helper methods for maps.
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

use base::map::Point;

pub fn parse_position(movement_bytes: &[u8]) -> Option<Point> {
    let mut cursor = 0;
    let length = *movement_bytes.get(cursor)? as usize;
    let progress: usize = 1;
    cursor += progress;
    let mut last_x: i16 = 0;
    let mut last_y: i16 = 0;
    for _ in 0..length {
        let command = *movement_bytes.get(cursor)?;
        cursor += 1;
        match command {
            0 | 5 | 17 => {
                last_x = i16::from_le_bytes([
                    *movement_bytes.get(cursor)?,
                    *movement_bytes.get(cursor + 1)?,
                ]);
                last_y = i16::from_le_bytes([
                    *movement_bytes.get(cursor + 2)?,
                    *movement_bytes.get(cursor + 3)?,
                ]);
                cursor += 13;
            }
            1 | 2 | 6 | 12 | 13 | 16 => {
                last_x = i16::from_le_bytes([
                    *movement_bytes.get(cursor)?,
                    *movement_bytes.get(cursor + 1)?,
                ]);
                last_y = i16::from_le_bytes([
                    *movement_bytes.get(cursor + 2)?,
                    *movement_bytes.get(cursor + 3)?,
                ]);
                cursor += 7;
            }
            11 => {
                last_x = i16::from_le_bytes([
                    *movement_bytes.get(cursor)?,
                    *movement_bytes.get(cursor + 1)?,
                ]);
                last_y = i16::from_le_bytes([
                    *movement_bytes.get(cursor + 2)?,
                    *movement_bytes.get(cursor + 3)?,
                ]);
                cursor += 9;
            }
            15 => {
                last_x = i16::from_le_bytes([
                    *movement_bytes.get(cursor)?,
                    *movement_bytes.get(cursor + 1)?,
                ]);
                last_y = i16::from_le_bytes([
                    *movement_bytes.get(cursor + 2)?,
                    *movement_bytes.get(cursor + 3)?,
                ]);
                cursor += 15;
            }
            3 | 4 | 7 | 8 | 9 | 10 | 14 => {}
            _ => return None,
        }
    }
    Some(Point {
        x: last_x,
        y: last_y,
    })
}
