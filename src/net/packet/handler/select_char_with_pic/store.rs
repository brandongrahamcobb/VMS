use crate::config::settings;
use crate::inc::helpers;
use crate::models::account::model::AccountModel;
use crate::models::channel::model::ChannelModel;
use crate::models::character;
use crate::models::character::model::Character;
use crate::net::error::NetworkError;
use crate::net::packet::handler::select_char_with_pic::reader::SelectCharWithPicReader;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;

#[derive(Clone)]
pub struct SelectCharWithPicStore {
    pub char: Character,
    pub channel_model: ChannelModel,
    pub octets: [u8; 4],
    pub pic_status: bool,
}

impl SelectCharWithPicStore {
    pub async fn store_select_char_with_pic(
        state: &SharedState,
        session: Session,
        reader: SelectCharWithPicReader,
    ) -> Result<Self, NetworkError> {
        let acc_model: AccountModel = session.acc.model.clone();
        let channel_model: ChannelModel = session.channel.model.clone();
        let char: Character =
            character::service::get_character_by_id(state, reader.char_id).await?;
        let acc_pic = acc_model.pic.clone();
        let mut pic_status = false;
        if acc_pic == reader.pic {
            pic_status = true;
        }
        let addr: String = settings::get_address()?;
        let octets: [u8; 4] = helpers::convert_to_ip_array(addr.clone());
        Ok(Self {
            channel_model,
            char,
            pic_status,
            octets,
        })
    }
}
