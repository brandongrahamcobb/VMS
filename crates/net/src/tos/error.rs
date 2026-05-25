/* tos/error.rs
 * The purpose of this module is to provide errors related to terms of service.
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

use assembly::account::error::AccountAssemblyError;
use db::error::DatabaseError;
use entity::account::error::AccountEntityError;
use packet::build::error::PacketBuildError;
use packet::io::error::IOError;
use session::error::SessionError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TosError {
    #[error("Packet io error in terms of service layer")]
    IOError(#[from] IOError),

    #[error("Session error in terms of service layer")]
    SessionError(#[from] SessionError),

    #[error("Account assembly error in terms of service layer")]
    AccountAssemblyError(#[from] AccountAssemblyError),

    #[error("Account entity error in terms of service layer")]
    AccountEntitykError(#[from] AccountEntityError),

    #[error("Database error in terms of service layer")]
    DatabaseError(#[from] DatabaseError),

    #[error("Packet build error in terms of service layer")]
    PacketBuildError(#[from] PacketBuildError),
}
