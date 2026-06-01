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

use crate::message::packet::accept_tos::TosMessage;
use crate::message::packet::attack_close::CloseAttackMessage;
use crate::message::packet::cc::ChangeChannelMessage;
use crate::message::packet::change_keymap::ChangeKeymapMessage;
use crate::message::packet::change_map::ChangeMapMessage;
use crate::message::packet::chat_text::ChatTextMessage;
use crate::message::packet::check_char_name::CheckCharNameMessage;
use crate::message::packet::create_char::CreateCharMessage;
use crate::message::packet::credentials::CredentialsMessage;
use crate::message::packet::delete_char::DeleteCharMessage;
use crate::message::packet::enter_cash_shop::EnterCashShopMessage;
use crate::message::packet::list_chars::ListCharsMessage;
use crate::message::packet::list_worlds::ListWorldsMessage;
use crate::message::packet::login_started::LoginStartMessage;
use crate::message::packet::mob_moved::MobMovedMessage;
use crate::message::packet::party_search::PartySearchMessage;
use crate::message::packet::pickup_item::PickupItemMessage;
use crate::message::packet::player_logged_in::PlayerLoggedInMessage;
use crate::message::packet::player_map_transferred::PlayerMapTransferMessage;
use crate::message::packet::player_moved::PlayerMovedMessage;
use crate::message::packet::raw::RawPacketMessage;
use crate::message::packet::register_pic::RegisterPicMessage;
use crate::message::packet::select_char::SelectCharMessage;
use crate::message::packet::select_char_with_pic::SelectCharWithPicMessage;
use crate::message::packet::server_status::ServerStatusMessage;
use crate::message::packet::take_damage::TakeDamageMessage;
use crate::plugin::custom_plugin::CustomReceiver;
use crate::system::packet::dispatch::{
    accept_tos, attack_close, cc, change_keymap, change_map, chat_text, check_char_name,
    create_char, credentials, delete_char, list_chars, list_worlds, mob_moved, pickup_item,
    player_logged_in, player_moved, register_pic, select_char, select_char_with_pic, take_damage,
};

use bevy::ecs::system::Res;
use bevy::prelude::MessageWriter;
use op::recv::RecvOpcode;

pub fn packet_dispatch_system(
    receiver: Res<CustomReceiver>,
    mut writer: MessageWriter<RawPacketMessage>,
) {
    let rx: MutexGuard<Receiver<RawPacketMessage>> = receiver.0.lock().unwrap();
    while let Ok(message) = rx.try_recv() {
        if let AsyncEvent::PacketReceived { client_id, packet } = message {
            writer.write(message);
        }
    }
}

pub fn login_packet_router_system(
    mut raw: MessageReader<RawPacketMessage>,
    mut credentials_writer: MessageWriter<CredentialsMessage>,
    mut tos_writer: MessageWriter<TosMessage>,
) {
    for msg in raw.read() {
        let client_id: i32 = msg.client_id;
        let mut packet: Packet = msg.packet;
        match packet.opcode() {
            x if x == RecvOpcode::RequestLogin => {
                let msg: CredentialsMessage =
                    credentials::read_credentials_packet(&packet, client_id);
                credentials_writer.write(msg);
            }
            x if x == RecvOpcode::AcceptTOS => {
                let msg: TosMessage = accept_tos::read_tos_packet(&packet, client_id);
                tos_writer.write(msg);
            }
            _ => {}
        }
    }
}

pub fn prepare_chars_router_system(
    mut raw: MessageReader<RawPacketMessage>,
    mut list_chars_writer: MessageWriter<ListCharsMessage>,
    mut list_worlds_writer: MessageWriter<ListWorldsMessage>,
    mut server_status_writer: MessageWriter<ServerStatusMessage>,
) {
    for msg in raw.read() {
        let client_id: i32 = msg.client_id;
        let mut packet: Packet = msg.packet;
        match packet.opcode() {
            x if x == RecvOpcode::CharListRequest => {
                let msg: ListCharsMessage = list_chars::read_list_chars_packet(&packet, client_id);
                list_chars_writer.write(msg);
            }
            x if x == RecvOpcode::ServerListRequest => {
                let msg: ListWorldsMessage = ListWorldsMessage { client_id };
                list_worlds_writer.write(msg);
            }
            x if x == RecvOpcode::ServerStatusRequest => {
                let msg: ServerStatusMessage = ServerStatusMessage { client_id };
                server_status_writer.write(msg);
            }
            _ => {}
        }
    }
}

