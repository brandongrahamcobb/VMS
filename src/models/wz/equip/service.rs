use crate::db::error::DatabaseError;
use crate::models::error::ModelError;
use crate::models::wz;
use crate::models::wz::equip::error::WzEquipError;
use crate::models::wz::equip::model::{Equip, NewEquip};
use crate::models::wz::error::WzError;
use crate::runtime::state::SharedState;

pub async fn generate_new_equip(state: SharedState, wz_id: i32) -> Result<Equip, ModelError> {
    let filename = String::from("Character.wz");
    let map = wz::service::get_img_map(wz_id, filename).map_err(ModelError::from)?;
    let new_item = NewEquip {
        wz_id: wz_id,
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
    let item = wz::equip::query::create_equip(state.clone(), new_item)
        .await
        .map_err(DatabaseError::from)
        .map_err(WzEquipError::from)
        .map_err(WzError::from)
        .map_err(ModelError::from)?;
    Ok(item)
}
