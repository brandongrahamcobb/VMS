use crate::models::error::ModelError;
use crate::models::item::equip;
use crate::models::item::equip::model::{Equip, EquipModel};
use crate::runtime::state::SharedState;
use crate::wz;
use crate::wz::error::WzError;
use futures::future::OptionFuture;
use serde_json::Value;
use std::time::SystemTime;

pub fn deserialize(wz_id: i32) -> Result<EquipModel, ModelError> {
    let filename = String::from("Character.wz");
    let root = wz::service::get_img_root(wz_id, &filename)?;
    let item = EquipModel {
        id: None,
        wz_id: wz_id,
        strength: get_equip_stat(&root, "incSTR").unwrap_or(0),
        dexterity: get_equip_stat(&root, "incDEX").unwrap_or(0),
        intelligence: get_equip_stat(&root, "incINT").unwrap_or(0),
        luck: get_equip_stat(&root, "incLUK").unwrap_or(0),
        attack: get_equip_stat(&root, "incPAD").unwrap_or(0),
        weapon_defense: get_equip_stat(&root, "incPDD").unwrap_or(0),
        magic: get_equip_stat(&root, "incMAD").unwrap_or(0),
        magic_defense: get_equip_stat(&root, "incMDD").unwrap_or(0),
        hp: get_equip_stat(&root, "incMHP").unwrap_or(0),
        mp: get_equip_stat(&root, "incMMP").unwrap_or(0),
        accuracy: get_equip_stat(&root, "incACC").unwrap_or(0),
        avoid: get_equip_stat(&root, "incEVA").unwrap_or(0),
        hands: get_equip_stat(&root, "incHANDS").unwrap_or(0),
        speed: get_equip_stat(&root, "incSPEED").unwrap_or(0),
        jump: get_equip_stat(&root, "incJUMP").unwrap_or(0),
        created_at: Some(SystemTime::now()),
        updated_at: SystemTime::now(),
    };
    Ok(item)
}

pub async fn get_equip_by_id(state: &SharedState, id: i32) -> Result<Equip, ModelError> {
    let equip_model = equip::query::getters::get_equip_model_by_id(state, id).await?;
    Ok(Equip { model: equip_model })
}

pub async fn resolve_equip(
    state: &SharedState,
    id: Option<i32>,
) -> Result<Option<Equip>, ModelError> {
    OptionFuture::from(id.map(|id| get_equip_by_id(state, id)))
        .await
        .transpose()
}

fn get_equip_stat(root: &Value, key: &str) -> Option<i32> {
    let map = root.get("info").ok_or(WzError::ObjectError).ok()?;
    map.get(key).and_then(|v| v.as_i64().map(|n| n as i32))
}
