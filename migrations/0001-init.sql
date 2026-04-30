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
    hat INTEGER NULL,
    pants INTEGER NULL,
    face_acc INTEGER NULL,
    eye_acc INTEGER NULL,
    ear_acc INTEGER NULL,
    top INTEGER NULL,
    bottom INTEGER NULL,
    shoes INTEGER NULL,
    gloves INTEGER NULL,
    cape INTEGER NULL,
    weapon INTEGER NULL,
    shield INTEGER NULL,
    saddle INTEGER NULL,
    tamed_mob INTEGER NULL,
    sub_weapon INTEGER NULL,
    belt INTEGER NULL,
    pendant_one INTEGER NULL,
    pendant_two INTEGER NULL,
    ring_one INTEGER NULL,
    ring_two INTEGER NULL,
    ring_three INTEGER NULL,
    ring_four INTEGER NULL,
    shoulder INTEGER NULL,
    emblem INTEGER NULL,
    medal INTEGER NULL,
    badge INTEGER NULL,
    android INTEGER NULL,
    heart INTEGER NULL,
    book INTEGER NULL,
    pocket INTEGER NULL,
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

