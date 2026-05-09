use crate::models::character::model::Character;
use crate::models::shroom::channel::model::Channel;
use crate::net::error::NetworkError;
use crate::net::packet::handler::player_logged_in::reader::PlayerLoggedInReader;
use crate::runtime::session::model::Session;
use crate::runtime::state::SharedState;

#[derive(Clone)]
pub struct PlayerLoggedInStore {
    pub channel: Channel,
    pub char: Character,
    pub sessions: Vec<Session>,
}

impl PlayerLoggedInStore {
    pub async fn store_player_logged_in(
        state: &SharedState,
        session: Session,
        reader: PlayerLoggedInReader,
    ) -> Result<Self, NetworkError> {
        let char: Character = session.get_char()?;
        let channel: Channel = session.get_channel()?;
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
            sessions,
        })
    }
}
