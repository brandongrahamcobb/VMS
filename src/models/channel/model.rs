#[derive(Clone, Debug)]
pub struct Channel {
    pub world_id: i16,
    pub channel_id: i16,
    pub name: String,
    pub capacity: i16,
    pub port: i16,
}
