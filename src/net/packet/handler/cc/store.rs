use crate::config::settings;
use crate::inc::helpers;
use crate::models::character::model::Character;
use crate::models::shroom::channel;
use crate::models::shroom::channel::model::Channel;
use crate::net::error::NetworkError;
use crate::net::packet::handler::cc::reader::ChangeChannelReader;
use crate::runtime::session::model::Session;
use crate::runtime::state::SharedState;

#[derive(Clone)]
pub struct ChangeChannelStore {
    pub char: Character,
    pub channel: Channel,
    pub octets: [u8; 4],
    pub sessions: Vec<Session>,
}

impl ChangeChannelStore {
    pub async fn store_change_channel(
        state: &SharedState,
        session: Session,
        reader: ChangeChannelReader,
    ) -> Result<Self, NetworkError> {
        let channel = channel::service::get_channel_by_id(state, reader.channel_id).await?;
        let addr = settings::get_routing_address()?;
        let octets = helpers::convert_to_ip_array(addr);
        let char = session.get_char()?;
        let mut sessions: Vec<Session> = Vec::<Session>::new();
        {
            let state = state.lock().await;
            for s in state.sessions.get_by_map_channel_world(
                session.get_map()?.model.wz_id,
                channel.model.id,
                session.get_world()?.model.id,
                session.id,
            ) {
                sessions.push(s);
            }
        }
        Ok(Self {
            char,
            channel,
            octets,
            sessions,
        })
    }
}
