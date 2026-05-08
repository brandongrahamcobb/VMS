use crate::{
    models::{
        character::{
            equipment_set, keybinding,
            model::{Character, CharacterModel},
            skill,
        },
        error::ModelError,
        map, world,
    },
    runtime::state::SharedState,
};

pub async fn get_character_from_model(
    state: &SharedState,
    model: CharacterModel,
) -> Result<Character, ModelError> {
    let bind_set_model = keybinding::query::get_keybindings_by_character_id(state, model.id)?;
    let android_equips_model =
        equipment_set::android::query::get_android_equipment_set_model_by_character_id(
            state, model.id,
        )
        .await?;
    let android_equips = equipment_set::android::service::get_android_equipment_set_from_model(
        state,
        android_equips_model.clone(),
    )
    .await?;
    let cash_equips_model =
        equipment_set::cash::query::get_cash_equipment_set_model_by_character_id(state, model.id)
            .await?;
    let cash_equips = equipment_set::cash::service::get_cash_equipment_set_from_model(
        state,
        cash_equips_model.clone(),
    )
    .await?;
    let pet_equips_model =
        equipment_set::pet::query::get_pet_equipment_set_model_by_character_id(state, model.id)
            .await?;
    let pet_equips = equipment_set::pet::service::get_pet_equipment_set_from_model(
        state,
        pet_equips_model.clone(),
    )
    .await?;
    let regular_equips_model =
        equipment_set::regular::query::get_regular_equipment_set_model_by_character_id(
            state, model.id,
        )
        .await?;
    let regular_equips = equipment_set::regular::service::get_regular_equipment_set_from_model(
        state,
        regular_equips_model.clone(),
    )
    .await?;
    let skills = skill::query::get_skills_by_character_id(state, model.id).await?;
    let map = map::service::get_map_by_id(model.map_id)?;
    let world = world::service::get_world_by_id(state, model.world_id)?;
    Ok(Character {
        model,
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