pub fn char_management_router_system(
    mut raw: MessageReader<RawPacketMessage>,
    mut create_char_writer: MessageWriter<CreateCharMessage>,
    mut check_char_name_writer: MessageWriter<CheckCharNameMessage>,
    mut delete_char_writer: MessageWriter<DeleteCharMessage>,
    mut select_char_writer: MessageWriter<SelectCharMessage>,
    mut select_char_pic_writer: MessageWriter<SelectCharWithPicMessage>,
    mut register_pic_writer: MessageWriter<RegisterPicMessage>,
) {
    for msg in raw.read() {
        let client_id: i32 = msg.client_id;
        let mut packet: Packet = msg.packet;
        match packet.opcode() {
            x if x == RecvOpcode::CreateChar => {
                let msg: CreateCharMessage =
                    create_char::read_create_character_packet(&packet, client_id);
                create_char_writer.write(msg);
            }
            x if x == RecvOpcode::CheckCharName => {
                let msg: CheckCharNameMessage =
                    check_char_name::read_check_char_name_packet(&packet, client_id);
                check_char_name_writer.write(msg);
            }
            x if x == RecvOpcode::DeleteChar => {
                let msg: DeleteCharMessage =
                    delete_char::read_delete_char_packet(&packet, client_id);
                delete_char_writer.write(msg);
            }
            x if x == RecvOpcode::CharSelect => {
                let msg: SelectCharMessage =
                    select_char::read_select_char_packet(&packet, client_id);
                select_char_writer.write(msg);
            }
            x if x == RecvOpcode::CharSelectWithPic => {
                let msg: SelectCharWithPicMessage =
                    select_char_with_pic::read_select_char_with_pic_packet(&packet, client_id);
                select_char_pic_writer.write(msg);
            }
            x if x == RecvOpcode::RegisterPic => {
                let msg: RegisterPicMessage =
                    register_pic::read_register_pic_packet(&packet, client_id);
                register_pic_writer.write(msg);
            }
            _ => {}
        }
    }
}

pub fn start_playing_router_system(
    mut raw: MessageReader<RawPacketMessage>,
    mut login_started_writer: MessageWriter<LoginStartMessage>,
    mut player_logged_in_writer: MessageWriter<PlayerLoggedInMessage>,
) {
    for msg in raw.read() {
        let client_id: i32 = msg.client_id;
        let mut packet: Packet = msg.packet;
        match packet.opcode() {
            x if x == RecvOpcode::LoginStarted => {
                let msg: LoginStartMessage = LoginStartMessage { client_id };
                login_started_writer.write(msg);
            }
            x if x == RecvOpcode::PlayerLoggedIn => {
                let msg: PlayerLoggedInMessage =
                    player_logged_in::read_player_logged_in_packet(&packet, client_id);
                player_logged_in_writer.write(msg);
            }
            _ => {}
        }
    }
}

pub fn ui_router_system(
    mut raw: MessageReader<RawPacketMessage>,
    mut change_keymap_writer: MessageWriter<ChangeKeymapMessage>,
    mut party_search_writer: MessageWriter<PartySearchMessage>,
) {
    for msg in raw.read() {
        let client_id: i32 = msg.client_id;
        let mut packet: Packet = msg.packet;
        match packet.opcode() {
            x if x == RecvOpcode::ChangeKeymap => {
                let msg: ChangeKeymapMessage =
                    change_keymap::read_change_keymap_packet(&packet, client_id);
                change_keymap_writer.write(msg);
            }
            x if x == RecvOpcode::PartySearch => {
                let msg: PartySearchMessage = PartySearchMessage { client_id };
                party_search_writer.write(msg);
            }
            _ => {}
        }
    }
}

