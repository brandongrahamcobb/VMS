use crate::error::HarnessError;

pub fn assert_handshake(version: i16, locale: u8) -> Result<(), HarnessError> {
    assert_eq!(version, 83);
    assert_eq!(locale, 8);
    Ok(())
}
