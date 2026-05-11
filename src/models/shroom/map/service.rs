use crate::models::error::ModelError;
use crate::models::shroom::map::model::{Map, MapModel, Portal, PortalModel};
use crate::wz;
use crate::wz::error::WzError;

pub fn get_map_by_id(wz_map_id: i32) -> Result<Map, ModelError> {
    let root = wz::service::get_img_root(wz_map_id, "Map.wz")?;
    let wz_portals = root.get("portal").and_then(|p| p.as_object()).unwrap();
    let mut portal_models: Vec<PortalModel> = Vec::<PortalModel>::new();
    for (key, target) in wz_portals {
        let pid = key.parse::<i16>().unwrap_or(0);
        let pn = target
            .get("pn")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let tm = target
            .get("tm")
            .and_then(|v| v.as_i64())
            .map(|v| v as i32)
            .ok_or(WzError::ObjectError)?;
        let tn = target
            .get("tn")
            .and_then(|v| v.as_str())
            .unwrap_or("sp")
            .to_string();
        portal_models.push(PortalModel { pid, pn, tm, tn });
    }
    let mut portals: Vec<Portal> = Vec::<Portal>::new();
    for p_model in portal_models {
        portals.push(Portal { model: p_model });
    }
    Ok(Map {
        model: MapModel { wz_id: wz_map_id },
        portals,
    })
}

pub fn get_map_by_job_id(job_id: i16) -> Result<Map, ModelError> {
    match job_id {
        1 => {
            let map_id = 10000;
            get_map_by_id(map_id)
        }
        1000 => {
            let map_id = 130000000;
            get_map_by_id(map_id)
        }
        2000 => {
            let map_id = 140000000;
            get_map_by_id(map_id)
        }
        _ => Err(ModelError::MapError),
    }
}

impl Map {
    pub fn get_portal(&self, tn: String) -> Result<Portal, ModelError> {
        let portal: Portal = self
            .portals
            .iter()
            .find(|p| p.model.pn == tn)
            .cloned()
            .ok_or(WzError::ObjectError)?;
        Ok(portal)
    }
}

impl PortalModel {
    pub fn load(&self) -> Result<Portal, ModelError> {
        Ok(Portal {
            model: self.clone(),
        })
    }
}
