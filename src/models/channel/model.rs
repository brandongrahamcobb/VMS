#[derive(Clone, Debug)]
pub struct Channel {
    pub id: i16,
    pub capacity: i16,
    pub flag: i8,
    pub port: i16,
    pub world_id: i16,
}
