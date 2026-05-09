use crate::constants::{DEFAULT_ACTION, DEFAULT_KEY, DEFAULT_TYPE};
use crate::models::account::model::Account;
use crate::models::character;
use crate::models::character::equipment_set::android::model::{
    AndroidEquipmentSet, AndroidEquipmentSetModel,
};
use crate::models::character::equipment_set::cash::model::{
    CashEquipmentSet, CashEquipmentSetModel,
};
use crate::models::character::equipment_set::pet::model::{PetEquipmentSet, PetEquipmentSetModel};
use crate::models::character::equipment_set::regular::model::{
    RegularEquipmentSet, RegularEquipmentSetModel,
};
use crate::models::character::keybinding::model::{Keybinding, KeybindingModel};
use crate::models::character::model::{Character, CharacterModel};
use crate::models::character::skill::model::{Skill, SkillModel};
use crate::models::character::{equipment_set, keybinding};
use crate::models::item::equip;
use crate::models::shroom::job;
use crate::models::shroom::map;
use crate::models::shroom::map::model::Map;
use crate::models::shroom::world::model::World;
use crate::net::error::NetworkError;
use crate::net::packet::handler::create_char::reader::CreateCharReader;
use crate::runtime::session::model::Session;
use crate::runtime::state::SharedState;
use itertools::izip;
use std::time::SystemTime;

#[derive(Clone)]
pub struct CreateCharStore {
    pub char: Character,
}

impl CreateCharStore {
    async fn init_char_model(
        state: &SharedState,
        reader: CreateCharReader,
        acc: Account,
        map: Map,
        world: World,
    ) -> Result<CharacterModel, NetworkError> {
        let char_models: Vec<CharacterModel> = Vec::from([CharacterModel {
            id: None,
            acc_id: acc.model.get_id()?,
            ign: reader.ign.clone(),
            job_id: reader.job_id,
            face_id: reader.face_id,
            hair_id: reader.hair_id,
            hair_color_id: reader.hair_color_id,
            skin_id: reader.skin_id,
            gender_id: reader.gender_id,
            map_id: map.model.wz_id,
            world_id: world.model.id,
            level: 0,
            exp: 0,
            strength: 4,
            dexterity: 4,
            luck: 4,
            intelligence: 4,
            hp: 50,
            mp: 5,
            max_hp: 50,
            max_mp: 5,
            ap: 0,
            fame: 0,
            meso: 0,
            created_at: Some(SystemTime::now()),
            updated_at: SystemTime::now(),
        }]);
        let char_models = character::query::setters::update_characters(state, char_models).await?;
        Ok(char_models[0].clone())
    }

    pub async fn init_keybindings(
        state: &SharedState,
        char_id: i32,
    ) -> Result<Vec<Keybinding>, NetworkError> {
        let bind_models: Vec<KeybindingModel> = izip!(DEFAULT_KEY, DEFAULT_TYPE, DEFAULT_ACTION)
            .map(
                |(key, bind_type, action): (i32, i16, i32)| KeybindingModel {
                    char_id,
                    key,
                    bind_type,
                    action,
                    created_at: Some(SystemTime::now()),
                    updated_at: SystemTime::now(),
                },
            )
            .collect();
        let bind_models =
            keybinding::query::setters::update_keybindings(state, bind_models).await?;
        let mut binds: Vec<Keybinding> = Vec::<Keybinding>::new();
        for bind_model in &bind_models {
            binds.push(bind_model.load()?);
        }
        Ok(binds.clone())
    }

    async fn init_regular_equips(
        state: &SharedState,
        reader: CreateCharReader,
        char_id: i32,
    ) -> Result<RegularEquipmentSet, NetworkError> {
        let top_model = equip::service::deserialize(reader.top_id)?;
        let top_model =
            equip::query::setters::update_equips(state, vec![top_model]).await?[0].clone();
        let bottom_model = equip::service::deserialize(reader.bottom_id)?;
        let bottom_model =
            equip::query::setters::update_equips(state, vec![bottom_model]).await?[0].clone();
        let shoes_model = equip::service::deserialize(reader.shoes_id)?;
        let shoes_model =
            equip::query::setters::update_equips(state, vec![shoes_model]).await?[0].clone();
        let weapon_model = equip::service::deserialize(reader.weapon_id)?;
        let weapon_model =
            equip::query::setters::update_equips(state, vec![weapon_model]).await?[0].clone();
        let regular_equip_set_models: Vec<RegularEquipmentSetModel> =
            Vec::from([RegularEquipmentSetModel {
                char_id,
                top_id: top_model.id,
                bottom_id: bottom_model.id,
                shoes_id: shoes_model.id,
                weapon_id: weapon_model.id,
                android_id: None,
                badge_id: None,
                belt_id: None,
                book_id: None,
                cape_id: None,
                ear_acc_id: None,
                emblem_id: None,
                eye_acc_id: None,
                face_acc_id: None,
                gloves_id: None,
                hat_id: None,
                heart_id: None,
                medal_id: None,
                pendant_one_id: None,
                pendant_two_id: None,
                pocket_id: None,
                ring_four_id: None,
                ring_one_id: None,
                ring_three_id: None,
                ring_two_id: None,
                saddle_id: None,
                shield_id: None,
                shoulder_id: None,
                subweapon_id: None,
                tamed_mob_id: None,
                created_at: Some(SystemTime::now()),
                updated_at: SystemTime::now(),
            }]);
        let regular_equip_set_models: Vec<RegularEquipmentSetModel> =
            equipment_set::regular::query::setters::update_regular_equips(
                state,
                regular_equip_set_models.clone(),
            )
            .await?;
        let mut regular_equips: Vec<RegularEquipmentSet> = Vec::new();
        for regular_equip_set_model in &regular_equip_set_models {
            regular_equips.push(regular_equip_set_model.load(state).await?)
        }
        Ok(regular_equips[0].clone())
    }

