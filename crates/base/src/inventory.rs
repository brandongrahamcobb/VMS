/* base/src/inventory.rs
 * The purpose of this module is to provide an inventory base.
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

// use crate::item::BaseItem;
//
// #[derive(Clone)]
// pub enum InventoryModMode {
//     Add = 0,
//     ChangeCount = 1,
//     Swap = 2,
//     Remove = 3,
// }
//
// // pub struct InventoryMod {
//     pub mode: InventoryModMode,
//     pub inv_type: i8,
//     pub pos: i16,
//     pub count: i16,
//     pub char_name: String,
//     pub item_id: i32,
//     pub base_itme: Option<BaseItem>,
// }

#[derive(num_derive::FromPrimitive)]
pub enum InventoryTab {
    Equip = 1,
    Use = 2,
    Setup = 3,
    Etc = 4,
    Cash = 5,
}

pub enum EquipSlot {
    Android,
    Cash,
    Pet,
    Regular,
}

pub struct ISlot {
    pub key: i16,
    name: &'static str,
    pub islot: &'static str,
}

pub const OTHER_EQUIP_SLOTS: &[&[ISlot]] =
    &[ANDROID_EQUIP_SLOTS, PET_EQUIP_SLOTS, REGULAR_EQUIP_SLOTS];

pub const ANDROID_EQUIP_SLOTS: &[ISlot] = &[
    ISlot {
        key: -999,
        name: "Hat",
        islot: "",
    },
    ISlot {
        key: -999,
        name: "Face",
        islot: "",
    },
    ISlot {
        key: -999,
        name: "Top",
        islot: "",
    },
    ISlot {
        key: -999,
        name: "Bottom",
        islot: "",
    },
    ISlot {
        key: -999,
        name: "Gloves",
        islot: "",
    },
    ISlot {
        key: -999,
        name: "Cape",
        islot: "",
    },
];

pub const REGULAR_EQUIP_SLOTS: &[ISlot] = &[
    ISlot {
        key: -1,
        name: "Hat",
        islot: "Cp",
    },
    ISlot {
        key: -2,
        name: "FaceAcc",
        islot: "Af",
    },
    ISlot {
        key: -3,
        name: "EyeAcc",
        islot: "Ay",
    },
    ISlot {
        key: -4,
        name: "EarAcc",
        islot: "Ae",
    },
    ISlot {
        key: -5,
        name: "Top",
        islot: "Ma",
    },
    ISlot {
        key: -5,
        name: "Overall",
        islot: "MaPn",
    },
    ISlot {
        key: -6,
        name: "Bottom",
        islot: "Pn",
    },
    ISlot {
        key: -7,
        name: "Shoes",
        islot: "So",
    },
    ISlot {
        key: -8,
        name: "Gloves",
        islot: "Gv",
    },
    ISlot {
        key: -9,
        name: "Cape",
        islot: "Sr",
    },
    ISlot {
        key: -10,
        name: "Shield",
        islot: "Si",
    },
    ISlot {
        key: -11,
        name: "Weapon",
        islot: "Wp",
    },
    ISlot {
        key: -11,
        name: "Two-Handed",
        islot: "WpSi",
    },
    ISlot {
        key: -12,
        name: "RingOne",
        islot: "Ri",
    },
    ISlot {
        key: -13,
        name: "RingTwo",
        islot: "Ri",
    },
    ISlot {
        key: -15,
        name: "RingThree",
        islot: "Ri",
    },
    ISlot {
        key: -16,
        name: "RingFour",
        islot: "Ri",
    },
    ISlot {
        key: -17,
        name: "PendantOne",
        islot: "Pe",
    },
    ISlot {
        key: -18,
        name: "TamedMob",
        islot: "Tm",
    },
    ISlot {
        key: -19,
        name: "Saddle",
        islot: "Sd",
    },
    ISlot {
        key: -49,
        name: "Medal",
        islot: "Me",
    },
    ISlot {
        key: -50,
        name: "Belt",
        islot: "Be",
    },
    ISlot {
        key: -51,
        name: "Pocket",
        islot: "",
    },
    ISlot {
        key: -52,
        name: "Book",
        islot: "",
    },
    ISlot {
        key: -53,
        name: "PendantTwo",
        islot: "Pe",
    },
    ISlot {
        key: -54,
        name: "Shoulder",
        islot: "",
    },
    ISlot {
        key: -55,
        name: "Android",
        islot: "",
    },
    ISlot {
        key: -56,
        name: "Emblem",
        islot: "",
    },
    ISlot {
        key: -57,
        name: "Badge",
        islot: "",
    },
    ISlot {
        key: -58,
        name: "Subweapon",
        islot: "",
    },
    ISlot {
        key: -59,
        name: "Heart",
        islot: "",
    },
];

pub const CASH_EQUIP_SLOTS: &[ISlot] = &[
    ISlot {
        key: -101,
        name: "Hat",
        islot: "Cp",
    },
    ISlot {
        key: -102,
        name: "FaceAcc",
        islot: "Af",
    },
    ISlot {
        key: -103,
        name: "EyeAcc",
        islot: "Ay",
    },
    ISlot {
        key: -104,
        name: "EarAcc",
        islot: "Ae",
    },
    ISlot {
        key: -105,
        name: "Top",
        islot: "Ma",
    },
    ISlot {
        key: -106,
        name: "Bottom",
        islot: "Pn",
    },
    ISlot {
        key: -107,
        name: "Shoes",
        islot: "So",
    },
    ISlot {
        key: -108,
        name: "Gloves",
        islot: "Gv",
    },
    ISlot {
        key: -109,
        name: "Cape",
        islot: "Sr",
    },
    ISlot {
        key: -111,
        name: "Weapon",
        islot: "Wp",
    },
    ISlot {
        key: -112,
        name: "RingOne",
        islot: "Ri",
    },
    ISlot {
        key: -113,
        name: "RingTwo",
        islot: "Ri",
    },
    ISlot {
        key: -115,
        name: "RingThree",
        islot: "Ri",
    },
    ISlot {
        key: -116,
        name: "RingFour",
        islot: "Ri",
    },
];

pub const PET_EQUIP_SLOTS: &[ISlot] = &[
    ISlot {
        key: -999,
        name: "AccOne",
        islot: "",
    },
    ISlot {
        key: -999,
        name: "AccTwo",
        islot: "",
    },
    ISlot {
        key: -999,
        name: "AccThree",
        islot: "",
    },
];
