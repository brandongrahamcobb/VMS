use std::time::SystemTime;

use crate::models::error::ModelError;
use crate::models::wz;
use crate::models::wz::equip::model::{Equip, NewEquipInsert};
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
