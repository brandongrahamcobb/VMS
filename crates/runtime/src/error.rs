/* runtime/error.rs
 * The purpose of this module is to provide errors related to the runtime.
 *
 * Copyright (C) 2026  https://github.com/brandongrahamcobb/VMS.git
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use config::error::ConfigError;
use db::account::error::AccountModelError;
use db::character::error::CharacterModelError;
use db::error::DatabaseError;
use inc::error::IncError;
use ipc::command::AsyncCommand;
use ipc::db_command::DatabaseCommand;
use metadata::item::error::ItemMetadataError;
use metadata::map::error::MapMetadataError;
use net::packet::io::error::IOError;
use thiserror::Error;
use tokio::sync::mpsc::error::SendError;

use crate::tcp::Register;

#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("Configuration error in runtime layer")]
    ConfigError(#[from] ConfigError),

    #[error("Generic error in runtime layer")]
    Error(#[from] std::io::Error),

    #[error("Packet io error in runtime layer")]
    IOError(#[from] IOError),

    #[error("Database error in runtime layer")]
    DatabaseError(#[from] DatabaseError),

    #[error("Tokio asyncronous command send error in runtime layer")]
    TokioSendAsyncCommandError(#[from] SendError<AsyncCommand>),

    #[error("Tokio db command send error in runtime layer")]
    TokioSendDbCommandError(#[from] SendError<DatabaseCommand>),

    #[error("Tokio register send error in runtime layer")]
    TokioSendRegisterError(#[from] SendError<Register>),

    #[error("Item metadata error in runtime layer")]
    ItemMetadataError(#[from] ItemMetadataError),

    #[error("Map metadata error in runtime layer")]
    MapMetadataError(#[from] MapMetadataError),

    #[error("Character db model error in runtime layer")]
    CharacterModelError(#[from] CharacterModelError),

    #[error("Account db model error in runtime layer")]
    AccountModelError(#[from] AccountModelError),

    #[error("Inc error in runtime layer")]
    IncError(#[from] IncError),
}
