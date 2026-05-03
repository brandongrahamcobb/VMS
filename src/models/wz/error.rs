use crate::models::wz::equip::error::WzEquipError;

#[derive(Debug, thiserror::Error)]
pub enum WzError {
    #[error("Requested Wz resource was not found in wz model layer: {0}")]
    NotFound(i32),

    #[error("Wz anyhow error in wz model layer")]
    AnyHowError(#[from] anyhow::Error),

    #[error("Wz binrw error in wz model layer")]
    BinRwError(#[from] binrw::error::Error),

    #[error("Wz version error in wz model layer")]
    NoVersion,

    #[error("Wz io error in wz model layer")]
    IOError(#[from] std::io::Error),

    #[error("Equip error in wz model layer")]
    EquipError(#[from] WzEquipError),

    #[error("WzDirEntry error in wz model layer")]
    EntryError,

    #[error("Wz part error in wz model layer")]
    PartError,

    #[error("Wz Object error in wz model layer")]
    ObjectError,
}
