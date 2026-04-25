// @generated automatically by Diesel CLI.

diesel::table! {
    accounts (id) {
        id -> Int8,
        username -> Text,
        password -> Text,
        pin -> Text,
        pic -> Text,
        last_login_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
        character_slots -> Int2,
        gender -> Int2,
        accepted_tos -> Bool,
        banned -> Bool,
        playing -> Bool,
        updated_at -> Timestamp
    }
}

diesel::table! {
    characters (id) {
        id -> Int4,
        account -> Int8,
        world -> Int2,
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
        job -> Int2,
        face -> Int4,
        hair -> Int4,
        hair_color -> Int4,
        skin -> Int4,
        gender -> Int2,
        created_at -> Timestamp,
        map -> Int4,
        updated_at -> Timestamp
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
diesel::allow_tables_to_appear_in_same_query!(accounts, character_limits, characters, worlds);