pub fn map_router_system(
    mut raw: MessageReader<RawPacketMessage>,
    mut change_map_writer: MessageWriter<ChangeMapMessage>,
    mut enter_cash_shop_writer: MessageWriter<EnterCashShopMessage>,
    mut player_map_transfer_writer: MessageWriter<PlayerMapTransferMessage>,
) {
    for msg in raw.read() {
        let client_id: i32 = msg.client_id;
        let mut packet: Packet = msg.packet;
        match packet.opcode() {
            x if x == RecvOpcode::ChangeMap => {
                let msg: ChangeMapMessage = change_map::read_change_map_packet(&packet, client_id);
                change_map_writer.write(msg);
            }
            x if x == RecvOpcode::EnterCashShop => {
                let msg: EnterCashShopMessage = EnterCashShopMessage { client_id };
                enter_cash_shop_writer.write(msg);
            }
            x if x == RecvOpcode::PlayerMapTranefer => {
                let msg: PlayerMapTransferMessage = PlayerMapTransferMessage { client_id };
                player_map_transfer_writer.write(msg);
            }
            _ => {}
        }
    }
}

pub fn channel_router_system(
    mut raw: MessageReader<RawPacketMessage>,
    mut change_channel_writer: MessageWriter<ChangeChannelMessage>,
) {
    for msg in raw.read() {
        let client_id: i32 = msg.client_id;
        let mut packet: Packet = msg.packet;
        match packet.opcode() {
            x if x == RecvOpcode::ChangeChannel => {
                let msg: ChangeChannelMessage = cc::read_change_channel_packet(&packet, client_id);
                change_channel_writer.write(msg);
            }
            _ => {}
        }
    }
}

pub fn char_router_system(
    mut raw: MessageReader<RawPacketMessage>,
    mut all_chat_writer: MessageWriter<ChatTextMessage>,
) {
    for msg in raw.read() {
        let mut packet: Packet = msg.packet;
        match packet.opcode() {
            x if x == RecvOpcode::AllChat => {
                let msg: ChatTextMessage = chat_text::read_chat_text_packet(&packet, client_id);
                all_chat_writer.write(msg);
            }
            _ => {}
        }
    }
}

pub fn item_router_system(
    mut raw: MessageReader<RawPacketMessage>,
    mut pickup_item_writer: MessageWriter<PickupItemMessage>,
) {
    for msg in raw.read() {
        let mut packet: Packet = msg.packet;
        match packet.opcode() {
            x if x == RecvOpcode::PickupItem => {
                let msg: PickupItemMessage =
                    pickup_item::read_pickup_item_packet(&packet, client_id);
                pickup_item_writer.write(msg);
            }
            _ => {}
        }
    }
}

pub fn move_router_system(
    mut raw: MessageReader<RawPacketMessage>,
    mut player_attacked_writer: MessageWriter<CloseAttackMessage>,
    mut player_moved_writer: MessageWriter<PlayerMovedMessage>,
    mut player_took_damage_writer: MessageWriter<TakeDamageMessage>,
    mut mob_moved_writer: MessageWriter<MobMovedMessage>,
) {
    for msg in raw.read() {
        let mut packet: Packet = msg.packet;
        match packet.opcode() {
            x if x == RecvOpcode::PlayerMoved => {
                let msg: PlayerMovedMessage =
                    player_moved::read_move_player_packet(&packet, client_id);
                player_moved_writer.write(msg);
            }
            x if x == RecvOpcode::MobMoved => {
                let msg: MobMovedMessage = mob_moved::read_mob_ai_packet(&packet, client_id);
                mob_moved_writer.write(msg);
            }
            x if x == RecvOpcode::CloseAttack => {
                let msg: CloseAttackMessage =
                    attack_close::read_close_attack_packet(&packet, client_id);
                player_attacked_writer.write(msg);
            }
            x if x == RecvOpcode::TakeDamage => {
                let msg: TakeDamageMessage =
                    take_damage::read_take_damage_packet(&packet, client_id);
                take_damage_writer.write(msg);
            }
            _ => {}
        }
    }
}
