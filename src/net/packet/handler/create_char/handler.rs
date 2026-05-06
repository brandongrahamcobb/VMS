use crate::constants::{DEFAULT_ACTION, DEFAULT_KEY, DEFAULT_TYPE};
use crate::models::character::equipment_set::model::{
    NewAndroidEquipmentSet, NewCashEquipmentSet, NewPetEquipmentSet, NewRegularEquipmentSet,
};
use crate::models::character::keybinding::model::NewKeybinding;
use crate::models::character::model::{Character, NewCharacter};
use crate::models::character::{equipment_set, keybinding};
use crate::models::{character, wz};
use crate::net::action::Action;
use crate::net::error::NetworkError;
use crate::net::packet::handler::create_char;
use crate::net::packet::handler::create_char::reader::CreateCharacterReader;
use crate::net::packet::handler::create_char::store::CreateCharStore;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::model::Packet;
use crate::runtime::error::SessionError;
use crate::runtime::scope::Scope;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;
use itertools::izip;

pub struct CreateCharHandler;

impl CreateCharHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        &self,
        state: &SharedState,
        session: &Session,
        packet: &Packet,
    ) -> Result<HandlerResult, NetworkError> {
        let reader: CreateCharReader =
            CreateCharReader::new().read_create_character_packet(packet)?;
        let store: CreateCharStore = CreateCharStore::new()
            .store_create_char(state, session, &reader)
            .await?;
        let result: HandlerResult = self.build_create_char_result(&store)?;
        Ok(result)
    }

    fn build_create_char_result(
        &self,
        store: &CreateCharStore,
    ) -> Result<HandlerResult, NetworkError> {
        let mut result: HandlerResult = HandlerResult::new();
        let packet: Packet = Packet::new_empty()
            .build_create_char_handler_packet(
                &store.char,
                &store.regular_equips,
                &store.cash_equips,
                &store.android_equips,
                &store.pet_equips,
            )
            .finish();
        result.add_action(Action::Send {
            packet: packet.clone(),
            scope: Scope::Local,
        })?;
        Ok(result)
    }
}
