// @generated automatically by Diesel CLI.

diesel::table! {
    accounts (id) {
        id -> Int8,
        username -> Text,
        password -> Text,
        pin -> Nullable<Text>,
        pic -> Nullable<Text>,
        last_login_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
        character_slots -> Int2,
        gender -> Int2,
        accepted_tos -> Bool,
        banned -> Bool,
        playing -> Bool,
        updated_at -> Timestamp,
        selected_character_id -> Nullable<Int4>,
        selected_channel_id -> Nullable<Int2>,
        selected_world_id -> Nullable<Int2>,
    }
}

diesel::table! {
    characters (id) {
        id -> Int4,
        acc_id -> Int8,
        world_id -> Int2,
        ign -> Text,
        level -> Nullable<Int2>,
        exp -> Nullable<Int4>,
        strength -> Nullable<Int2>,
        dexterity -> Nullable<Int2>,
        luck -> Nullable<Int2>,
        intelligence -> Nullable<Int2>,
        hp -> Nullable<Int2>,
        mp -> Nullable<Int2>,
        max_hp -> Nullable<Int2>,
        max_mp -> Nullable<Int2>,
        ap -> Nullable<Int2>,
        fame -> Nullable<Int2>,
        meso -> Nullable<Int4>,
        job -> Int2,
        face -> Int4,
        hair -> Int4,
        hair_color -> Int4,
        skin -> Int4,
        gender -> Int2,
        created_at -> Nullable<Timestamp>,
        map -> Nullable<Int4>,
        updated_at -> Nullable<Timestamp>
    }
}

diesel::table! {
    worlds (id) {
        id -> Int2,
    }
}

diesel::table! {
    character_limits (account_id, world_id) {
        account_id -> Int8,
        world_id -> Int2,
        char_max -> Int4,
        updated_at -> Timestamp,
    }
}

diesel::table! {

    keybindings (id) {
        id -> Int4,
        character_id -> Int4,
        key -> Int2,
        bind_type -> Int2,
        action -> Int2,
    }
}
diesel::allow_tables_to_appear_in_same_query!(accounts, character_limits, characters, worlds);
