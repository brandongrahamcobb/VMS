/* app/src/model.rs
 * The purpose of this module is to provide actions to take after processing a system message.
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

use crate::scope::{ActionScope, TickScope};
use net::packet::model::Packet;

#[derive(Clone, Debug)]
pub enum Action {
    HandlerAction { packet: Packet, scope: ActionScope },
    TickAction { packet: Packet, scope: TickScope },
}
