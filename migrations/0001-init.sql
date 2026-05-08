CREATE TABLE accounts (
    id SERIAL PRIMARY KEY,
    username TEXT NOT NULL,
    password TEXT NOT NULL,
    pin TEXT NULL,
    pic TEXT NULL,
    last_login_at TIMESTAMP NULL,
    character_slots SMALLINT NOT NULL DEFAULT 8,
    gender_id SMALLINT NOT NULL,
    accepted_tos BOOLEAN NOT NULL DEFAULT FALSE,
    banned BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE characters (
    id SERIAL PRIMARY KEY,
    acc_id INTEGER NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    world_id SMALLINT NOT NULL,
    ign TEXT NOT NULL,
    level SMALLINT NULL DEFAULT 1,
    exp INTEGER NULL DEFAULT 0,
    strength SMALLINT NULL DEFAULT 4,
    dexterity SMALLINT NULL DEFAULT 4,
    luck SMALLINT NULL DEFAULT 4,
    intelligence SMALLINT NULL DEFAULT 4,
    hp SMALLINT NULL DEFAULT 50,
    mp SMALLINT NULL DEFAULT 5,
    max_hp SMALLINT NULL DEFAULT 50,
    max_mp SMALLINT NULL DEFAULT 0,
    ap SMALLINT NULL DEFAULT 0,
    fame SMALLINT NULL DEFAULT 0,
    meso INTEGER NULL DEFAULT 0,
    job_id SMALLINT NOT NULL,
    face_id INTEGER NOT NULL,
    hair_id INTEGER NOT NULL,
    hair_color_id INTEGER NOT NULL,
    skin_id INTEGER NOT NULL,
    gender_id SMALLINT NOT NULL,
    map_id INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE equips (
    id SERIAL PRIMARY KEY,
    wz_id INTEGER NOT NULL,
    strength INTEGER NULL DEFAULT 0,
    dexterity INTEGER NULL DEFAULT 0,
    intelligence INTEGER NULL DEFAULT 0,
    luck INTEGER NULL DEFAULT 0,
    attack INTEGER NULL DEFAULT 0,
    weapon_defense INTEGER NULL DEFAULT 0,
    magic INTEGER NULL DEFAULT 0,
    magic_defense INTEGER NULL DEFAULT 0,
    hp INTEGER NULL DEFAULT 0,
    mp INTEGER NULL DEFAULT 0,
    accuracy INTEGER NULL DEFAULT 0,
    avoid INTEGER NULL DEFAULT 0,
    hands INTEGER NULL DEFAULT 0,
    speed INTEGER NULL DEFAULT 0,
    jump INTEGER NULL DEFAULT 0,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE regular_equipment_set (
    char_id INTEGER NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
    hat_id INTEGER NULL REFERENCES equips(id) ON DELETE CASCADE,
    pants_id INTEGER NULL REFERENCES equips(id) ON DELETE CASCADE,
    face_acc_id INTEGER NULL REFERENCES equips(id) ON DELETE CASCADE,
    eye_acc_id INTEGER NULL REFERENCES equips(id) ON DELETE CASCADE,
    ear_acc_id INTEGER NULL REFERENCES equips(id) ON DELETE CASCADE,
    top_id INTEGER NULL REFERENCES equips(id) ON DELETE CASCADE,
    bottom_id INTEGER NULL REFERENCES equips(id) ON DELETE CASCADE,
    shoes_id INTEGER NULL REFERENCES equips(id) ON DELETE CASCADE,
    gloves_id INTEGER NULL REFERENCES equips(id) ON DELETE CASCADE,
    cape_id INTEGER NULL REFERENCES equips(id) ON DELETE CASCADE,
    weapon_id INTEGER NULL REFERENCES equips(id) ON DELETE CASCADE,
    shield_id INTEGER NULL REFERENCES equips(id) ON DELETE CASCADE,
    saddle_id INTEGER NULL REFERENCES equips(id) ON DELETE CASCADE,
    tamed_mob_id INTEGER NULL REFERENCES equips(id) ON DELETE CASCADE,
    subweapon_id INTEGER NULL REFERENCES equips(id) ON DELETE CASCADE,
    belt_id INTEGER NULL REFERENCES equips(id) ON DELETE CASCADE,
    pendant_one_id INTEGER NULL REFERENCES equips(id) ON DELETE CASCADE,
    pendant_two_id INTEGER NULL REFERENCES equips(id) ON DELETE CASCADE,
    ring_one_id INTEGER NULL REFERENCES equips(id) ON DELETE CASCADE,
    ring_two_id INTEGER NULL REFERENCES equips(id) ON DELETE CASCADE,
    ring_three_id INTEGER NULL REFERENCES equips(id) ON DELETE CASCADE,
    ring_four_id INTEGER NULL REFERENCES equips(id) ON DELETE CASCADE,
    shoulder_id INTEGER NULL REFERENCES equips(id) ON DELETE CASCADE,
    emblem_id INTEGER NULL REFERENCES equips(id) ON DELETE CASCADE,
    medal_id INTEGER NULL REFERENCES equips(id) ON DELETE CASCADE,
    badge_id INTEGER NULL REFERENCES equips(id) ON DELETE CASCADE,
    android_id INTEGER NULL REFERENCES equips(id) ON DELETE CASCADE,
    heart_id INTEGER NULL REFERENCES equips(id) ON DELETE CASCADE,
    book_id INTEGER NULL REFERENCES equips(id) ON DELETE CASCADE,
    pocket_id INTEGER NULL REFERENCES equips(id) ON DELETE CASCADE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    PRIMARY KEY (char_id)
);

CREATE TABLE cash_equipment_set (
    char_id INTEGER NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
    ring_one_id INTEGER NULL,
    ring_two_id INTEGER NULL,
    ring_three_id INTEGER NULL,
    ring_four_id INTEGER NULL,
    hat_id INTEGER NULL,
    face_acc_id INTEGER NULL,
    hair_id INTEGER NULL,
    pendant_id INTEGER NULL,
    weapon_id INTEGER NULL,
    belt_id INTEGER NULL,
    top_id INTEGER NULL,
    bottom_id INTEGER NULL,
    shoes_id INTEGER NULL,
    ear_acc_id INTEGER NULL,
    shoulder_id INTEGER NULL,
    subweapon_id INTEGER NULL,
    cape_id INTEGER NULL,
    gloves_id INTEGER NULL,
    eye_acc_id INTEGER NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    PRIMARY KEY (char_id)
);

CREATE TABLE pet_equipment_set (
    char_id INTEGER NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
    accessory_one_id INTEGER NULL,
    paccessory_two_id INTEGER NULL,
    accessory_three_id INTEGER NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    PRIMARY KEY (char_id)
);

CREATE TABLE android_equipment_set (
    char_id INTEGER NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
    hat_id INTEGER NULL,
    face_id INTEGER NULL,
    top_id INTEGER NULL,
    bottom_id INTEGER NULL,
    gloves_id INTEGER NULL,
    cape_id INTEGER NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    PRIMARY KEY (char_id)
);

CREATE TABLE character_limits (
    acc_id INTEGER NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    world_id SMALLINT NOT NULL,
    char_max SMALLINT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    PRIMARY KEY (acc_id, world_id)
);

CREATE TABLE keybindings (
    id SERIAL PRIMARY KEY,
    char_id INTEGER NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
    key INTEGER NOT NULL,
    bind_type SMALLINT NOT NULL,
    action INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

ALTER TABLE keybindings
ADD CONSTRAINT key_is_unique_per_character UNIQUE (char_id, key);

CREATE TABLE skills (
    id SERIAL PRIMARY KEY,
    char_id INTEGER NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
    wz_id INTEGER NOT NULL,
    level SMALLINT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);



