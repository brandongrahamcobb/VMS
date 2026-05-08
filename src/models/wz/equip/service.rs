use std::time::SystemTime;

use crate::models::error::ModelError;
use crate::models::wz::equip::model::{Equip, EquipModel, NewEquipInsert};
use crate::models::wz::{self, equip};
use crate::runtime::state::SharedState;

pub fn create_equip_insert(wz_id: i32) -> Result<NewEquipInsert, ModelError> {
    let filename = String::from("Character.wz");
    let map = wz::service::get_img_map(wz_id, &filename)?;
    let item = NewEquipInsert {
        wz_id: wz_id,
        strength: wz::service::get_i32(&map, "incSTR").unwrap_or(0),
        dexterity: wz::service::get_i32(&map, "incDEX").unwrap_or(0),
        intelligence: wz::service::get_i32(&map, "incINT").unwrap_or(0),
        luck: wz::service::get_i32(&map, "incLUK").unwrap_or(0),
        attack: wz::service::get_i32(&map, "incPAD").unwrap_or(0),
        weapon_defense: wz::service::get_i32(&map, "incPDD").unwrap_or(0),
        magic: wz::service::get_i32(&map, "incMAD").unwrap_or(0),
        magic_defense: wz::service::get_i32(&map, "incMDD").unwrap_or(0),
        hp: wz::service::get_i32(&map, "incMHP").unwrap_or(0),
        mp: wz::service::get_i32(&map, "incMMP").unwrap_or(0),
        accuracy: wz::service::get_i32(&map, "incACC").unwrap_or(0),
        avoid: wz::service::get_i32(&map, "incEVA").unwrap_or(0),
        hands: wz::service::get_i32(&map, "incHANDS").unwrap_or(0),
        speed: wz::service::get_i32(&map, "incSPEED").unwrap_or(0),
        jump: wz::service::get_i32(&map, "incJUMP").unwrap_or(0),
    };
    Ok(item)
}

pub async fn get_equip_by_id(state: &SharedState, id: i32) -> Result<Equip, ModelError> {
    let equip_model = equip::query::get_equip_model_by_id(state, id).await?;
    Ok(Equip { model: equip_model })
}

impl EquipModel {
    pub fn new() -> Self {
        Self {
            id: -1,
            wz_id: -1,
            strength: -1,
            dexterity: -1,
            intelligence: -1,
            luck: -1,
            attack: -1,
            weapon_defense: -1,
            magic: -1,
            magic_defense: -1,
            hp: -1,
            mp: -1,
            accuracy: -1,
            avoid: -1,
            hands: -1,
            speed: -1,
            jump: -1,
            created_at: SystemTime::now(),
            updated_at: SystemTime::now(),
        }
    }
}

impl Equip {
    pub fn new() -> Self {
        Self {
            model: EquipModel::new(),
        }
    }
}
