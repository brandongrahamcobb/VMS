#[derive(Clone, Debug)]
pub struct ChannelModel {
    pub id: i16,
    pub capacity: i16,
    pub flag: i16,
    pub port: i16,
}

#[derive(Clone)]
pub struct Channel {
    pub model: ChannelModel,
}
