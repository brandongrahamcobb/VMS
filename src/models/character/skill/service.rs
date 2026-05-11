use crate::models::character::skill::model::Skill;
use crate::models::character::skill::model::SkillModel;
use crate::models::error::ModelError;
use crate::wz;

impl SkillModel {
    pub fn load(&self) -> Result<Skill, ModelError> {
        Ok(Skill {
            model: self.clone(),
        })
    }
}

pub fn generate_skill_ids_by_job_id(wz_job_id: i32) -> Result<Vec<i32>, ModelError> {
    let root = wz::service::get_img_root(wz_job_id, "Skill.wz")?;
    let mut wz_ids: Vec<i32> = root
        .get("skill")
        .and_then(|s| s.as_object())
        .unwrap_or(&serde_json::Map::new())
        .keys()
        .filter_map(|k| k.parse::<i32>().ok())
        .collect();
    let basic_attack_skill_wz_id: i32 = 256;
    wz_ids.push(basic_attack_skill_wz_id);
    Ok(wz_ids)
}
