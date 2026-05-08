use crate::models::error::ModelError;
use crate::models::wz::equip::model::{Equip, EquipModel, NewEquipInsert};
use crate::models::wz::{self, equip};
use crate::runtime::state::SharedState;
use futures::future::OptionFuture;

pub fn create_equip_insert(wz_id: i32) -> Result<NewEquipInsert, ModelError> {
    let filename = String::from("Character.wz");
    let map = wz::service::get_img_map(wz_id, &filename)?;
    let item = NewEquipInsert {
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
    Ok(item)
}

pub async fn get_equip_by_id(state: &SharedState, id: i32) -> Result<Equip, ModelError> {
    let equip_model = equip::query::get_equip_model_by_id(state, id).await?;
    Ok(Equip { model: equip_model })
}

impl NewEquipInsert {
    pub fn default(
        wz_id: i32,
        strength: i32,
        dexterity: i32,
        intelligence: i32,
        luck: i32,
        attack: i32,
        weapon_defense: i32,
        magic: i32,
        magic_defense: i32,
        hp: i32,
        mp: i32,
        accuracy: i32,
        avoid: i32,
        hands: i32,
        speed: i32,
        jump: i32,
    ) -> Self {
        Self {
            wz_id,
            strength: Some(strength),
            dexterity: Some(dexterity),
            intelligence: Some(intelligence),
            luck: Some(luck),
            attack: Some(attack),
            weapon_defense: Some(weapon_defense),
            magic: Some(magic),
            magic_defense: Some(magic_defense),
            hp: Some(hp),
            mp: Some(mp),
            accuracy: Some(accuracy),
            avoid: Some(avoid),
            hands: Some(hands),
            speed: Some(speed),
            jump: Some(jump),
        }
    }
}

impl Equip {
    pub fn new(model: EquipModel) -> Self {
        Self { model }
    }
}

pub async fn resolve_equip(
    state: &SharedState,
    id: Option<i32>,
) -> Result<Option<Equip>, ModelError> {
    OptionFuture::from(id.map(|id| get_equip_by_id(state, id)))
        .await
        .transpose()
}
