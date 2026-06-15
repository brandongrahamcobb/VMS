/* app/src/system/handler/result/enter_cash_shop_result.rs
 * The purpose of this module is to write the cash shop entrance packet result.
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

use action::model::Action;
use action::scope::{ActionScope, MapScope};
use bevy::ecs::message::MessageWriter;

use crate::component::account::MapleAccount;
use crate::component::character::MapleCharacter;
use crate::component::hp::MapleHealth;
use crate::component::item::MapleItem;
use crate::component::mp::MapleMana;
use crate::message::result::HandlerResult;
use crate::system::packet::build::{codec, enter_cash_shop};

pub fn write_result(
    client_id: i32,
    acc: &MapleAccount,
    char: &MapleCharacter,
    equips: &Vec<MapleItem>,
    hp: &MapleHealth,
    mp: &MapleMana,
    results: &mut MessageWriter<HandlerResult>,
) -> () {
    let mut actions: Vec<Action> = Vec::new();
    let Ok(mut despawn_packet) = codec::player::spawn::build_despawn_player_packet(char.id) else {
        return;
    };
    let Ok(mut enter_cash_shop_packet) =
        enter_cash_shop::build_enter_cash_shop_packet(acc.username.clone(), char, equips, hp, mp)
    else {
        return;
    };
    actions.push(Action::HandlerAction {
        packet: despawn_packet.finish(),
        scope: ActionScope::Map(MapScope::SameChannelSameWorld),
    });
    actions.push(Action::HandlerAction {
        packet: enter_cash_shop_packet.finish(),
        scope: ActionScope::Map(MapScope::SameChannelSameWorld),
    });
    results.write(HandlerResult {
        client_id: client_id,
        actions,
    });
}
