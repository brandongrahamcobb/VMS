use crate::models::character::model::Character;
use crate::models::shroom::channel::model::Channel;
use crate::models::shroom::map::model::Map;
use crate::models::shroom::world::model::World;
use crate::net::error::NetworkError;
use crate::net::packet::handler::despawn_player::reader::DespawnPlayerReader;
use crate::runtime::session::model::Session;
use crate::runtime::state::SharedState;

#[derive(Clone)]
pub struct DespawnPlayerStore {
    pub char: Character,
    pub world: World,
    pub channel: Channel,
    pub map: Map,
}

impl DespawnPlayerStore {
    pub async fn store_despawn_player(
        state: &SharedState,
        session: Session,
        reader: DespawnPlayerReader,
    ) -> Result<Self, NetworkError> {
        std::hint::black_box(state);
        std::hint::black_box(reader.clone());
        let world: World = session.get_world()?;
        let channel: Channel = session.get_channel()?;
        let map: Map = session.get_map()?;
        let char: Character = session.get_char()?;
        Ok(Self {
            char,
            world,
            channel,
            map,
        })
    }
}
