/* app/src/component/skill.rs
 * The purpose of this module is to provide a skill component.
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

use core::convert::From;

use base::skill::BaseSkill;
use bevy::ecs::component::Component;
use db::skill::model::SkillModel;

#[derive(Component)]
pub struct MapleSkill {
    pub id: i32,
    pub char_id: i32,
    pub level: i16,
    pub base: BaseSkill,
}

impl From<(BaseSkill, SkillModel)> for MapleSkill {
    fn from((base, model): (BaseSkill, SkillModel)) -> Self {
        let id = if let Some(id) = model.id { id } else { 0 };
        Self {
            id,
            char_id: model.char_id,
            level: model.level,
            base,
        }
    }
}
