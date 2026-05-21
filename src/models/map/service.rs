/* map/service.rs
 * The purpose of this module is to provide assisting functions and implementations for maps.
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

use crate::metadata;
use crate::models::map::error::MapError;
use crate::models::map::model::{MapWzInfo, Point};

pub fn build_map_wz_info(map_wz: i32) -> Result<MapWzInfo, MapError> {
    let return_map_wz = get_return_map(map_wz)?;
    let mob_rate = get_mob_rate(map_wz)?;
    Ok(MapWzInfo {
        mob_rate,
        return_map_wz,
    })
}

pub fn get_map_wz_by_job_id(job_id: i16) -> Result<i32, MapError> {
    match job_id {
        1 => Ok(10000),
        1000 => Ok(130000000),
        2000 => Ok(140000000),
        _ => Ok(0), //placeholder
    }
}

pub fn get_return_map(map_wz: i32) -> Result<i32, MapError> {
    let filename: &str = "Map.wz";
    let json = metadata::service::wz_to_img(map_wz, &filename)?;
    let return_map_wz = json["info"]["returnMap"].as_i64().unwrap() as i32;
    Ok(return_map_wz)
}

pub fn get_mob_rate(map_wz: i32) -> Result<f32, MapError> {
    let filename: &str = "Map.wz";
    let json = metadata::service::wz_to_img(map_wz, &filename)?;
    let mob_rate = json["info"]["mobRate"].as_f64().unwrap() as f32;
    Ok(mob_rate)
}

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
                // ABSOLUTE: xpos(2) + ypos(2) + lastx(2) + lasty(2) + fh(2) + newstate(1) + duration(2) = 13
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
                // RELATIVE: xpos(2) + ypos(2) + newstate(1) + duration(2) = 7
                last_x = i16::from_le_bytes([
                    *movement_bytes.get(cursor)?,
                    *movement_bytes.get(cursor + 1)?,
                ]);
                last_y = i16::from_le_bytes([
                    *movement_bytes.get(cursor + 2)?,
                    *movement_bytes.get(cursor + 3)?,
                ]);
                cursor += 7; // was 5, off by 2
            }
            11 => {
                // CHAIR: xpos(2) + ypos(2) + skip(2) + newstate(1) + duration(2) = 9
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
                // JUMPDOWN: xpos(2) + ypos(2) + lastx(2) + lasty(2) + skip(2) + fh(2) + newstate(1) + duration(2) = 15
                last_x = i16::from_le_bytes([
                    *movement_bytes.get(cursor)?,
                    *movement_bytes.get(cursor + 1)?,
                ]);
                last_y = i16::from_le_bytes([
                    *movement_bytes.get(cursor + 2)?,
                    *movement_bytes.get(cursor + 3)?,
                ]);
                cursor += 15; // was 13, off by 2
            }
            3 | 4 | 7 | 8 | 9 | 10 | 14 => {
                // NONE: no payload
            }
            _ => return None,
        }
    }
    Some(Point {
        x: last_x,
        y: last_y,
    })
}
