/* aes.rs
 * The purpose of this module is to provide the AES encryption and decryption of packets.
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

use crate::constants;
use aes::Aes256;
use aes::cipher::BlockCipherEncrypt;
use aes::cipher::KeyInit;

pub struct AES {
    pub iv: Vec<u8>,
    pub version: u16,
}

impl AES {
    pub fn new(iv: &[u8], version: i16) -> AES {
        let iv = iv.to_owned();
        let adjusted_version = (version as u16 >> 8) & 0xFF | (((version as u16) << 8) & 0xFF00);
        AES {
            iv,
            version: adjusted_version,
        }
    }

    pub fn crypt(&mut self, data: &mut [u8]) {
        let mut remaining = data.len();
        let mut llength = 0x5B0;
        let mut start = 0;
        let key = self.get_trimmed_user_key();
        let cipher = Aes256::new_from_slice(&key).expect("Invalid key length");
        while remaining > 0 {
            let iv = self.repeat_bytes(&self.iv, 4);
            let mut block = aes::Block::try_from(&iv[..16]).expect("Invalid block length");
            if remaining < llength {
                llength = remaining;
            }
            for i in start..(start + llength) {
                if (i - start) % iv.len() == 0 {
                    cipher.encrypt_block(&mut block);
                }
                data[i] ^= block[(i - start) % 16];
            }
            start += llength;
            remaining -= llength;
            llength = 0x5B4;
        }
        self.update_iv();
    }

    pub fn gen_packet_header(&self, length: i16) -> Vec<u8> {
        let mut iiv: u32 = self.iv[3] as u32 & 0xFF;
        iiv |= ((self.iv[2] as u32) << 8) & 0xFF00;
        iiv ^= self.version as u32;
        let mlength = (((length as u32) << 8) & 0xFF00) | ((length as u32) >> 8);
        let xored_iv = iiv ^ mlength;
        vec![
            (iiv >> 8) as u8,
            iiv as u8,
            (xored_iv >> 8) as u8,
            xored_iv as u8,
        ]
    }

    fn get_trimmed_user_key(&self) -> [u8; 32] {
        let mut key = [0u8; 32];
        for i in (0..128).step_by(16) {
            key[i / 4] = constants::USER_KEY[i];
        }
        key
    }

    fn update_iv(&mut self) {
        self.iv = self.get_new_iv(&self.iv);
    }

    fn get_new_iv(&self, iv: &[u8]) -> Vec<u8> {
        let mut new_iv: Vec<u8> = constants::DEFAULT_AES_KEY_VALUE.to_vec();
        let shuffle_bytes = constants::SHUFFLE_BYTES;
        for item in iv.iter().take(4) {
            let byte = *item;
            new_iv[0] =
                new_iv[0].wrapping_add(shuffle_bytes[(new_iv[1]) as usize].wrapping_sub(byte));
            new_iv[1] = new_iv[1].wrapping_sub(new_iv[2] ^ shuffle_bytes[(byte) as usize]);
            new_iv[2] ^= shuffle_bytes[(new_iv[3]) as usize].wrapping_add(byte);
            new_iv[3] =
                new_iv[3].wrapping_add((shuffle_bytes[(byte) as usize]).wrapping_sub(new_iv[0]));
            let mut mask = 0usize;
            mask |= (new_iv[0] as usize) & 0xFF;
            mask |= ((new_iv[1] as usize) << 8) & 0xFF00;
            mask |= ((new_iv[2] as usize) << 16) & 0xFF0000;
            mask |= ((new_iv[3] as usize) << 24) & 0xFF000000;
            mask = (mask >> 0x1D) | (mask << 3);
            for (j, item) in new_iv.iter_mut().enumerate().take(4) {
                *item = ((mask >> (8 * j)) & 0xFF) as u8;
            }
        }
        new_iv
    }

    fn repeat_bytes(&self, input: &[u8], mul: usize) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        let iv_len = input.len();
        for i in 0..(iv_len * mul) {
            result.push(input[i % iv_len])
        }
        result
    }
}
