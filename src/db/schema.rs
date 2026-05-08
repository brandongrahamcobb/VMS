// @generated automatically by Diesel CLI.

diesel::table! {
    accounts (id) {
        id -> Int4,
        username -> Text,
        password -> Text,
        pin -> Text,
        pic -> Text,
        last_login_at -> Timestamp,
        gender_id -> Int2,
        accepted_tos -> Bool,
        banned -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    characters (id) {
        id -> Int4,
        acc_id -> Int4,
        world_id -> Int2,
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
        job_id -> Int2,
        face_id -> Int4,
        hair_id -> Int4,
        hair_color_id -> Int4,
        skin_id -> Int4,
        gender_id -> Int2,
        map_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp
    }
}

diesel::table! {
    regular_equipment_set (char_id) {
        char_id -> Int4,
        hat_id -> Int4,
        face_acc_id -> Int4,
        eye_acc_id -> Int4,
        ear_acc_id -> Int4,
        top_id -> Int4,
        bottom_id -> Int4,
        shoes_id -> Int4,
        gloves_id -> Int4,
        cape_id -> Int4,
        shield_id -> Int4,
        weapon_id -> Int4,
        ring_one_id -> Int4,
        ring_two_id -> Int4,
        ring_three_id -> Int4,
        ring_four_id -> Int4,
        pendant_one_id -> Int4,
        tamed_mob_id -> Int4,
        saddle_id -> Int4,
        medal_id -> Int4,
        belt_id -> Int4,
        pocket_id -> Int4,
        book_id -> Int4,
        pendant_two_id -> Int4,
        shoulder_id -> Int4,
        android_id -> Int4,
        emblem_id -> Int4,
        badge_id -> Int4,
        subweapon_id -> Int4,
        heart_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    cash_equipment_set (char_id) {
        char_id -> Int4,
        hat_id -> Int4,
        face_acc_id -> Int4,
        eye_acc_id -> Int4,
        ear_acc_id -> Int4,
        top_id -> Int4,
        bottom_id -> Int4,
        shoes_id -> Int4,
        gloves_id -> Int4,
        cape_id -> Int4,
        weapon_id -> Int4,
        ring_one_id -> Int4,
        ring_two_id -> Int4,
        ring_three_id -> Int4,
        ring_four_id -> Int4,
        pendant_id -> Int4,
        belt_id -> Int4,
        shoulder_id -> Int4,
        subweapon_id -> Int4,
        hair_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    equips (id) {
        id -> Int4,
        wz_id -> Int4,
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
        created_at -> Timestamp,
        updated_at -> Timestamp
    }
}

diesel::table! {
    pet_equipment_set (char_id) {
        char_id -> Int4,
        accessory_one_id -> Int4,
        accessory_two_id -> Int4,
        accessory_three_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp
    }
}

diesel::table! {
    android_equipment_set (char_id) {
        char_id -> Int4,
        hat_id -> Int4,
        face_id -> Int4,
        top_id -> Int4,
        bottom_id -> Int4,
        gloves_id -> Int4,
        cape_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    character_limits (acc_id, world_id) {
        acc_id -> Int4,
        world_id -> Int2,
        char_max -> Int2,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    keybindings (id) {
        id -> Int4,
        char_id -> Int4,
        key -> Int4,
        bind_type -> Int2,
        action -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    skills (id) {
        id -> Int4,
        char_id -> Int4,
        wz_id -> Int4,
        level -> Int2,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    android_equipment_set,
    cash_equipment_set,
    character_limits,
    characters,
    regular_equipment_set,
    skills,
    equips,
    pet_equipment_set,
);
