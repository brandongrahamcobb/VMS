use crate::db::models::character::core::NewCharacter;
use crate::net::error::NetworkError;
use crate::net::packet::core::Packet;
use crate::net::packet::error::PacketError;
use crate::net::packet::handler::action::login::LoginAction;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::io::error::IOError::ReadError;
use crate::prelude::*;
use crate::runtime::error::SessionError;
use crate::runtime::relay::RuntimeContext;
use std::io::BufReader;
pub struct CreateCharacterHandler;

impl CreateCharacterHandler {
    pub fn new() -> Self {
        Self
    }
}

impl CreateCharacterHandler {
    pub async fn handle(
        self: &Self,
        ctx: &RuntimeContext,
        packet: &Packet,
    ) -> Result<HandlerResult<LoginAction>, NetworkError> {
        let mut reader = BufReader::new(&**packet);
        reader
            .read_short()
            .map_err(ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        let session = ctx
            .shared_state
            .sessions
            .get(ctx.session_id as u32)
            .ok_or(SessionError::NotFound(ctx.session_id))
            .map_err(NetworkError::from)?;
        let account_id = session
            .account_id
            .ok_or(SessionError::NoAccount)
            .map_err(NetworkError::from)?;
        let ign = &reader
            .read_str_with_length()
            .map_err(ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        let job = reader
            .read_int()
            .map_err(ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)? as i16;
        let face = reader
            .read_int()
            .map_err(ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        let hair = reader
            .read_int()
            .map_err(ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        let hair_color = reader
            .read_int()
            .map_err(ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        let skin = reader
            .read_int()
            .map_err(ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        let _top = reader
            .read_int() // Slot 5
            .map_err(ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        let _bot = reader
            .read_int() // Slot 6
            .map_err(ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        let _shoes = reader
            .read_int() // Slot 7
            .map_err(ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        let _weapon = reader
            .read_int() // Special
            .map_err(ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        let gender = reader
            .read_byte()
            .map_err(ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)? as i16;
        let world_id = session
            .selected_world_id
            .ok_or(SessionError::NoAccount)
            .map_err(NetworkError::from)?;
        let character = NewCharacter {
            account: account_id,
            world: world_id,
            ign: ign.clone(),
            level: None,
            exp: None,
            strength: None,
            dexterity: None,
            luck: None,
            intelligence: None,
            hp: None,
            mp: None,
            max_hp: None,
            max_mp: None,
            ap: None,
            fame: None,
            meso: None,
            job: job,
            face: face,
            hair: hair,
            hair_color: hair_color,
            skin: skin,
            gender: gender,
            created_at: None,
            map: None,
            updated_at: None,
        };
        let mut result = HandlerResult::new();
        let action = LoginAction::CreateCharacter { character };
        result.add_action(action)?;
        Ok(result)
    }
}
