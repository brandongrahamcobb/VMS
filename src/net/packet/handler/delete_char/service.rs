use crate::models::account::model::Account;
use crate::net::error::NetworkError;

pub fn check_pic(acc: &Account, pic: &String) -> Result<bool, NetworkError> {
    if acc.pic.as_ref() == Some(pic) {
        return Ok(true);
    } else {
        return Ok(false);
    }
}
