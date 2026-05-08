#[derive(Clone, Debug)]
pub struct ChannelModel {
    pub id: i8,
    pub capacity: i16,
    pub flag: i8,
    pub port: u16,
}

#[derive(Clone)]
pub struct Channel {
    pub model: ChannelModel,
}
