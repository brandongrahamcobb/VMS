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

use core::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use rand::RngExt;

pub fn to_hex_string(bytes: &[u8]) -> String {
    let strs: Vec<String> = bytes.iter().map(|b| format!("{:02X}", b)).collect();
    strs.join(" ")
}

pub fn convert_to_ip_array(addr: String) -> [u8; 4] {
    let mut octets = [0u8; 4];
    let parts: Vec<&str> = addr.trim().split('.').collect();
    if parts.len() != 4 {
        return octets;
    }
    octets[0] = parts[0].parse().unwrap_or(0);
    octets[1] = parts[1].parse().unwrap_or(0);
    octets[2] = parts[2].parse().unwrap_or(0);
    octets[3] = parts[3].parse().unwrap_or(0);
    octets
}

pub fn build_server_addr(addr: String, port: i16) -> SocketAddr {
    let octets = convert_to_ip_array(addr);
    SocketAddr::V4(SocketAddrV4::new(
        Ipv4Addr::new(octets[0], octets[1], octets[2], octets[3]),
        port as u16,
    ))
}

pub fn calculate_rand_stat(default_value: i16, max_range: i32) -> i16 {
    if default_value == 0 {
        return 0;
    }
    let l_max_range = ((default_value as f64 * 0.1).ceil() as i32).min(max_range);
    let rand = rand::rng().random::<f64>();
    ((default_value as i32 - l_max_range) + (rand * (l_max_range * 2 + 1) as f64).floor() as i32)
        as i16
}

pub fn calculate_rand_meso_amount(meso_rate: f64, level: i16) -> i32 {
    let base = (level * 3 + 20) as f32;
    let min = (base * 0.75) as i32;
    let max = (base * 1.25) as i32;
    let amount = rand::rng().random_range(min..=max);
    (amount as f64 * meso_rate) as i32
}
