use entity::character::{model::CharacterModel, wrapper::Character};

use crate::error::DomainError;

pub async fn load_characters_by_acc_id(
    pool: &DbPool,
    acc_id: i32,
) -> Result<Vec<Character>, DomainError> {
    let mut chars: Vec<Character> = Vec::new();
    let models: Vec<CharacterModel> = db::character::getters::get_by_acc_id(&pool, acc_id).await?;
    for model in models {
        let char: Character = assembly::character::assemble::assemble_char_by_id(&db, id).await?;
        chars.push(char);
    }
    Ok(chars)
}

pub fn check_pic(acc_pic: Option<String>, pic: String) -> bool {
    acc_pic == Some(pic)
}

pub fn authenticate(acc_pw: String, pw: String) -> Result<bool, IPCError> {
    verify(&pw, &acc_pw).map_err(IPCError::CryptError)
}

pub fn get_status_code_by_account(acc_model: &AccountModel) -> StatusCode {
    if acc_model.banned {
        return Ok(StatusCode::Failed(FailedCode::Banned));
    }
    if !acc_model.accepted_tos {
        return Ok(StatusCode::Pending(PendingCode::PendingTOS));
    }
    Ok(StatusCode::Success(SuccessCode::Success))
}
