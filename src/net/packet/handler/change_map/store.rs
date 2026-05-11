use crate::models::character;
use crate::models::character::model::{Character, CharacterModel};
use crate::models::shroom::channel::model::Channel;
use crate::models::shroom::map;
use crate::models::shroom::map::model::{Map, Portal};
use crate::net::error::NetworkError;
use crate::net::packet::handler::change_map::reader::ChangeMapReader;
use crate::runtime::session::model::Session;
use crate::runtime::state::SharedState;

#[derive(Clone)]
pub struct ChangeMapStore {
    pub channel: Channel,
    pub char: Character,
    pub died: i16,
    pub map: Map,
    pub portal: Portal,
    pub sessions: Vec<Session>,
    pub wheel_of_destiny: i16,
}

impl ChangeMapStore {
    pub async fn store_change_map(
        state: &SharedState,
        session: Session,
        reader: ChangeMapReader,
    ) -> Result<Self, NetworkError> {
        let mut char: Character = session.get_char()?;
        let channel: Channel = session.get_channel()?;
        let map: Map = session.get_map()?;
        let portal: Portal = map.get_portal(reader.tn)?;
        let map: Map = map::service::get_map_by_id(portal.model.tm)?;
        let mut char_model: CharacterModel = char.model.clone();
        char_model.map_id = map.model.wz_id;
        char.update_model(char_model.clone());
        let char: Character = char_model.load(state).await?;
        character::query::setters::update_characters(state, vec![char_model]).await?;
        let died: i16 = reader.died;
        let wheel_of_destiny: i16 = reader.wod;
        let mut sessions: Vec<Session> = Vec::<Session>::new();
        {
            let state = state.lock().await;
            for s in state.sessions.get_by_map_channel_world(
                char.map.model.wz_id,
                channel.model.id,
                session.get_world()?.model.id,
                session.id,
            ) {
                sessions.push(s);
            }
        }
        Ok(Self {
            channel,
            char,
            died,
            map,
            portal,
            sessions,
            wheel_of_destiny,
        })
    }
}
