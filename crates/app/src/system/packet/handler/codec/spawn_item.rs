use std::collections::HashMap;

use crate::component::item::MapleItem;
use crate::component::slot::MapleFilledItemSlot;
use bevy::ecs::entity::Entity;
use bevy::ecs::hierarchy::ChildOf;
use bevy::ecs::system::Commands;
use db::item::model::ItemModel;

pub fn spawn_item(
    commands: &mut Commands,
    cid: i32,
    item_map: &HashMap<i32, Vec<ItemModel>>,
    tab_entity: Entity,
) -> Vec<MapleItem> {
    let mut filled_slots: Vec<MapleItem> = Vec::new();
    for (char_id, item_models) in item_map {
        if *char_id == cid {
            for item_model in item_models.clone() {
                let Ok(info) = metadata::item::equip::build_equip_item_wz_info_by_wz(item_model.wz)
                else {
                    continue;
                };
                let item: MapleItem = MapleItem::from((info, item_model));
                if let Some(ipos) = item.ipos {
                    let filled_slot = MapleFilledItemSlot { ipos };
                    let filled_slot_entity =
                        commands.spawn((filled_slot, ChildOf(tab_entity))).id();
                    commands.spawn((item.clone(), ChildOf(filled_slot_entity)));
                    filled_slots.push(item);
                }
            }
        }
    }
    filled_slots
}