    async fn init_cash_equips(
        state: &SharedState,
        _reader: CreateCharReader,
        char_id: i32,
    ) -> Result<CashEquipmentSet, NetworkError> {
        let cash_equip_set_models: Vec<CashEquipmentSetModel> =
            Vec::from([CashEquipmentSetModel {
                char_id,
                pendant_id: None,
                hair_id: None,
                top_id: None,
                bottom_id: None,
                shoes_id: None,
                weapon_id: None,
                belt_id: None,
                cape_id: None,
                ear_acc_id: None,
                eye_acc_id: None,
                face_acc_id: None,
                gloves_id: None,
                hat_id: None,
                ring_four_id: None,
                ring_one_id: None,
                ring_three_id: None,
                ring_two_id: None,
                shoulder_id: None,
                subweapon_id: None,
                created_at: Some(SystemTime::now()),
                updated_at: SystemTime::now(),
            }]);
        let cash_equip_set_models: Vec<CashEquipmentSetModel> =
            equipment_set::cash::query::setters::update_cash_equips(
                state,
                cash_equip_set_models.clone(),
            )
            .await?;
        let mut cash_equips: Vec<CashEquipmentSet> = Vec::new();
        for cash_equip_set_model in &cash_equip_set_models {
            cash_equips.push(cash_equip_set_model.load(state).await?)
        }
        Ok(cash_equips[0].clone())
    }

    async fn init_android_equips(
        state: &SharedState,
        _reader: CreateCharReader,
        char_id: i32,
    ) -> Result<AndroidEquipmentSet, NetworkError> {
        let android_equip_set_models: Vec<AndroidEquipmentSetModel> =
            Vec::from([AndroidEquipmentSetModel {
                char_id,
                top_id: None,
                bottom_id: None,
                cape_id: None,
                gloves_id: None,
                hat_id: None,
                face_id: None,
                created_at: Some(SystemTime::now()),
                updated_at: SystemTime::now(),
            }]);
        let android_equip_set_models: Vec<AndroidEquipmentSetModel> =
            equipment_set::android::query::setters::update_android_equips(
                state,
                android_equip_set_models.clone(),
            )
            .await?;
        let mut android_equips: Vec<AndroidEquipmentSet> = Vec::new();
        for android_equip_set_model in &android_equip_set_models {
            android_equips.push(android_equip_set_model.load(state).await?)
        }
        Ok(android_equips[0].clone())
    }

    async fn init_pet_equips(
        state: &SharedState,
        _reader: CreateCharReader,
        char_id: i32,
    ) -> Result<PetEquipmentSet, NetworkError> {
        let pet_equip_set_models: Vec<PetEquipmentSetModel> = Vec::from([PetEquipmentSetModel {
            char_id,
            accessory_one_id: None,
            accessory_two_id: None,
            accessory_three_id: None,
            created_at: Some(SystemTime::now()),
            updated_at: SystemTime::now(),
        }]);
        let pet_equip_set_models: Vec<PetEquipmentSetModel> =
            equipment_set::pet::query::setters::update_pet_equips(
                state,
                pet_equip_set_models.clone(),
            )
            .await?;
        let mut pet_equips: Vec<PetEquipmentSet> = Vec::new();
        for pet_equip_set_model in &pet_equip_set_models {
            pet_equips.push(pet_equip_set_model.load(state).await?)
        }
        Ok(pet_equips[0].clone())
    }

    pub async fn init_skills(
        _state: &SharedState,
        _reader: CreateCharReader,
        _char_id: i32,
    ) -> Result<Vec<Skill>, NetworkError> {
        // let filename = String::from("Skill.wz");
        // let map = wz::service::get_img_map(reader.job_id as i32, &filename)?;
        // use tracing::debug;
        // debug!("{:?}", map);
        let skill_models: Vec<SkillModel> = Vec::<SkillModel>::new();
        // skill::service::get_skill_models_by_job_id(state, reader.job_id).await?;
        let mut skills: Vec<Skill> = Vec::<Skill>::new();
        for skill_model in skill_models {
            skills.push(skill_model.load()?);
        }
        Ok(skills)
    }

    pub async fn store_create_char(
        state: &SharedState,
        session: Session,
        reader: CreateCharReader,
    ) -> Result<Self, NetworkError> {
        let map: Map = map::service::get_map_by_job_id(reader.job_id)?;
        let world = session.get_world()?;
        let acc = session.get_acc()?;
        let char_model = Self::init_char_model(
            state,
            reader.clone(),
            acc.clone(),
            map.clone(),
            world.clone(),
        )
        .await?;
        let char_id = char_model.get_id()?;
        let android_equip_set: AndroidEquipmentSet =
            Self::init_android_equips(state, reader.clone(), char_id).await?;
        let cash_equip_set: CashEquipmentSet =
            Self::init_cash_equips(state, reader.clone(), char_id).await?;
        let pet_equip_set: PetEquipmentSet =
            Self::init_pet_equips(state, reader.clone(), char_id).await?;
        let regular_equip_set: RegularEquipmentSet =
            Self::init_regular_equips(state, reader.clone(), char_id).await?;
        let binds: Vec<Keybinding> = Self::init_keybindings(state, char_id).await?;
        let skills = Self::init_skills(state, reader.clone(), char_id).await?;
        let job = job::service::get_job_by_id(reader.job_id)?;
        let char = Character {
            model: char_model,
            regular_equip_set,
            cash_equip_set,
            pet_equip_set,
            android_equip_set,
            skills,
            binds,
            world,
            map,
            job,
        };
        Ok(Self { char })
    }
}
