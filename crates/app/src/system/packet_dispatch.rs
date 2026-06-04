/* app/src/system/packet_dispatch.rs
 * The purpose of this module is to provide a system for dispatching packets.
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

use std::sync::MutexGuard;
use std::sync::mpsc::Receiver;

use crate::message::packet::accept_tos::ReadTosRequestMessage;
use crate::message::packet::attack_close::ReadCloseAttackRequestMessage;
use crate::message::packet::cc::ReadChangeChannelRequestMessage;
use crate::message::packet::change_keymap::ReadChangeKeymapRequestMessage;
use crate::message::packet::change_map::ReadChangeMapRequestMessage;
use crate::message::packet::chat_text::ReadChatTextRequestMessage;
use crate::message::packet::check_char_name::ReadCheckCharNameRequestMessage;
use crate::message::packet::create_char::ReadCreateCharRequestMessage;
use crate::message::packet::delete_char::ReadDeleteCharRequestMessage;
use crate::message::packet::enter_cash_shop::ReadEnterCashShopRequestMessage;
use crate::message::packet::list_chars::ReadListCharsRequestMessage;
use crate::message::packet::list_worlds::ReadListWorldsRequestMessage;
use crate::message::packet::login::ReadLoginRequestMessage;
use crate::message::packet::login_started::ReadLoginStartRequestMessage;
use crate::message::packet::mob_moved::ReadMobMovedRequestMessage;
use crate::message::packet::party_search::ReadPartySearchRequestMessage;
use crate::message::packet::pickup_item::ReadPickupItemRequestMessage;
use crate::message::packet::player_logged_in::ReadPlayerLoggedInRequestMessage;
use crate::message::packet::player_map_transferred::ReadPlayerMapTransferRequestMessage;
use crate::message::packet::player_moved::ReadPlayerMovedRequestMessage;
use crate::message::packet::raw::RawPacketMessage;
use crate::message::packet::register_pic::ReadRegisterPicRequestMessage;
use crate::message::packet::select_char::ReadSelectCharRequestMessage;
use crate::message::packet::select_char_with_pic::ReadSelectCharWithPicRequestMessage;
use crate::message::packet::server_status::ReadServerStatusRequestMessage;
use crate::message::packet::take_damage::ReadTakeDamageRequestMessage;
use crate::resource::custom_resource::CustomReceiver;
use crate::system::packet::dispatch::{
    accept_tos, attack_close, cc, change_keymap, change_map, chat_text, check_char_name,
    create_char, delete_char, list_chars, login, mob_moved, pickup_item, player_logged_in,
    player_moved, register_pic, select_char, select_char_with_pic, take_damage,
};

use bevy::ecs::message::MessageReader;
use bevy::ecs::system::Res;
use bevy::prelude::MessageWriter;
use ipc::asyncronous::event::AsyncEvent;
use net::packet::model::Packet;
use op::recv::RecvOpcode;

pub fn packet_dispatch_system(
    receiver: Res<CustomReceiver>,
    mut writer: MessageWriter<RawPacketMessage>,
) {
    let rx: MutexGuard<Receiver<AsyncEvent>> = receiver.0.lock().unwrap();
    while let Ok(message) = rx.try_recv() {
        if let AsyncEvent::PacketReceived { client_id, packet } = message {
            writer.write(RawPacketMessage { client_id, packet });
        }
    }
}

pub fn login_packet_router_system(
    mut raw: MessageReader<RawPacketMessage>,
    mut login_writer: MessageWriter<ReadLoginRequestMessage>,
    mut tos_writer: MessageWriter<ReadTosRequestMessage>,
) {
    for msg in raw.read() {
        let client_id: i32 = msg.client_id;
        let packet: Packet = msg.packet.clone();
        match packet.opcode() {
            x if x == RecvOpcode::RequestLogin as i16 => {
                let Ok(msg) = login::read_credentials_packet(&packet, client_id) else {
                    continue;
                };
                login_writer.write(msg);
            }
            x if x == RecvOpcode::AcceptTOS as i16 => {
                let Ok(msg) = accept_tos::read_tos_packet(&packet, client_id) else {
                    continue;
                };
                tos_writer.write(msg);
            }
            _ => {}
        }
    }
}

pub fn prepare_chars_router_system(
    mut raw: MessageReader<RawPacketMessage>,
    mut list_chars_writer: MessageWriter<ReadListCharsRequestMessage>,
    mut list_worlds_writer: MessageWriter<ReadListWorldsRequestMessage>,
    mut server_status_writer: MessageWriter<ReadServerStatusRequestMessage>,
) {
    for msg in raw.read() {
        let client_id: i32 = msg.client_id;
        let packet: Packet = msg.packet.clone();
        match packet.opcode() {
            x if x == RecvOpcode::CharListRequest as i16 => {
                let Ok(msg) = list_chars::read_list_chars_packet(&packet, client_id) else {
                    continue;
                };
                list_chars_writer.write(msg);
            }
            x if x == RecvOpcode::ServerListRequest as i16 => {
                let msg: ReadListWorldsRequestMessage = ReadListWorldsRequestMessage { client_id };
                list_worlds_writer.write(msg);
            }
            x if x == RecvOpcode::ServerStatusRequest as i16 => {
                let msg: ReadServerStatusRequestMessage =
                    ReadServerStatusRequestMessage { client_id };
                server_status_writer.write(msg);
            }
            _ => {}
        }
    }
}

pub fn char_management_router_system(
    mut raw: MessageReader<RawPacketMessage>,
    mut create_char_writer: MessageWriter<ReadCreateCharRequestMessage>,
    mut check_char_name_writer: MessageWriter<ReadCheckCharNameRequestMessage>,
    mut delete_char_writer: MessageWriter<ReadDeleteCharRequestMessage>,
    mut select_char_writer: MessageWriter<ReadSelectCharRequestMessage>,
    mut select_char_pic_writer: MessageWriter<ReadSelectCharWithPicRequestMessage>,
    mut register_pic_writer: MessageWriter<ReadRegisterPicRequestMessage>,
) {
    for msg in raw.read() {
        let client_id: i32 = msg.client_id;
        let packet: Packet = msg.packet.clone();
        match packet.opcode() {
            x if x == RecvOpcode::CreateChar as i16 => {
                let Ok(msg) = create_char::read_create_character_packet(&packet, client_id) else {
                    continue;
                };
                create_char_writer.write(msg);
            }
            x if x == RecvOpcode::CheckCharName as i16 => {
                let Ok(msg) = check_char_name::read_check_char_name_packet(&packet, client_id)
                else {
                    continue;
                };
                check_char_name_writer.write(msg);
            }
            x if x == RecvOpcode::DeleteChar as i16 => {
                let Ok(msg) = delete_char::read_delete_char_packet(&packet, client_id) else {
                    continue;
                };
                delete_char_writer.write(msg);
            }
            x if x == RecvOpcode::CharSelect as i16 => {
                let Ok(msg) = select_char::read_select_char_packet(&packet, client_id) else {
                    continue;
                };
                select_char_writer.write(msg);
            }
            x if x == RecvOpcode::CharSelectWithPic as i16 => {
                let Ok(msg) =
                    select_char_with_pic::read_select_char_with_pic_packet(&packet, client_id)
                else {
                    continue;
                };
                select_char_pic_writer.write(msg);
            }
            x if x == RecvOpcode::RegisterPic as i16 => {
                let Ok(msg) = register_pic::read_register_pic_packet(&packet, client_id) else {
                    continue;
                };
                register_pic_writer.write(msg);
            }
            _ => {}
        }
    }
}

pub fn start_playing_router_system(
    mut raw: MessageReader<RawPacketMessage>,
    mut login_started_writer: MessageWriter<ReadLoginStartRequestMessage>,
    mut player_logged_in_writer: MessageWriter<ReadPlayerLoggedInRequestMessage>,
) {
    for msg in raw.read() {
        let client_id: i32 = msg.client_id;
        let packet: Packet = msg.packet.clone();
        match packet.opcode() {
            x if x == RecvOpcode::LoginStarted as i16 => {
                let msg: ReadLoginStartRequestMessage = ReadLoginStartRequestMessage { client_id };
                login_started_writer.write(msg);
            }
            x if x == RecvOpcode::PlayerLoggedIn as i16 => {
                let Ok(msg) = player_logged_in::read_player_logged_in_packet(&packet, client_id)
                else {
                    continue;
                };
                player_logged_in_writer.write(msg);
            }
            _ => {}
        }
    }
}

pub fn ui_router_system(
    mut raw: MessageReader<RawPacketMessage>,
    mut change_keymap_writer: MessageWriter<ReadChangeKeymapRequestMessage>,
    mut party_search_writer: MessageWriter<ReadPartySearchRequestMessage>,
) {
    for msg in raw.read() {
        let client_id: i32 = msg.client_id;
        let packet: Packet = msg.packet.clone();
        match packet.opcode() {
            x if x == RecvOpcode::ChangeKeymap as i16 => {
                let Ok(msg) = change_keymap::read_change_keymap_packet(&packet, client_id) else {
                    continue;
                };
                change_keymap_writer.write(msg);
            }
            x if x == RecvOpcode::PartySearch as i16 => {
                let msg: ReadPartySearchRequestMessage =
                    ReadPartySearchRequestMessage { client_id };
                party_search_writer.write(msg);
            }
            _ => {}
        }
    }
}

pub fn map_router_system(
    mut raw: MessageReader<RawPacketMessage>,
    mut change_map_writer: MessageWriter<ReadChangeMapRequestMessage>,
    mut enter_cash_shop_writer: MessageWriter<ReadEnterCashShopRequestMessage>,
    mut player_map_transfer_writer: MessageWriter<ReadPlayerMapTransferRequestMessage>,
) {
    for msg in raw.read() {
        let client_id: i32 = msg.client_id;
        let packet: Packet = msg.packet.clone();
        match packet.opcode() {
            x if x == RecvOpcode::ChangeMap as i16 => {
                let Ok(msg) = change_map::read_change_map_packet(&packet, client_id) else {
                    continue;
                };
                change_map_writer.write(msg);
            }
            x if x == RecvOpcode::EnterCashShop as i16 => {
                let msg: ReadEnterCashShopRequestMessage =
                    ReadEnterCashShopRequestMessage { client_id };
                enter_cash_shop_writer.write(msg);
            }
            x if x == RecvOpcode::PlayerMapTransfer as i16 => {
                let msg: ReadPlayerMapTransferRequestMessage =
                    ReadPlayerMapTransferRequestMessage { client_id };
                player_map_transfer_writer.write(msg);
            }
            _ => {}
        }
    }
}

pub fn channel_router_system(
    mut raw: MessageReader<RawPacketMessage>,
    mut change_channel_writer: MessageWriter<ReadChangeChannelRequestMessage>,
) {
    for msg in raw.read() {
        let client_id: i32 = msg.client_id;
        let packet: Packet = msg.packet.clone();
        match packet.opcode() {
            x if x == RecvOpcode::ChangeChannel as i16 => {
                let Ok(msg) = cc::read_change_channel_packet(&packet, client_id) else {
                    continue;
                };
                change_channel_writer.write(msg);
            }
            _ => {}
        }
    }
}

pub fn char_router_system(
    mut raw: MessageReader<RawPacketMessage>,
    mut all_chat_writer: MessageWriter<ReadChatTextRequestMessage>,
) {
    for msg in raw.read() {
        let client_id: i32 = msg.client_id;
        let packet: Packet = msg.packet.clone();
        match packet.opcode() {
            x if x == RecvOpcode::AllChat as i16 => {
                let Ok(msg) = chat_text::read_chat_text_packet(&packet, client_id) else {
                    continue;
                };
                all_chat_writer.write(msg);
            }
            _ => {}
        }
    }
}

pub fn item_router_system(
    mut raw: MessageReader<RawPacketMessage>,
    mut pickup_item_writer: MessageWriter<ReadPickupItemRequestMessage>,
) {
    for msg in raw.read() {
        let client_id: i32 = msg.client_id;
        let packet: Packet = msg.packet.clone();
        match packet.opcode() {
            x if x == RecvOpcode::PickupItem as i16 => {
                let Ok(msg) = pickup_item::read_pickup_item_packet(&packet, client_id) else {
                    continue;
                };
                pickup_item_writer.write(msg);
            }
            _ => {}
        }
    }
}

pub fn move_router_system(
    mut raw: MessageReader<RawPacketMessage>,
    mut player_attacked_writer: MessageWriter<ReadCloseAttackRequestMessage>,
    mut player_moved_writer: MessageWriter<ReadPlayerMovedRequestMessage>,
    mut mob_moved_writer: MessageWriter<ReadMobMovedRequestMessage>,
    mut take_damage_writer: MessageWriter<ReadTakeDamageRequestMessage>,
) {
    for msg in raw.read() {
        let client_id: i32 = msg.client_id;
        let packet: Packet = msg.packet.clone();
        match packet.opcode() {
            x if x == RecvOpcode::PlayerMoved as i16 => {
                let Ok(msg) = player_moved::read_move_player_packet(&packet, client_id) else {
                    continue;
                };
                player_moved_writer.write(msg);
            }
            x if x == RecvOpcode::MobMoved as i16 => {
                let Ok(msg) = mob_moved::read_mob_ai_packet(&packet, client_id) else {
                    continue;
                };
                mob_moved_writer.write(msg);
            }
            x if x == RecvOpcode::CloseAttack as i16 => {
                let Ok(msg) = attack_close::read_close_attack_packet(&packet, client_id) else {
                    continue;
                };
                player_attacked_writer.write(msg);
            }
            x if x == RecvOpcode::TakeDamage as i16 => {
                let Ok(msg) = take_damage::read_take_damage_packet(&packet, client_id) else {
                    continue;
                };
                take_damage_writer.write(msg);
            }
            _ => {}
        }
    }
}
