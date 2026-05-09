use crate::models::shroom::channel::model::Channel;

#[derive(Clone)]
pub struct WorldModel {
    pub id: i16,
    pub name: &'static str,
    pub flag: i16,
    pub event_message: &'static str,
    pub port: i16,
}

#[derive(Clone)]
pub struct World {
    pub model: WorldModel,
    pub channels: Vec<Channel>,
}
