/* schema.rs
 * The purpose of this module is to frame the database schema for diesel.
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

diesel::table! {
    accounts (id) {
        id -> Nullable<Int4>,
        username -> Text,
        password -> Text,
        pin -> Nullable<Text>,
        pic -> Nullable<Text>,
        last_login_at -> Nullable<Timestamp>,
        gender_wz -> Int2,
        accepted_tos -> Bool,
        banned -> Bool,
        admin -> Bool,
        created_at -> Nullable<Timestamp>,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    characters (id) {
        id -> Nullable<Int4>,
        acc_id -> Int4,
        world_id -> Int2,
        map_wz -> Int4,
        ign -> Text,
        level -> Int2,
        exp -> Int4,
        strength -> Int2,
        dexterity -> Int2,
        luck -> Int2,
        intelligence -> Int2,
        hp -> Int2,
        mp -> Int2,
        max_hp -> Int2,
        max_mp -> Int2,
        ap -> Int2,
        sp -> Int2,
        fame -> Int2,
        meso -> Int4,
        job_wz -> Int2,
        face_wz -> Int4,
        hair_wz -> Int4,
        hair_color_wz -> Int4,
        skin_wz -> Int4,
        gender_wz -> Int2,
        last_portal -> Int2,
        created_at -> Nullable<Timestamp>,
        updated_at -> Timestamp
    }
}

diesel::table! {
    inventory_capacity (char_id) {
        id -> Nullable<Int4>,
        char_id -> Int4,
        equip_slot_capacity -> Int2,
        use_slot_capacity -> Int2,
        etc_slot_capacity -> Int2,
        setup_slot_capacity -> Int2,
        cash_slot_capacity -> Int2,
        created_at -> Nullable<Timestamp>,
        updated_at -> Timestamp
    }
}

diesel::table! {
    character_limits (acc_id, id, world_id) {
        id -> Nullable<Int4>,
        acc_id -> Int4,
        world_id -> Int2,
        char_max -> Int2,
        created_at -> Nullable<Timestamp>,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    keybindings (char_id, id, key) {
        id -> Nullable<Int4>,
        char_id -> Int4,
        key -> Int4,
        bind_type -> Int2,
        action -> Int4,
        created_at -> Nullable<Timestamp>,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    skills (char_id, id, wz) {
        id -> Nullable<Int4>,
        char_id -> Int4,
        wz -> Int4,
        level -> Int2,
        created_at -> Nullable<Timestamp>,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    items (id) {
        id -> Nullable<Int4>,
        char_id -> Nullable<Int4>,
        ipos -> Nullable<Int2>,
        strength -> Int2,
        dexterity -> Int2,
        intelligence -> Int2,
        luck -> Int2,
        attack -> Int2,
        weapon_defense -> Int2,
        magic -> Int2,
        magic_defense -> Int2,
        hp -> Int2,
        mp -> Int2,
        accuracy -> Int2,
        avoid -> Int2,
        hands -> Int2,
        speed -> Int2,
        jump -> Int2,
        wz -> Int4,
        slots -> Int4,
        expire -> Int8,
        level -> Int2,
        item_level -> Int2,
        flag -> Int2,
        item_exp -> Int2,
        vicious -> Int4,
        created_at -> Nullable<Timestamp>,
        updated_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    character_limits,
    characters,
    drops,
    items,
    keybindings,
    skills,
);

diesel::table! {
    drops (id) {
        id -> BigInt,
        mob_wz -> Integer,
        item_wz -> Integer,
        minimum_quantity -> Integer,
        maximum_quantity -> Integer,
        chance -> Integer,
    }
}
