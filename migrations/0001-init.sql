CREATE TABLE accounts (
    id SERIAL PRIMARY KEY,
    username TEXT NOT NULL,
    password TEXT NOT NULL,
    pin TEXT NULL,
    pic TEXT NULL,
    last_login_at TIMESTAMP NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    character_slots SMALLINT NOT NULL DEFAULT 8,
    gender SMALLINT NOT NULL,
    accepted_tos BOOLEAN NOT NULL DEFAULT FALSE,
    banned BOOLEAN NOT NULL DEFAULT FALSE,
    playing BOOLEAN NOT NULL DEFAULT FALSE,
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    selected_char_id INTEGER NULL,
    selected_channel_id SMALLINT NULL,
    selected_world_id SMALLINT NULL
);

CREATE TABLE characters (
    id SERIAL PRIMARY KEY,
    acc_id INTEGER NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    world_id SMALLINT NOT NULL,
    ign TEXT NOT NULL,
    level SMALLINT NOT NULL DEFAULT 1,
    exp INTEGER NOT NULL DEFAULT 0,
    strength SMALLINT NOT NULL DEFAULT 4,
    dexterity SMALLINT NOT NULL DEFAULT 4,
    luck SMALLINT NOT NULL DEFAULT 4,
    intelligence SMALLINT NOT NULL DEFAULT 4,
    hp SMALLINT NOT NULL DEFAULT 50,
    mp SMALLINT NOT NULL DEFAULT 5,
    max_hp SMALLINT NOT NULL DEFAULT 50,
    max_mp SMALLINT NOT NULL DEFAULT 0,
    ap SMALLINT NOT NULL DEFAULT 0,
    fame SMALLINT NOT NULL DEFAULT 0,
    meso INTEGER NOT NULL DEFAULT 0,
    job SMALLINT NOT NULL,
    face INTEGER NOT NULL,
    hair INTEGER NOT NULL,
    hair_color INTEGER NOT NULL,
    skin INTEGER NOT NULL,
    gender SMALLINT NOT NULL,
    map INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE character_equipment (
    char_id INTEGER NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
    hat INTEGER NULL REFERENCES equip_stats(id) ON DELETE CASCADE,
    pants INTEGER NULL REFERENCES equip_stats(id) ON DELETE CASCADE,
    face_acc INTEGER NULL REFERENCES equip_stats(id) ON DELETE CASCADE,
    eye_acc INTEGER NULL REFERENCES equip_stats(id) ON DELETE CASCADE,
    ear_acc INTEGER NULL REFERENCES equip_stats(id) ON DELETE CASCADE,
    top INTEGER NULL REFERENCES equip_stats(id) ON DELETE CASCADE,
    bottom INTEGER NULL REFERENCES equip_stats(id) ON DELETE CASCADE,
    shoes INTEGER NULL REFERENCES equip_stats(id) ON DELETE CASCADE,
    gloves INTEGER NULL REFERENCES equip_stats(id) ON DELETE CASCADE,
    cape INTEGER NULL REFERENCES equip_stats(id) ON DELETE CASCADE,
    weapon INTEGER NULL REFERENCES equip_stats(id) ON DELETE CASCADE,
    shield INTEGER NULL REFERENCES equip_stats(id) ON DELETE CASCADE,
    saddle INTEGER NULL REFERENCES equip_stats(id) ON DELETE CASCADE,
    tamed_mob INTEGER NULL REFERENCES equip_stats(id) ON DELETE CASCADE,
    sub_weapon INTEGER NULL REFERENCES equip_stats(id) ON DELETE CASCADE,
    belt INTEGER NULL REFERENCES equip_stats(id) ON DELETE CASCADE,
    pendant_one INTEGER NULL REFERENCES equip_stats(id) ON DELETE CASCADE,
    pendant_two INTEGER NULL REFERENCES equip_stats(id) ON DELETE CASCADE,
    ring_one INTEGER NULL REFERENCES equip_stats(id) ON DELETE CASCADE,
    ring_two INTEGER NULL REFERENCES equip_stats(id) ON DELETE CASCADE,
    ring_three INTEGER NULL REFERENCES equip_stats(id) ON DELETE CASCADE,
    ring_four INTEGER NULL REFERENCES equip_stats(id) ON DELETE CASCADE,
    shoulder INTEGER NULL REFERENCES equip_stats(id) ON DELETE CASCADE,
    emblem INTEGER NULL REFERENCES equip_stats(id) ON DELETE CASCADE,
    medal INTEGER NULL REFERENCES equip_stats(id) ON DELETE CASCADE,
    badge INTEGER NULL REFERENCES equip_stats(id) ON DELETE CASCADE,
    android INTEGER NULL REFERENCES equip_stats(id) ON DELETE CASCADE,
    heart INTEGER NULL REFERENCES equip_stats(id) ON DELETE CASCADE,
    book INTEGER NULL REFERENCES equip_stats(id) ON DELETE CASCADE,
    pocket INTEGER NULL REFERENCES equip_stats(id) ON DELETE CASCADE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE cash_equipment (
    char_id INTEGER NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
    ring_one INTEGER NULL,
    ring_two INTEGER NULL,
    ring_three INTEGER NULL,
    ring_four INTEGER NULL,
    hat INTEGER NULL,
    face_acc INTEGER NULL,
    hair INTEGER NULL,
    pendant INTEGER NULL,
    weapon INTEGER NULL,
    belt INTEGER NULL,
    top INTEGER NULL,
    bottom INTEGER NULL,
    shoes INTEGER NULL,
    ear_acc INTEGER NULL,
    shoulder INTEGER NULL,
    sub_weapon INTEGER NULL,
    cape INTEGER NULL,
    gloves INTEGER NULL,
    eye_acc INTEGER NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE pet_equipement (
    char_id INTEGER NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
    pet_one_acc INTEGER NULL,
    pet_two_acc INTEGER NULL,
    pet_three_acc INTEGER NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE android_equipment (
    char_id INTEGER NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
    android_hat INTEGER NULL,
    android_face INTEGER NULL,
    android_top INTEGER NULL,
    android_bottom INTEGER NULL,
    android_cloves INTEGER NULL,
    android_cape INTEGER NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()

);

CREATE TABLE character_limits (
    acc_id INTEGER NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    world_id SMALLINT NOT NULL,
    char_max INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    PRIMARY KEY (acc_id, world_id)
);

CREATE TABLE keybindings (
    id SERIAL PRIMARY KEY,
    char_id INTEGER NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
    key SMALLINT NOT NULL,
    bind_type SMALLINT NOT NULL,
    action SMALLINT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

ALTER TABLE keybindings
ADD CONSTRAINT key_is_unique_per_character UNIQUE (char_id, key);

CREATE TABLE equip_stats (
    id INTEGER PRIMARY KEY,
    wz_id INTEGER NOT NULL,
    strength INTEGER DEFAULT 0,
    dexterity INTEGER DEFAULT 0,
    intelligence INTEGER DEFAULT 0,
    luck INTEGER DEFAULT 0,
    attack INTEGER DEFAULT 0,
    weapon_defense INTEGER DEFAULT 0,
    magic INTEGER DEFAULT 0,
    magic_defense INTEGER DEFAULT 0,
    hp INTEGER DEFAULT 0,
    mp INTEGER DEFAULT 0,
    accuracy INTEGER DEFAULT 0,
    avoid INTEGER DEFAULT 0,
    hands INTEGER DEFAULT 0,
    speed INTEGER DEFAULT 0,
    jump INTEGER DEFAULT 0,
    created_at TIMESTAMP NULL,
    updated_at TIMESTAMP NULL
);
