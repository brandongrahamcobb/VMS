use crate::models::account::model::Account;
use crate::models::channel::model::Channel;
use crate::models::character::equipment_set::model::{
    AndroidEquipmentSet, CashEquipmentSet, PetEquipmentSet, RegularEquipmentSet,
};
use crate::models::character::keybinding::model::Keybinding;
use crate::models::character::model::Character;
use crate::models::character::{equipment_set, keybinding};
use crate::models::map::model::Map;
use crate::models::world::model::World;
use crate::models::{account, channel, map, world};
use crate::net::error::NetworkError;
use crate::net::packet::handler::player_logged_in::reader::PlayerLoggedInReader;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;

pub struct PlayerLoggedInStore {
    pub acc: Account,
    pub char: Character,
    pub world: World,
    pub channel: Channel,
    pub map: Map,
    pub regular_equips: RegularEquipmentSet,
    pub cash_equips: CashEquipmentSet,
    pub android_equips: AndroidEquipmentSet,
    pub pet_equips: PetEquipmentSet,
    pub binds: Vec<Keybinding>,
}

impl PlayerLoggedInStore {
    pub fn new() -> Self {
        Self
    }

    pub async fn store_player_logged_in(
        &self,
        state: &SharedState,
        session: &Session,
        reader: &PlayerLoggedInReader,
    ) -> Result<Self, NetworkError> {
        let char: Character = character::query::get_character_by_id(state, &read.char_id).await?;
        let world_id: i8 = session.world_id.ok_or(SessionError::NoWorld(session.id))?;
        let world = world::service::get_world_by_id(&world_id)?;
        let channel_id: i8 = session
            .channel_id
            .ok_or(SessionError::NoChannel(session.id))?;
        let channel: Channel = channel::service::get_channel_by_ids(&channel_id, &world_id)?;
        let map_id: i8 = session.map_id.ok_or(SessionError::NoMap(session.id))?;
        let map: Map = map::service::get_map_by_id(&map_id)?;
        let regular_equips: RegularEquipmentSet =
            equipment_set::query::get_regular_equipment_set_by_character_id(state, &char_id)?;
        let cash_equips: CashEquipmentSet =
            equipment_set::query::get_cash_equipment_set_by_character_id(state, &char_id)?;
        let android_equips: AndroidEquipmentSet =
            equipment_set::query::get_android_equipment_set_by_character_id(state, &char_id)?;
        let pet_equips: PetEquipmentSet =
            equipment_set::query::get_pet_equipment_set_by_character_id(state, &char_id)?;
        let binds: Vec<Keybinding> =
            keybinding::query::get_keybindings_by_character_id(state, &char_id)?;
        Ok(Self {
            acc: acc.clone(),
            char: char.clone(),
            world: world.clone(),
            channel: channel.clone(),
            map: map.clone(),
            regular_equips: regular_equips.clone(),
            cash_equips: cash_equips.clone(),
            android_equips: android_equips.clone(),
            pet_equips: pet_equips.clone(),
            binds: binds.clone(),
        })
    }
}
