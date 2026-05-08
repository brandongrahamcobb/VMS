use crate::models::character::equipment_set::android::model::{
    AndroidEquipmentSet, NewAndroidEquipmentSetInsert,
};
use crate::models::character::equipment_set::cash::model::{
    CashEquipmentSet, NewCashEquipmentSetInsert,
};
use crate::models::character::equipment_set::pet::model::{
    NewPetEquipmentSetInsert, PetEquipmentSet,
};
use crate::models::character::equipment_set::regular::model::{
    NewRegularEquipmentSetInsert, RegularEquipmentSet,
};
use crate::models::character::keybinding::model::Keybinding;
use crate::models::character::model::{Character, CharacterModel, NewInsert};
use crate::models::character::skill::model::Skill;
use crate::models::character::{self, equipment_set, keybinding, skill};
use crate::models::error::ModelError;
use crate::models::map::model::Map;
use crate::models::world::model::World;
use crate::models::{map, world};
use crate::runtime::state::SharedState;

pub async fn get_characters_by_account_id(
    state: &SharedState,
    acc_id: i32,
) -> Result<Vec<Character>, ModelError> {
    let char_models = character::query::get_characters_models_by_account_id(state, acc_id).await?;
    let mut chars: Vec<Character> = Vec::<Character>::new();
    for char_model in char_models {
        chars.push(get_character_by_id(state, char_model.id).await?)
    }
    Ok(chars)
}

pub async fn get_character_by_id(
    state: &SharedState,
    char_id: i32,
) -> Result<Character, ModelError> {
    let char_model = character::query::get_character_model_by_id(state, char_id).await?;
    let binds_models =
        keybinding::query::get_keybinding_models_by_character_id(state, char_model.id).await?;
    let mut binds: Vec<Keybinding> = Vec::new();
    for bind_model in binds_models {
        binds.push(Keybinding { model: bind_model });
    }
    let android_equips_model =
        equipment_set::android::query::get_android_equipment_set_model_by_character_id(
            state,
            char_model.id,
        )
        .await?;
    let android_equips = equipment_set::android::service::get_android_equipment_set_from_model(
        state,
        android_equips_model.clone(),
    )
    .await?;
    let cash_equips_model =
        equipment_set::cash::query::get_cash_equipment_set_model_by_character_id(
            state,
            char_model.id,
        )
        .await?;
    let cash_equips = equipment_set::cash::service::get_cash_equipment_set_from_model(
        state,
        cash_equips_model.clone(),
    )
    .await?;
    let pet_equips_model = equipment_set::pet::query::get_pet_equipment_set_model_by_character_id(
        state,
        char_model.id,
    )
    .await?;
    let pet_equips = equipment_set::pet::service::get_pet_equipment_set_from_model(
        state,
        pet_equips_model.clone(),
    )
    .await?;
    let regular_equips_model =
        equipment_set::regular::query::get_regular_equipment_set_model_by_character_id(
            state,
            char_model.id,
        )
        .await?;
    let regular_equips = equipment_set::regular::service::get_regular_equipment_set_from_model(
        state,
        regular_equips_model.clone(),
    )
    .await?;
    let skills = skill::service::get_skills_by_character_id(state, char_model.id).await?;
    let map = map::service::get_map_by_id(char_model.map_id)?;
    let world = world::service::get_world_by_id(state, char_model.world_id).await?;
    Ok(Character {
        model: char_model,
        regular_equips,
        cash_equips,
        pet_equips,
        android_equips,
        skills,
        binds,
        world,
        map,
    })
}

impl Character {
    pub fn new(
        android_equips: AndroidEquipmentSet,
        binds: Vec<Keybinding>,
        cash_equips: CashEquipmentSet,
        map: Map,
        model: CharacterModel,
        pet_equips: PetEquipmentSet,
        regular_equips: RegularEquipmentSet,
        skills: Vec<Skill>,
        world: World,
    ) -> Self {
        Self {
            android_equips,
            binds,
            cash_equips,
            map,
            model,
            pet_equips,
            regular_equips,
            skills,
            world,
        }
    }
}

