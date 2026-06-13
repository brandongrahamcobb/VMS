/* op/src/send.rs
 * The purpose of this module is to provide hexcode associations with outgoing packet opcodes.
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
pub enum SendOpcode {
    AccountStatus = 0x00,
    // GuestIdLogin = 0x01,
    ServerStatus = 0x03,
    // CheckPin = 0x06,
    // UpdatePin = 0x07,
    Ping = 0x11,
    ServerList = 0x0A,
    NewChar = 0x0E,
    DeleteCharacter = 0x0F,
    ChangeChannel = 0x10,
    CharList = 0x0B,
    ServerIp = 0x0C,
    CheckSpwResult = 0x1C,
    CharNameResponse = 0x0D,
    LastConnectedWorld = 0x1A,
    RecommendedWorlds = 0x1B,
    //
    // StatChange = 0x1F,
    //
    // BuddyList = 0x3F,
    // FamilyInfo = 0x5F,
    // FamilyList = 0x64,
    // Whisper = 0x87,
    SpawnPlayer = 0xA0,
    DespawnPlayer = 0xA1,
    // RemovePlayerFromMap = 0xA1,
    ChatText = 0xA2,
    MovePlayer = 0xB9,
    // SpawnNpc = 0x101,
    SpawnMob = 0xEC,
    MoveMonster = 0xEF,
    ChangeStats = 0x1F,
    ShowMobHp = 0xFA,
    KillMob = 0xED,
    SpawnMobController = 0xEE,
    DropLoot = 0x10C,
    RemoveLoot = 0x10D,
    ModifyInventory = 0x1D,
    ShowForeignEffect = 0xC6,
    //
    SetCashShop = 0x7F,
    SetField = 0x7D,
    KeyMap = 0x14F,
    AttackedClose = 0xBA,
}
