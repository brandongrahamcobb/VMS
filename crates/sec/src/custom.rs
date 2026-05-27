/* custom.rs
 * The purpose of this module is to provide the custom encryption and decryption of packets.
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

pub fn encrypt(data: &mut [u8]) {
    let size: usize = data.len();
    let mut c: u8;
    let mut a: u8;
    for _ in 0..3 {
        a = 0;
        for j in (1..(size + 1)).rev() {
            c = data[size - j];
            c = rotl(c, 3);
            c = (c as usize).overflowing_add(j).0 as u8;
            c ^= a;
            a = c;
            c = rotr(a, j as u32);
            c ^= 0xFF;
            c = c.overflowing_add(0x48).0;
            data[size - j] = c;
        }
        a = 0;
        for j in (1..(size + 1)).rev() {
            c = data[j - 1];
            c = rotl(c, 4);
            c = (c as usize).overflowing_add(j).0 as u8;
            c ^= a;
            a = c;
            c ^= 0x13;
            c = rotr(c, 3);
            data[j - 1] = c;
        }
    }
}

pub fn decrypt(data: &mut [u8]) {
    let size: usize = data.len();
    let mut a: u8;
    let mut b: u8;
    let mut c: u8;
    for _ in 0..3 {
        b = 0;
        for j in (1..(size + 1)).rev() {
            c = data[j - 1];
            c = rotl(c, 3);
            c ^= 0x13;
            a = c;
            c ^= b;
            c = (c as usize).overflowing_sub(j).0 as u8;
            c = rotr(c, 4);
            b = a;
            data[j - 1] = c;
        }
        b = 0;
        for j in (1..(size + 1)).rev() {
            c = data[size - j];
            c = c.overflowing_sub(0x48).0;
            c ^= 0xFF;
            c = rotl(c, j as u32);
            a = c;
            c ^= b;
            c = (c as usize).overflowing_sub(j).0 as u8;
            c = rotr(c, 3);
            b = a;
            data[size - j] = c;
        }
    }
}

fn rotl(byte: u8, count: u32) -> u8 {
    let count = count % 8;
    if count > 0 {
        byte.rotate_left(count)
    } else {
        byte
    }
}

fn rotr(byte: u8, count: u32) -> u8 {
    let count = count % 8;
    if count > 0 {
        byte.rotate_right(count)
    } else {
        byte
    }
}
