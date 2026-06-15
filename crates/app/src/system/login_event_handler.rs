/* app/src/system/login_event_handler.rs
 * The purpose of this module is to provide a system for handling raw login events.
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

use crate::message::packet::check_char_name::CheckCharNameResponseMessage;
use crate::message::packet::create_char::CreateCharResponseMessage;
use crate::message::packet::list_chars::{
    ListCharsFailedResponseMessage, ListCharsSuccessResponseMessage,
};
use crate::message::packet::login::{
    InvalidLoginAccountResponseMessage, ValidLoginAccountResponseMessage,
};
use crate::message::packet::select_char_with_pic::SelectCharWithPicResponseMessage;
use crate::system::event::RawEvent;
use bevy::ecs::message::{MessageReader, MessageWriter};
use ipc::event::AsyncEvent;

pub fn handle_login_events_system(
    mut messages: MessageReader<RawEvent>,
    mut check_char_name_response_writer: MessageWriter<CheckCharNameResponseMessage>,
    mut create_char_response_writer: MessageWriter<CreateCharResponseMessage>,
    mut list_chars_success_writer: MessageWriter<ListCharsSuccessResponseMessage>,
    mut list_chars_fail_writer: MessageWriter<ListCharsFailedResponseMessage>,
    mut login_valid_writer: MessageWriter<ValidLoginAccountResponseMessage>,
    mut login_invalid_writer: MessageWriter<InvalidLoginAccountResponseMessage>,
    mut select_char_success_writer: MessageWriter<SelectCharWithPicResponseMessage>,
) {
    for msg in messages.read() {
        match msg {
            RawEvent::LoginValid(event) => match event {
                AsyncEvent::LoginValid {
                    client_id,
                    acc_id,
                    acc_model,
                    code,
                } => {
                    login_valid_writer.write(ValidLoginAccountResponseMessage {
                        client_id: *client_id,
                        acc_id: *acc_id,
                        acc_model: acc_model.clone(),
                        code: *code,
                    });
                }
                _ => {}
            },
            RawEvent::LoginInvalid(event) => match event {
                AsyncEvent::LoginInvalid { client_id, code } => {
                    login_invalid_writer.write(InvalidLoginAccountResponseMessage {
                        client_id: *client_id,
                        code: code.clone(),
                    });
                }
                _ => {}
            },
            RawEvent::SelectCharWithPic(event) => match event {
                AsyncEvent::SelectCharWithPic {
                    client_id,
                    char_id,
                    status,
                } => {
                    select_char_success_writer.write(SelectCharWithPicResponseMessage {
                        client_id: *client_id,
                        char_id: *char_id,
                        status: *status,
                    });
                }
                _ => {}
            },
            RawEvent::ListCharsSuccess(event) => match event {
                AsyncEvent::ListCharsSuccess {
                    client_id,
                    channel_id,
                    char_models,
                    keybinding_model_map,
                    skill_model_map,
                    equipped_item_model_map,
                    equip_item_model_map,
                    use_item_model_map,
                    etc_item_model_map,
                    setup_item_model_map,
                    cash_item_model_map,
                    equip_tab_inv_capacity_map,
                    use_tab_inv_capacity_map,
                    etc_tab_inv_capacity_map,
                    setup_tab_inv_capacity_map,
                    cash_tab_inv_capacity_map,
                    slots,
                    world_id,
                } => {
                    list_chars_success_writer.write(ListCharsSuccessResponseMessage {
                        client_id: *client_id,
                        channel_id: *channel_id,
                        char_models: char_models.clone(),
                        keybinding_model_map: keybinding_model_map.clone(),
                        skill_model_map: skill_model_map.clone(),
                        equipped_item_model_map: equipped_item_model_map.clone(),
                        equip_item_model_map: equip_item_model_map.clone(),
                        use_item_model_map: use_item_model_map.clone(),
                        etc_item_model_map: etc_item_model_map.clone(),
                        setup_item_model_map: setup_item_model_map.clone(),
                        cash_item_model_map: cash_item_model_map.clone(),
                        equip_tab_inv_capacity_map: equip_tab_inv_capacity_map.clone(),
                        use_tab_inv_capacity_map: use_tab_inv_capacity_map.clone(),
                        etc_tab_inv_capacity_map: etc_tab_inv_capacity_map.clone(),
                        setup_tab_inv_capacity_map: setup_tab_inv_capacity_map.clone(),
                        cash_tab_inv_capacity_map: cash_tab_inv_capacity_map.clone(),
                        slots: *slots,
                        world_id: *world_id,
                    });
                }
                _ => {}
            },
            RawEvent::ListCharsFailed(event) => match event {
                AsyncEvent::ListCharsFailed { client_id } => {
                    list_chars_fail_writer.write(ListCharsFailedResponseMessage {
                        client_id: *client_id,
                    });
                }
                _ => {}
            },
            RawEvent::CharCreationSuccess(event) => match event {
                AsyncEvent::CharCreationSuccess {
                    client_id,
                    char_model,
                    equipped_item_model_map,
                    equip_item_model_map,
                    use_item_model_map,
                    etc_item_model_map,
                    setup_item_model_map,
                    cash_item_model_map,
                    keybinding_model_map,
                    skill_model_map,
                    equip_tab_inv_capacity_map,
                    use_tab_inv_capacity_map,
                    etc_tab_inv_capacity_map,
                    setup_tab_inv_capacity_map,
                    cash_tab_inv_capacity_map,
                } => {
                    let Some(char_id) = char_model.id else {
                        continue;
                    };
                    create_char_response_writer.write(CreateCharResponseMessage {
                        client_id: *client_id,
                        char_id,
                        char_model: char_model.clone(),
                        keybinding_model_map: keybinding_model_map.clone(),
                        skill_model_map: skill_model_map.clone(),
                        equipped_item_model_map: equipped_item_model_map.clone(),
                        equip_item_model_map: equip_item_model_map.clone(),
                        use_item_model_map: use_item_model_map.clone(),
                        etc_item_model_map: etc_item_model_map.clone(),
                        setup_item_model_map: setup_item_model_map.clone(),
                        cash_item_model_map: cash_item_model_map.clone(),
                        equip_tab_inv_capacity_map: equip_tab_inv_capacity_map.clone(),
                        use_tab_inv_capacity_map: use_tab_inv_capacity_map.clone(),
                        etc_tab_inv_capacity_map: etc_tab_inv_capacity_map.clone(),
                        setup_tab_inv_capacity_map: setup_tab_inv_capacity_map.clone(),
                        cash_tab_inv_capacity_map: cash_tab_inv_capacity_map.clone(),
                    });
                }
                _ => {}
            },
            RawEvent::CheckCharName(event) => match event {
                AsyncEvent::CheckCharName {
                    client_id,
                    exists,
                    ign,
                } => {
                    check_char_name_response_writer.write(CheckCharNameResponseMessage {
                        client_id: *client_id,
                        exists: *exists,
                        ign: ign.clone(),
                    });
                }
                _ => {}
            },
            _ => {}
        }
    }
}
