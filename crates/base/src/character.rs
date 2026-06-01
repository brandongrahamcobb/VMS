/* base/src/character.rs
 * The purpose of this module is to provide constants for base structs.
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

#[derive(Clone)]
pub enum StatsUpdate {
    Exp { exp: i32 },
    Str { strength: i16 },
    Dex { dexterity: i16 },
    Luk { luck: i16 },
    Int { intelligence: i16 },
    Health { hp: i16 },
    Mana { mp: i16 },
    MaxHealth { max_hp: i16 },
    MaxMana { max_mp: i16 },
    AbilityPoints { ap: i16 },
    SkillPoints { sp: i16 },
}
