use crate::models::channel::model::Channel;
use crate::models::character::equipment_set;
use crate::models::character::equipment_set::model::{
    AndroidEquipmentSet, CashEquipmentSet, PetEquipmentSet, RegularEquipmentSet,
};
use crate::models::character::model::Character;
use crate::models::map::model::Map;
use crate::models::world::model::World;
use crate::net::error::NetworkError;
use crate::net::packet::handler::cc::read::ChangeChannelRead;
use crate::runtime::error::SessionError;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;

pub struct ChangeChannelStore {
    pub char: Character,
    pub world: World,
    pub source_channel: Channel,
    pub target_channel: Channel,
    pub map: Map,
    pub regular_equips: RegularEquipmentSet,
    pub cash_equips: CashEquipmentSet,
    pub android_equips: AndroidEquipmentSet,
    pub pet_equips: PetEquipmentSet,
}

impl ChangeChannelStore {
    pub fn new() -> Self {
        Self
    }

    pub async fn store_change_channel(
        &self,
        state: &SharedState,
        session: &Session,
        read: &ChangeChannelRead,
    ) -> Result<Self, NetworkError> {
        let world_id = session
            .world_id
            .ok_or(SessionError::NoWorldSelected(*session.id))?;
        let world = world::service::get_world_by_id(&world_id)?;
        let source_channel = session.channel.ok_or(SessionError::NoChannel(*session.id));
        let target_channel = channel::service::get_channel_by_ids(&read.channel_id, &world_id)?;
        let map_id = session
            .map_id
            .ok_or(SessionError::NoMapSelected(*session.id))?;
        let map = map::service::get_map_by_id(&map_id)?;
        let char_id = session.char_id.ok_or(SessionError::NoChar(session.id))?;
        let char = character::query::get_character_by_id(state, &char_id).await?;
        let regular_equips =
            equipment_set::query::get_regular_equipment_set_by_character_id(state, &char_id)?;
        let cash_equips =
            equipment_set::query::get_cash_equipment_set_by_character_id(state, &char_id)?;
        let android_equips =
            equipment_set::query::get_android_equipment_set_by_character_id(state, &char_id)?;
        let pet_equips =
            equipment_set::query::get_pet_equipment_set_by_character_id(state, &char_id)?;
        Ok(Self {
            char: char.clone(),
            world: world.clone(),
            source_channel: source_channel.clone(),
            target_channel: target_channel.clone(),
            map: map.clone(),
            regular_equips: regular_equips.clone(),
            cash_equips: cash_equips.clone(),
            android_equips: android_equips.clone(),
            pet_equips: pet_equips.clone(),
        })
    }
}
