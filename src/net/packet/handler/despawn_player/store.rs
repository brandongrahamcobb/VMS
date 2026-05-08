use crate::models::channel::model::Channel;
use crate::models::character::model::CharacterModel;
use crate::models::map::model::Map;
use crate::models::world::model::World;
use crate::net::error::NetworkError;
use crate::net::packet::handler::despawn_player::reader::DespawnPlayerReader;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;

#[derive(Clone)]
pub struct DespawnPlayerStore {
    pub char_model: CharacterModel,
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
        let world: World = session.world.clone();
        let channel: Channel = session.channel.clone();
        let map: Map = session.map.clone();
        let char_model: CharacterModel = session.char.model.clone();
        Ok(Self {
            char_model,
            world,
            channel,
            map,
        })
    }
}
