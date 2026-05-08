use crate::models::account::model::AccountModel;
use crate::net::error::NetworkError;

pub fn check_pic(acc_model: AccountModel, pic: String) -> Result<bool, NetworkError> {
    if acc_model.pic == pic {
        return Ok(true);
    } else {
        return Ok(false);
    }
}
