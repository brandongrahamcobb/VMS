// @generated automatically by Diesel CLI.

diesel::table! {
    accounts (id) {
        id -> Int4,
        username -> Text,
        password -> Text,
        pin -> Nullable<Text>,
        pic -> Nullable<Text>,
        last_login_at -> Nullable<Timestamp>,
        gender -> Int2,
        accepted_tos -> Bool,
        banned -> Bool,
        playing -> Bool,
        selected_char_id -> Nullable<Int4>,
        selected_channel_id -> Nullable<Int2>,
        selected_world_id -> Nullable<Int2>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    characters (id) {
        id -> Int4,
        acc_id -> Int4,
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
        map -> Int4,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>
    }
}

diesel::table! {
    character_equipment (char_id) {
        char_id -> Int4,
        hat -> Nullable<Int4>,
        face_acc -> Nullable<Int4>,
        eye_acc -> Nullable<Int4>,
        ear_acc -> Nullable<Int4>,
        top -> Nullable<Int4>,
        bottom -> Nullable<Int4>,
        shoes -> Nullable<Int4>,
        gloves -> Nullable<Int4>,
        cape -> Nullable<Int4>,
        shield -> Nullable<Int4>,
        weapon -> Nullable<Int4>,
        ring_one -> Nullable<Int4>,
        ring_two -> Nullable<Int4>,
        ring_three -> Nullable<Int4>,
        ring_four -> Nullable<Int4>,
        pendant_one -> Nullable<Int4>,
        tamed_mob -> Nullable<Int4>,
        saddle -> Nullable<Int4>,
        medal -> Nullable<Int4>,
        belt -> Nullable<Int4>,
        pocket -> Nullable<Int4>,
        book -> Nullable<Int4>,
        pendant_two -> Nullable<Int4>,
        shoulder -> Nullable<Int4>,
        android -> Nullable<Int4>,
        emblem -> Nullable<Int4>,
        badge -> Nullable<Int4>,
        sub_weapon -> Nullable<Int4>,
        heart -> Nullable<Int4>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    cash_equipment (char_id) {
        char_id -> Int4,
        hat -> Nullable<Int4>,
        face_acc -> Nullable<Int4>,
        eye_acc -> Nullable<Int4>,
        ear_acc -> Nullable<Int4>,
        top -> Nullable<Int4>,
        bottom -> Nullable<Int4>,
        shoes -> Nullable<Int4>,
        gloves -> Nullable<Int4>,
        cape -> Nullable<Int4>,
        weapon -> Nullable<Int4>,
        ring_one -> Nullable<Int4>,
        ring_two -> Nullable<Int4>,
        ring_three -> Nullable<Int4>,
        ring_four -> Nullable<Int4>,
        pendant -> Nullable<Int4>,
        belt -> Nullable<Int4>,
        shoulder -> Nullable<Int4>,
        sub_weapon -> Nullable<Int4>,
        hair -> Nullable<Int4>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    equip_items (id) {
        id -> Int4,
        wz_id -> Int4,
        strength -> Nullable<Int4>,
        dexterity -> Nullable<Int4>,
        intelligence -> Nullable<Int4>,
        luck -> Nullable<Int4>,
        attack -> Nullable<Int4>,
        weapon_defense -> Nullable<Int4>,
        magic -> Nullable<Int4>,
        magic_defense -> Nullable<Int4>,
        hp -> Nullable<Int4>,
        mp -> Nullable<Int4>,
        accuracy -> Nullable<Int4>,
        avoid -> Nullable<Int4>,
        hands -> Nullable<Int4>,
        speed -> Nullable<Int4>,
        jump -> Nullable<Int4>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>
    }
}

diesel::table! {
    pet_equipment (char_id) {
        char_id -> Int4,
        pet_one_acc -> Nullable<Int4>,
        pet_two_acc -> Nullable<Int4>,
        pet_three_acc -> Nullable<Int4>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>
    }
}

diesel::table! {
    android_equipment (char_id) {
        char_id -> Int4,
        android_hat -> Nullable<Int4>,
        android_face -> Nullable<Int4>,
        android_top -> Nullable<Int4>,
        android_bottom -> Nullable<Int4>,
        android_gloves -> Nullable<Int4>,
        android_cape -> Nullable<Int4>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>
    }
}

diesel::table! {
    character_limits (acc_id, world_id) {
        acc_id -> Int4,
        world_id -> Int2,
        char_max -> Int4,
        created_at -> Nullable<Timestamp>,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    keybindings (id) {
        id -> Int4,
        char_id -> Int4,
        key -> Int2,
        bind_type -> Int2,
        action -> Int2,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,

    }
}
diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    android_equipment,
    cash_equipment,
    character_equipment,
    character_limits,
    characters,
    pet_equipment,
);
