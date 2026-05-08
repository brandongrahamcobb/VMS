use std::time::SystemTime;

#[derive(Clone, Debug)]
pub struct ChannelModel {
    pub id: i16,
    pub capacity: Option<i16>,
    pub flag: Option<i16>,
    pub port: i16,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

#[derive(Clone)]
pub struct Channel {
    pub model: ChannelModel,
}

pub struct NewChannelInsert {
    pub id: i16,
    pub port: i16,
}
