use crate::models::character::equipment_set::regular::model::{
    RegularEquipmentSet, RegularEquipmentSetModel,
};
use crate::models::error::ModelError;
use crate::models::wz;
use crate::runtime::state::SharedState;

pub async fn get_regular_equipment_set_from_model(
    state: &SharedState,
    model: RegularEquipmentSetModel,
) -> Result<RegularEquipmentSet, ModelError> {
    Ok(RegularEquipmentSet {
        model,
        hat: wz::equip::query::get_equip_by_id(state, model.hat_id).await?,
        face_acc: wz::equip::query::get_equip_by_id(state, model.face_acc_id).await?,
        eye_acc: wz::equip::query::get_equip_by_id(state, model.eye_acc_id).await?,
        ear_acc: wz::equip::query::get_equip_by_id(state, model.ear_acc_id).await?,
        top: wz::equip::query::get_equip_by_id(state, model.top_id).await?,
        bottom: wz::equip::query::get_equip_by_id(state, model.bottom_id).await?,
        shoes: wz::equip::query::get_equip_by_id(state, model.shoes_id).await?,
        gloves: wz::equip::query::get_equip_by_id(state, model.gloves_id).await?,
        cape: wz::equip::query::get_equip_by_id(state, model.cape_id).await?,
        shield: wz::equip::query::get_equip_by_id(state, model.shield_id).await?,
        weapon: wz::equip::query::get_equip_by_id(state, model.weapon_id).await?,
        ring_one: wz::equip::query::get_equip_by_id(state, model.ring_one_id).await?,
        ring_two: wz::equip::query::get_equip_by_id(state, model.ring_two_id).await?,
        ring_three: wz::equip::query::get_equip_by_id(state, model.ring_three_id).await?,
        ring_four: wz::equip::query::get_equip_by_id(state, model.ring_four_id).await?,
        pendant_one: wz::equip::query::get_equip_by_id(state, model.pendant_one_id).await?,
        tamed_mob: wz::equip::query::get_equip_by_id(state, model.tamed_mob_id).await?,
        saddle: wz::equip::query::get_equip_by_id(state, model.saddle_id).await?,
        medal: wz::equip::query::get_equip_by_id(state, model.medal_id).await?,
        belt: wz::equip::query::get_equip_by_id(state, model.belt_id).await?,
        pocket: wz::equip::query::get_equip_by_id(state, model.pocket_id).await?,
        book: wz::equip::query::get_equip_by_id(state, model.book_id).await?,
        pendant_two: wz::equip::query::get_equip_by_id(state, model.pendant_two_id).await?,
        shoulder: wz::equip::query::get_equip_by_id(state, model.shoulder_id).await?,
        android: wz::equip::query::get_equip_by_id(state, model.android_id).await?,
        emblem: wz::equip::query::get_equip_by_id(state, model.emblem_id).await?,
        badge: wz::equip::query::get_equip_by_id(state, model.badge_id).await?,
        subweapon: wz::equip::query::get_equip_by_id(state, model.subweapon_id).await?,
        heart: wz::equip::query::get_equip_by_id(state, model.heart_id).await?,
    })
}
