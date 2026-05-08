use crate::db::schema::characters;
use crate::models::character::equipment_set::android::model::AndroidEquipmentSet;
use crate::models::character::equipment_set::cash::model::CashEquipmentSet;
use crate::models::character::equipment_set::pet::model::PetEquipmentSet;
use crate::models::character::equipment_set::regular::model::RegularEquipmentSet;
use crate::models::character::keybinding::model::Keybinding;
use crate::models::character::skill::model::Skill;
use crate::models::map::model::Map;
use crate::models::world::model::World;
use diesel::prelude::*;
use std::time::SystemTime;

#[derive(Clone, Identifiable, Queryable, AsChangeset, Selectable)]
#[diesel(table_name = characters)]
pub struct CharacterModel {
    pub id: i32,
    pub acc_id: i32,
    pub world_id: i16,
    pub ign: String,
    pub level: i16,
    pub exp: i32,
    pub strength: i16,
    pub dexterity: i16,
    pub luck: i16,
    pub intelligence: i16,
    pub hp: i16,
    pub mp: i16,
    pub max_hp: i16,
    pub max_mp: i16,
    pub ap: i16,
    pub fame: i16,
    pub meso: i32,
    pub job_id: i16,
    pub face_id: i32,
    pub hair_id: i32,
    pub hair_color_id: i32,
    pub skin_id: i32,
    pub gender_id: i16,
    pub map_id: i32,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

#[derive(Insertable)]
#[diesel(table_name = characters)]
pub struct NewInsert {
    pub acc_id: i32,
    pub world_id: i16,
    pub ign: String,
    pub job_id: i16,
    pub face_id: i32,
    pub hair_id: i32,
    pub hair_color_id: i32,
    pub skin_id: i32,
    pub gender_id: i16,
    pub map_id: i32,
}

#[derive(Clone)]
pub struct Character {
    pub model: CharacterModel,
    pub regular_equips: RegularEquipmentSet,
    pub cash_equips: CashEquipmentSet,
    pub pet_equips: PetEquipmentSet,
    pub android_equips: AndroidEquipmentSet,
    pub skills: Vec<Skill>,
    pub binds: Vec<Keybinding>,
    pub world: World,
    pub map: Map,
}
