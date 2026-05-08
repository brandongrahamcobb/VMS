use crate::models::character::skill;
use crate::models::character::skill::model::NewSkillInsert;
use crate::models::character::skill::model::Skill;
use crate::models::character::skill::model::SkillModel;
use crate::models::error::ModelError;
use crate::models::wz;
use crate::runtime::state::SharedState;

impl Skill {
    pub fn new(model: SkillModel) -> Self {
        Self { model }
    }
}

impl NewSkillInsert {
    pub fn default(char_id: i32, wz_id: i32) -> Self {
        Self { char_id, wz_id }
    }
}

pub async fn get_skills_by_character_id(
    state: &SharedState,
    char_id: i32,
) -> Result<Vec<Skill>, ModelError> {
    let skill_models: Vec<SkillModel> =
        skill::query::get_skill_models_by_character_id(state, char_id).await?;
    let mut skills: Vec<Skill> = Vec::<Skill>::new();
    for skill_model in skill_models {
        skills.push(Skill { model: skill_model });
    }
    Ok(skills)
}

pub async fn create_skills_for_new_character(
    state: &SharedState,
    char_id: i32,
    job_id: i16,
) -> Result<Vec<SkillModel>, ModelError> {
    let filename = String::from("Skill.wz");
    let map = wz::service::get_img_map(job_id as i32, &filename)?;
    let skills: Vec<SkillModel> =
        skill::query::create_skills_by_character_id_and_job_id(state, char_id, map).await?;
    Ok(skills)
}

pub async fn get_skill_by_character_id_and_skill_id(
    state: &SharedState,
    char_id: i32,
    skill_id: i32,
) -> Result<Skill, ModelError> {
    let skill_model =
        skill::query::get_skill_model_by_character_id_and_skill_id(state, char_id, skill_id)
            .await?;
    let skill = Skill { model: skill_model };
    Ok(skill)
}
