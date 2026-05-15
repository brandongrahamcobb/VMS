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
        playing -> Bool,
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
        fame -> Int2,
        meso -> Int4,
        job_wz -> Int2,
        face_wz -> Int4,
        hair_wz -> Int4,
        hair_color_wz -> Int4,
        skin_wz -> Int4,
        gender_wz -> Int2,
        created_at -> Nullable<Timestamp>,
        updated_at -> Timestamp
    }
}

diesel::table! {
    character_limits (acc_id, world_id) {
        acc_id -> Int4,
        world_id -> Int2,
        char_max -> Int2,
        created_at -> Nullable<Timestamp>,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    keybindings (char_id, key) {
        char_id -> Int4,
        key -> Int4,
        bind_type -> Int2,
        action -> Int4,
        created_at -> Nullable<Timestamp>,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    skills (char_id, wz) {
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
        wz -> Int4,
        pos -> Nullable<Int2>,
        strength -> Int4,
        dexterity -> Int4,
        intelligence -> Int4,
        luck -> Int4,
        attack -> Int4,
        weapon_defense -> Int4,
        magic -> Int4,
        magic_defense -> Int4,
        hp -> Int4,
        mp -> Int4,
        accuracy -> Int4,
        avoid -> Int4,
        hands -> Int4,
        speed -> Int4,
        jump -> Int4,
        created_at -> Nullable<Timestamp>,
        updated_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    character_limits,
    characters,
    items,
    keybindings,
    skills,
);
