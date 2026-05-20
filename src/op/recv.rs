/* recv.rs
 * The purpose of this module is to provide hexcode associations with incoming packet opcodes.
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

#[derive(Debug, num_derive::FromPrimitive)]
pub enum RecvOpcode {
    RequestLogin = 0x01,
    // GuestLogin = 0x02,
    // ServerListReRequest = 0x04,
    CharListRequest = 0x05,
    ServerStatusRequest = 0x06,
    AcceptTOS = 0x07,
    // SetGender = 0x08,
    // AfterLogin = 0x09,
    // RegisterPin = 0x0A,
    ServerListRequest = 0x0B,
    // ViewAllChar = 0x0D,
    // PickAllChar = 0x0E,
    CharSelect = 0x13,
    CheckCharName = 0x15,
    CreateChar = 0x16,
    DeleteChar = 0x17,
    RegisterPic = 0x1D,
    CharSelectWithPic = 0x1E,
    // ViewAllPicRegister = 0x1F,
    // ViewAllWithPic = 0x20,
    LoginStarted = 0x23,
    //
    PlayerLoggedIn = 0x14,
    ChangeChannel = 0x27,
    EnterCashShop = 0x28,
    //
    ChangeMap = 0x26,
    //
    PlayerMove = 0x29,
    MobMoved = 0xBC,
    TakeDamage = 0x30,
    AllChat = 0x31,
    PickupItem = 0xCA,
    // Whisper = 0x78,
    //
    ChangeKeymap = 0x87,
    //
    PlayerMapTransfer = 0xCF,
    PartySearch = 0xDF,
    //
    // UnusedOpcode = 0xFF,
    CloseAttack = 0x2C,
}
