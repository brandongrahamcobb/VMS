/* net/src/packet/io/constants.rs
 * The purpose of this module is to provide input/output constants.
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

pub const DEFAULT_AES_KEY_VALUE: [u8; 4] = [0xF2, 0x53, 0x50, 0xC6];

pub const INVALID_OPCODE: i16 = -1;

pub const MAX_PACKET_LENGTH: i16 = i16::MAX;

pub const HEADER_SIZE: u8 = 4;