impl NewInsert {
    pub fn default(
        acc_id: i32,
        ign: String,
        job_id: i16,
        face_id: i32,
        hair_id: i32,
        hair_color_id: i32,
        skin_id: i32,
        gender_id: i16,
        map_id: i32,
        world_id: i16,
    ) -> Self {
        Self {
            acc_id,
            ign,
            job_id,
            face_id,
            hair_id,
            hair_color_id,
            skin_id,
            gender_id,
            map_id,
            world_id,
        }
    }
}

pub fn new_character_regular_equips_defaults(
    char_id: i32,
    bottom_id: i32,
    top_id: i32,
    shoes_id: i32,
    weapon_id: i32,
) -> Result<NewRegularEquipmentSetInsert, ModelError> {
    let android_id = None;
    let badge_id = None;
    let belt_id = None;
    let book_id = None;
    let cape_id = None;
    let ear_acc_id = None;
    let emblem_id = None;
    let eye_acc_id = None;
    let face_acc_id = None;
    let gloves_id = None;
    let hat_id = None;
    let heart_id = None;
    let medal_id = None;
    let pendant_one_id = None;
    let pendant_two_id = None;
    let pocket_id = None;
    let ring_four_id = None;
    let ring_one_id = None;
    let ring_three_id = None;
    let ring_two_id = None;
    let saddle_id = None;
    let shield_id = None;
    let shoulder_id = None;
    let subweapon_id = None;
    let tamed_mob_id = None;
    let regular_equips_insert = NewRegularEquipmentSetInsert::default(
        char_id,
        android_id,
        badge_id,
        belt_id,
        book_id,
        Some(bottom_id),
        cape_id,
        ear_acc_id,
        emblem_id,
        eye_acc_id,
        face_acc_id,
        gloves_id,
        hat_id,
        heart_id,
        medal_id,
        pendant_one_id,
        pendant_two_id,
        pocket_id,
        ring_four_id,
        ring_one_id,
        ring_three_id,
        ring_two_id,
        saddle_id,
        shield_id,
        Some(shoes_id),
        shoulder_id,
        subweapon_id,
        tamed_mob_id,
        Some(top_id),
        Some(weapon_id),
    );
    Ok(regular_equips_insert)
}

pub fn new_character_cash_equips_defaults(
    char_id: i32,
) -> Result<NewCashEquipmentSetInsert, ModelError> {
    let belt_id = None;
    let bottom_id = None;
    let cape_id = None;
    let ear_acc_id = None;
    let eye_acc_id = None;
    let face_acc_id = None;
    let gloves_id = None;
    let hair_id = None;
    let hat_id = None;
    let pendant_id = None;
    let ring_four_id = None;
    let ring_one_id = None;
    let ring_three_id = None;
    let ring_two_id = None;
    let shoes_id = None;
    let shoulder_id = None;
    let subweapon_id = None;
    let top_id = None;
    let weapon_id = None;
    let cash_equips_insert = NewCashEquipmentSetInsert::default(
        char_id,
        belt_id,
        bottom_id,
        cape_id,
        ear_acc_id,
        eye_acc_id,
        face_acc_id,
        gloves_id,
        hair_id,
        hat_id,
        pendant_id,
        ring_four_id,
        ring_one_id,
        ring_three_id,
        ring_two_id,
        shoes_id,
        shoulder_id,
        subweapon_id,
        top_id,
        weapon_id,
    );
    Ok(cash_equips_insert)
}

pub fn new_character_android_equips_defaults(
    char_id: i32,
) -> Result<NewAndroidEquipmentSetInsert, ModelError> {
    let bottom_id = None;
    let cape_id = None;
    let face_id = None;
    let gloves_id = None;
    let hat_id = None;
    let top_id = None;
    let android_equips_insert = NewAndroidEquipmentSetInsert::default(
        char_id, bottom_id, cape_id, face_id, gloves_id, hat_id, top_id,
    );
    Ok(android_equips_insert)
}

pub fn new_character_pet_equips_defaults(
    char_id: i32,
) -> Result<NewPetEquipmentSetInsert, ModelError> {
    let accessory_one = None;
    let accessory_two = None;
    let accessory_three = None;
    let pet_equips_insert =
        NewPetEquipmentSetInsert::default(char_id, accessory_one, accessory_two, accessory_three);
    Ok(pet_equips_insert)
}
