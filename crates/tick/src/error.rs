/* tick/error.rs
 * The purpose of this module is to provide errors related to ticks.
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

use net::packet::build::error::PacketBuildError;
use thiserror::Error;

use crate::mob_respawn::error::MobRespawnError;

#[derive(Debug, Error)]
pub enum TickError {
    #[error("Packet build error in tick layer")]
    PacketBuildError(#[from] PacketBuildError),

    #[error("Mob respawn error in tick layer")]
    MobRespawnError(#[from] MobRespawnError),
}
