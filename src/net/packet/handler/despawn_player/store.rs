use crate::models::channel::model::ChannelModel;
use crate::models::character::model::CharacterModel;
use crate::models::map::model::MapModel;
use crate::models::world::model::WorldModel;
use crate::net::error::NetworkError;
use crate::net::packet::handler::despawn_player::reader::DespawnPlayerReader;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;

#[derive(Clone)]
pub struct DespawnPlayerStore {
    pub char: CharacterModel,
    pub world: WorldModel,
    pub channel: ChannelModel,
    pub map: MapModel,
}

impl DespawnPlayerStore {
    pub async fn store_despawn_player(
        state: &SharedState,
        session: Session,
        reader: DespawnPlayerReader,
    ) -> Result<Self, NetworkError> {
        std::hint::black_box(state);
        std::hint::black_box(reader.clone());
        let world = session.world.clone();
        let channel: ChannelModel = session.channel.clone();
        let map = session.map.clone();
        let char = session.char.clone();
        Ok(Self {
            char,
            world,
            channel,
            map,
        })
    }
}
