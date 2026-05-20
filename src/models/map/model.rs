/* map/model.rs
 * The purpose of this module is to provide a map model and its methods.
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

use crate::metadata;
use crate::models::map::error::MapError;
use crate::models::map::wrapper::{Map, VacancyState};
use crate::models::mob::wrapper::Mob;
use crate::models::portal::error::PortalError;
use crate::models::portal::model::PortalModel;
use crate::models::portal::wrapper::Portal;
use crate::models::{map, mob};
use crate::net::packet::handler::mob_respawn;
use crate::net::packet::handler::mob_respawn::handler::MobRespawnHandler;
use crate::runtime::state::SharedState;
use std::collections::HashMap;
use tokio::sync::broadcast;

#[derive(Clone)]
pub struct Point {
    pub x: i16,
    pub y: i16,
}

#[derive(Clone)]
pub struct MapWzInfo {
    pub mob_rate: f32,
    pub return_map_wz: i32,
}

#[derive(Clone)]
pub struct MapModel {
    pub wz: i32,
}

impl MapModel {
    pub fn load_portals(&self, map_wz: i32) -> Result<HashMap<u8, Portal>, MapError> {
        let mut portals: HashMap<u8, Portal> = HashMap::new();
        let filename: String = String::from("Map.wz");
        let json = metadata::service::wz_to_img(map_wz, &filename)?;
        let portal_map = json["portal"].as_object().ok_or(PortalError::NoPortal)?;
        for (pid, _) in portal_map.iter() {
            let pid: u8 = pid.parse::<u8>()?;
            let p_model: PortalModel = PortalModel { map_wz };
            portals.insert(pid, p_model.load(map_wz, pid)?);
        }
        Ok(portals)
    }

    pub fn load_mobs(&self, map_wz: i32) -> Result<HashMap<u32, Mob>, MapError> {
        let mut mobs: HashMap<u32, Mob> = HashMap::new();
        let mut next_id: u32 = 1;
        let mob_lifes = mob::service::get_mob_lifes(map_wz)?;
        for mob_life in mob_lifes {
            let mob_wz_life = mob::service::get_mob_wz_life(mob_life.clone())?;
            let mob_wz_info = mob::service::get_mob_wz_info(&mob_wz_life)?;
            let mob = mob::service::init_mob(next_id, &mob_wz_info, &mob_wz_life)?;
            mobs.insert(next_id, mob);
            next_id += 1;
        }
        Ok(mobs)
    }

    pub async fn load(
        self,
        state: &SharedState,
        world_id: i16,
        channel_id: u8,
        map_wz: i32,
    ) -> Result<Map, MapError> {
        let (tick_tx, _) = broadcast::channel(32);
        let wz_info: MapWzInfo = map::service::build_map_wz_info(map_wz)?;
        let mobs: HashMap<u32, Mob> = self.load_mobs(map_wz)?;
        let portals: HashMap<u8, Portal> = self.load_portals(map_wz)?;
        let mob_respawn_handler: MobRespawnHandler = mob_respawn::handler::MobRespawnHandler::new();
        mob_respawn_handler
            .handle(state, tick_tx.clone(), world_id, channel_id, map_wz)
            .await?;
        Ok(Map {
            model: MapModel { wz: map_wz },
            chars: HashMap::new(),
            info: wz_info,
            items: HashMap::new(),
            mobs,
            portals,
            tick_tx,
            vacancy_token: None,
            vacancy: VacancyState::Vacant,
        })
    }
}
