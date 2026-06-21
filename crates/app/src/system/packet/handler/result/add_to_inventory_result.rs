/* app/src/system/handler/result/pickup_item_result.rs
 * The purpose of this module is to write the pickup item packet result.
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
use action::scope::ActionScope;
use base::inventory::InventoryMod;
use bevy::ecs::message::MessageWriter;

use crate::component::item::MapleItem;
use crate::message::result::HandlerResult;
use crate::system::packet::build::codec;

pub fn write_result(
    client_id: i32,
    inv_mod: InventoryMod,
    item: &MapleItem,
    results: &mut MessageWriter<HandlerResult>,
) -> () {
    let mut actions: Vec<Action> = Vec::new();
    let Ok(mut add_to_inventory_packet) =
        codec::item::builder::build_add_to_inventory_packet(inv_mod, item)
    else {
        return;
    };
    actions.push(Action::HandlerAction {
        packet: add_to_inventory_packet.finish(),
        scope: ActionScope::Local,
    });
    results.write(HandlerResult { client_id, actions });
}
