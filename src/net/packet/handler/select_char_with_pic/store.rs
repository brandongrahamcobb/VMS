use crate::config::settings;
use crate::inc::helpers;
use crate::models::account::model::Account;
use crate::models::character;
use crate::models::character::model::Character;
use crate::models::shroom::channel::model::Channel;
use crate::net::error::NetworkError;
use crate::net::packet::handler::select_char_with_pic::reader::SelectCharWithPicReader;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;

#[derive(Clone)]
pub struct SelectCharWithPicStore {
    pub char: Character,
    pub channel: Channel,
    pub octets: [u8; 4],
    pub pic_status: bool,
}

impl SelectCharWithPicStore {
    pub async fn store_select_char_with_pic(
        state: &SharedState,
        session: Session,
        reader: SelectCharWithPicReader,
    ) -> Result<Self, NetworkError> {
        let acc: Account = session.get_acc()?;
        let channel: Channel = session.get_channel()?;
        let char: Character =
            character::service::get_character_by_id(state, reader.char_id).await?;
        let acc_pic = acc.model.get_pic()?;
        let mut pic_status = false;
        if acc_pic == reader.pic {
            pic_status = true;
        }
        let addr: String = settings::get_routing_address()?;
        let octets: [u8; 4] = helpers::convert_to_ip_array(addr.clone());
        Ok(Self {
            channel,
            char,
            pic_status,
            octets,
        })
    }
}
