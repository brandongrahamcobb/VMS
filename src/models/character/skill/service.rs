use crate::models::character::model::CharacterModel;
use crate::models::character::skill;
use crate::models::character::skill::model::Skill;
use crate::models::character::skill::model::SkillModel;
use crate::models::error::ModelError;
use crate::runtime::state::SharedState;

impl SkillModel {
    pub fn load(&self) -> Result<Skill, ModelError> {
        Ok(Skill {
            model: self.clone(),
        })
    }
}

pub fn generate_skill_ids_by_job_id(_job_id: i16) -> Result<Vec<i32>, ModelError> {
    let skill_ids: Vec<i32> = Vec::<i32>::new();
    // let filename = String::from("Skill.wz");
    // let map = item::service::get_img_map(job_id, &filename)?;
    // debug!("{?:}", map);
    // !todo!();
    // let skill_ids: Vec<i32> = get_skill_ids_by_job(&map, job_id);
    Ok(skill_ids)
}

pub async fn load_skills_by_character_model(
    state: &SharedState,
    char_model: CharacterModel,
) -> Result<Vec<Skill>, ModelError> {
    let skill_ids = generate_skill_ids_by_job_id(char_model.job_id)?;
    let mut skill_models: Vec<SkillModel> = Vec::<SkillModel>::new();
    for skill_id in skill_ids {
        skill_models.push(
            skill::query::getters::get_skill_model_by_character_id_and_skill_id(
                state,
                char_model.get_id()?,
                skill_id,
            )
            .await?,
        );
    }
    let mut skills: Vec<Skill> = Vec::<Skill>::new();
    for skill_model in skill_models {
        skills.push(skill_model.load()?);
    }
    Ok(skills)
}
