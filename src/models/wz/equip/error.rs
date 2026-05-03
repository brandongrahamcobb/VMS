use crate::db::error::DatabaseError;

#[derive(Debug, thiserror::Error)]
pub enum WzEquipError {
    #[error("Requested equip resource was not found in equip model layer: {0}")]
    NotFound(i32),

    #[error("Wz read error in wz model layer")]
    ReadError(#[from] anyhow::Error),

    #[error("Wz from reader error in wz model layer")]
    PropertyError(#[from] binrw::error::Error),

    #[error("Wz version error in wz model layer")]
    NoVersion,

    #[error("Wz Base.wz read error in wz model layer")]
    FileNotFound(#[from] std::io::Error),

    #[error("Database error in equip model layer")]
    DatabaseError(#[from] DatabaseError),
}
