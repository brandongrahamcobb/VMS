/* handler/error.rs
 * The purpose of this module is to provide errors related to handlers.
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

use crate::cc::error::ChangeChannelEntityError;
use crate::change_keymap::error::ChangeKeymapError;
use crate::change_map::error::ChangeMapEntityError;
use crate::chat_text::error::ChatTextError;
use crate::check_char_name::error::CheckCharNameError;
use crate::close_attack::error::CloseAttackError;
use crate::create_char::error::CreateCharError;
use crate::credentials::error::CredentialsError;
use crate::delete_char::error::DeleteCharError;
use crate::enter_cash_shop::error::EnterCashShopError;
use crate::list_chars::error::ListCharsError;
use crate::list_worlds::error::ListWorldsError;
use crate::login_start::error::LoginStartError;
use crate::mob_ai::error::MobAiError;
use crate::move_player::error::MovePlayerError;
use crate::party_search::error::PartySearchError;
use crate::pickup_item::error::PickupItemEntityError;
use crate::player_logged_in::error::PlayerLoggedInError;
use crate::player_map_transfer::error::PlayerMapTransferError;
use crate::register_pic::error::RegisterPicError;
use crate::select_char::error::SelectCharError;
use crate::select_char_with_pic::error::SelectCharWithPicError;
use crate::server_status::error::ServerStatusError;
use crate::spw::error::SpwError;
use crate::take_damage::error::TakeDamageError;
use crate::tos::error::TosError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PacketHandlerError {
    #[error("Change channel error in packet handler layer")]
    ChangeChannelEntityError(#[from] ChangeChannelEntityError),

    #[error("Change keymap error in packet handler layer")]
    ChangeKeymapError(#[from] ChangeKeymapError),

    #[error("Change map error in packet handler layer")]
    ChangeMapEntityError(#[from] ChangeMapEntityError),

    #[error("Chat text error in packet handler layer")]
    ChatTextError(#[from] ChatTextError),

    #[error("Check character name error in packet handler layer")]
    CheckCharNameError(#[from] CheckCharNameError),

    #[error("Close attack error in packet handler layer")]
    CloseAttackError(#[from] CloseAttackError),

    #[error("Create character error in packet handler layer")]
    CreateCharError(#[from] CreateCharError),

    #[error("Credentials error in packet handler layer")]
    CredentialsError(#[from] CredentialsError),

    #[error("Delete character error in packet handler layer")]
    DeleteCharError(#[from] DeleteCharError),

    #[error("Enter cash shop error in packet handler layer")]
    EnterCashShopError(#[from] EnterCashShopError),

    #[error("List characters error in packet handler layer")]
    ListCharsError(#[from] ListCharsError),

    #[error("List worlds error in packet handler layer")]
    ListWorldsError(#[from] ListWorldsError),

    #[error("Login start error in packet handler layer")]
    LoginStartError(#[from] LoginStartError),

    #[error("Mob AI error in packet handler layer")]
    MobAiError(#[from] MobAiError),

    #[error("Move player error in packet handler layer")]
    MovePlayerError(#[from] MovePlayerError),

    #[error("Party search error in packet handler layer")]
    PartySearchError(#[from] PartySearchError),

    #[error("Pickup item error in packet handler layer")]
    PickupItemEntityError(#[from] PickupItemEntityError),

    #[error("Player logged in error in packet handler layer")]
    PlayerLoggedInError(#[from] PlayerLoggedInError),

    #[error("Player map transfer error in packet handler layer")]
    PlayerMapTransferError(#[from] PlayerMapTransferError),

    #[error("Register PIC error in packet handler layer")]
    RegisterPicError(#[from] RegisterPicError),

    #[error("Select character error in packet handler layer")]
    SelectCharError(#[from] SelectCharError),

    #[error("Select character with PIC error in packet handler layer")]
    SelectCharWithPicError(#[from] SelectCharWithPicError),

    #[error("Server status error in packet handler layer")]
    ServerStatusError(#[from] ServerStatusError),

    #[error("SPW error in packet handler layer")]
    SpwError(#[from] SpwError),

    #[error("Take damage error in packet handler layer")]
    TakeDamageError(#[from] TakeDamageError),

    #[error("TOS error in packet handler layer")]
    TosError(#[from] TosError),
}
