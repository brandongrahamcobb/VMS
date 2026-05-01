use crate::models::error::ModelError;
use crate::models::wz;
// use crate::models::wz::equip::error::EquipError;
use crate::models::wz::equip::model::NewEquip;

pub fn generate_new_equip(id: i32, filename: String) -> Result<NewEquip, ModelError> {
    let filename = String::from("Character.wz");
    let map = wz::service::get_img_map(id, filename).map_err(ModelError::from)?;
    let item = NewEquip {
        wz_id: id,
        strength: wz::service::get_i32(&map, "incSTR"),
        dexterity: wz::service::get_i32(&map, "incDEX"),
        intelligence: wz::service::get_i32(&map, "incINT"),
        luck: wz::service::get_i32(&map, "incLUK"),
        attack: wz::service::get_i32(&map, "incPAD"),
        weapon_defense: wz::service::get_i32(&map, "incPDD"),
        magic: wz::service::get_i32(&map, "incMAD"),
        magic_defense: wz::service::get_i32(&map, "incMDD"),
        hp: wz::service::get_i32(&map, "incMHP"),
        mp: wz::service::get_i32(&map, "incMMP"),
        accuracy: wz::service::get_i32(&map, "incACC"),
        avoid: wz::service::get_i32(&map, "incEVA"),
        hands: wz::service::get_i32(&map, "incHANDS"),
        speed: wz::service::get_i32(&map, "incSPEED"),
        jump: wz::service::get_i32(&map, "incJUMP"),
    };
    Ok(item)
}
