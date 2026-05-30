use entity::character::{model::CharacterModel, wrapper::Character};

pub async fn list_chars(
    pool: &DbPool,
    acc_id: i32,
    world_id: i16,
) -> Result<Vec<Characters>, DomainError> {
    let char_models: Vec<CharacterModel> =
        db::character::getters::get_char_models_by_acc_id(pool, acc_id).await?;
    let mut chars: Vec<Character> = Vec::new();
    for char_model in char_models {
        if char_model.world_id == world_id {
            let char: Character =
                assembly::character::assemble::assemble_char_by_id(pool, char_id).await?;
            chars.push(char);
        }
    }
    Ok(chars)
}
