/* enter_cash_shop/error.rs
 * The purpose of this module is to provide errors related to entering the cash shop.
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
use assembly::character::error::CharacterAssemblyError;
use entity::account::error::AccountEntityError;
use entity::character::error::CharacterEntityError;
use packet::build::error::PacketBuildError;
use packet::io::error::IOError;
use session::error::SessionError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EnterCashShopError {
    #[error("Packet io error in enter cash shop layer")]
    IOError(#[from] IOError),

    #[error("Session error in enter cash shop layer")]
    SessionError(#[from] SessionError),

    #[error("Character entity error in enter cash shop layer")]
    CharacterEntityError(#[from] CharacterEntityError),

    #[error("Account entity error in enter cash shop layer")]
    AccountEntityError(#[from] AccountEntityError),

    #[error("Account assembly error in enter cash shop layer")]
    AccountAssemblyError(#[from] AccountAssemblyError),

    #[error("Character entity error in enter cash shop layer")]
    CharacterAssemblyError(#[from] CharacterAssemblyError),

    #[error("Packet build error in enter cash shop layer")]
    PacketBuildError(#[from] PacketBuildError),
}
