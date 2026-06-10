use std::collections::{HashMap, HashSet};

use base::inventory::{
    ANDROID_EQUIP_SLOTS, CASH_EQUIP_SLOTS, PET_EQUIP_SLOTS, REGULAR_EQUIP_SLOTS,
};
use base::skill::BaseSkill;
use bevy::ecs::entity::Entity;
use bevy::ecs::hierarchy::ChildOf;
use bevy::ecs::system::Commands;
use db::item::model::ItemModel;
use db::keybinding::model::KeybindingModel;
use db::skill::model::SkillModel;

use crate::component::character::MapleCharacter;
use crate::component::inventory::{
    MapleCashTab, MapleEquipTab, MapleEquippedTab, MapleEtcTab, MapleInventory, MapleSetupTab,
    MapleUseTab,
};
use crate::component::item::MapleItem;
use crate::component::keybinding::MapleKeybinding;
use crate::component::skill::MapleSkill;
use crate::component::slot::MapleEmptyItemSlot;
use crate::system::packet::handler::codec::spawn_item;

pub fn spawn_char_with_equips(
    commands: &mut Commands,
    chars: HashMap<i32, (Entity, MapleCharacter)>,
    keybinding_model_map: &HashMap<i32, Vec<KeybindingModel>>,
    skill_model_map: &HashMap<i32, Vec<SkillModel>>,
    equipped_item_model_map: &HashMap<i32, Vec<ItemModel>>,
    equip_item_model_map: &HashMap<i32, Vec<ItemModel>>,
    use_item_model_map: &HashMap<i32, Vec<ItemModel>>,
    etc_item_model_map: &HashMap<i32, Vec<ItemModel>>,
    setup_item_model_map: &HashMap<i32, Vec<ItemModel>>,
    cash_item_model_map: &HashMap<i32, Vec<ItemModel>>,
    equip_tab_inv_capacity_map: &HashMap<i32, i16>,
    use_tab_inv_capacity_map: &HashMap<i32, i16>,
    etc_tab_inv_capacity_map: &HashMap<i32, i16>,
    setup_tab_inv_capacity_map: &HashMap<i32, i16>,
    cash_tab_inv_capacity_map: &HashMap<i32, i16>,
) -> HashMap<i32, Vec<MapleItem>> {
    let mut equipped_filled_slots_map: HashMap<i32, Vec<MapleItem>> = HashMap::new();
    for (_, (char_entity, char)) in chars.iter() {
        let Some(keybinding_models) = keybinding_model_map.get(&char.id) else {
            continue;
        };
        for keybinding_model in keybinding_models {
            let keybinding: MapleKeybinding = MapleKeybinding::from(keybinding_model.clone());
            commands.spawn((keybinding, ChildOf(*char_entity)));
        }
        let Some(skill_models) = skill_model_map.get(&char.id) else {
            continue;
        };
        for skill_model in skill_models {
            let base_skill: BaseSkill = BaseSkill { wz: skill_model.wz };
            let skill: MapleSkill = MapleSkill::from((base_skill, skill_model.clone()));
            commands.spawn((skill, ChildOf(*char_entity)));
        }
        let inventory: MapleInventory = MapleInventory;
        let inv_entity = commands.spawn((inventory, ChildOf(*char_entity))).id();
        let equipped_tab: MapleEquippedTab = MapleEquippedTab;
        let equipped_tab_entity = commands.spawn((equipped_tab, ChildOf(inv_entity))).id();
        let Some(equip_tab_capacity) = equip_tab_inv_capacity_map.get(&char.id) else {
            continue;
        };
        let equip_tab: MapleEquipTab = MapleEquipTab {
            capacity: *equip_tab_capacity,
        };
        let equip_tab_entity = commands.spawn((equip_tab, ChildOf(inv_entity))).id();
        let Some(use_tab_capacity) = use_tab_inv_capacity_map.get(&char.id) else {
            continue;
        };
        let use_tab: MapleUseTab = MapleUseTab {
            capacity: *use_tab_capacity,
        };
        let use_tab_entity = commands.spawn((use_tab, ChildOf(inv_entity))).id();
        let Some(etc_tab_capacity) = etc_tab_inv_capacity_map.get(&char.id) else {
            continue;
        };
        let etc_tab: MapleEtcTab = MapleEtcTab {
            capacity: *etc_tab_capacity,
        };
        let etc_tab_entity = commands.spawn((etc_tab, ChildOf(inv_entity))).id();
        let Some(setup_tab_capacity) = setup_tab_inv_capacity_map.get(&char.id) else {
            continue;
        };
        let setup_tab: MapleSetupTab = MapleSetupTab {
            capacity: *setup_tab_capacity,
        };
        let setup_tab_entity = commands.spawn((setup_tab, ChildOf(inv_entity))).id();
        let Some(cash_tab_capacity) = cash_tab_inv_capacity_map.get(&char.id) else {
            continue;
        };
        let cash_tab: MapleCashTab = MapleCashTab {
            capacity: *cash_tab_capacity,
        };
        let cash_tab_entity = commands.spawn((cash_tab, ChildOf(inv_entity))).id();

        let equipped_items: Vec<MapleItem> = spawn_item::spawn_item(
            commands,
            char.id,
            equipped_item_model_map,
            equipped_tab_entity,
        );
        let equipped_filled_pos: HashSet<i16> =
            equipped_items.iter().filter_map(|s| s.ipos).collect();
        let islots = CASH_EQUIP_SLOTS
            .iter()
            .chain(ANDROID_EQUIP_SLOTS.iter())
            .chain(REGULAR_EQUIP_SLOTS.iter())
            .chain(PET_EQUIP_SLOTS.iter())
            .filter(|islot| !equipped_filled_pos.contains(&islot.key));
        for islot in islots {
            commands.spawn((
                MapleEmptyItemSlot { ipos: islot.key },
                ChildOf(equipped_tab_entity),
            ));
        }
        let equip_filled_slots: Vec<MapleItem> =
            spawn_item::spawn_item(commands, char.id, equip_item_model_map, equip_tab_entity);
        let equip_filled_pos: HashSet<i16> =
            equip_filled_slots.iter().filter_map(|s| s.ipos).collect();
        let Some(equip_tab_inv_capacity) = equip_tab_inv_capacity_map.get(&char.id) else {
            continue;
        };
        for ipos in 0..*equip_tab_inv_capacity {
            if !equip_filled_pos.contains(&ipos) {
                commands.spawn((MapleEmptyItemSlot { ipos }, ChildOf(equip_tab_entity)));
            }
        }
        let use_filled_slots: Vec<MapleItem> =
            spawn_item::spawn_item(commands, char.id, use_item_model_map, use_tab_entity);
        let use_filled_pos: HashSet<i16> = use_filled_slots.iter().filter_map(|s| s.ipos).collect();
        let Some(use_tab_inv_capacity) = use_tab_inv_capacity_map.get(&char.id) else {
            continue;
        };
        for ipos in 0..*use_tab_inv_capacity {
            if !use_filled_pos.contains(&ipos) {
                commands.spawn((MapleEmptyItemSlot { ipos }, ChildOf(use_tab_entity)));
            }
        }
        let etc_filled_slots: Vec<MapleItem> =
            spawn_item::spawn_item(commands, char.id, etc_item_model_map, etc_tab_entity);
        let etc_filled_pos: HashSet<i16> = etc_filled_slots.iter().filter_map(|s| s.ipos).collect();
        let Some(etc_tab_inv_capacity) = etc_tab_inv_capacity_map.get(&char.id) else {
            continue;
        };
        for ipos in 0..*etc_tab_inv_capacity {
            if !etc_filled_pos.contains(&ipos) {
                commands.spawn((MapleEmptyItemSlot { ipos }, ChildOf(etc_tab_entity)));
            }
        }
        let setup_filled_slots: Vec<MapleItem> =
            spawn_item::spawn_item(commands, char.id, setup_item_model_map, setup_tab_entity);
        let setup_filled_pos: HashSet<i16> =
            setup_filled_slots.iter().filter_map(|s| s.ipos).collect();
        let Some(setup_tab_inv_capacity) = setup_tab_inv_capacity_map.get(&char.id) else {
            continue;
        };
        for ipos in 0..*setup_tab_inv_capacity {
            if !setup_filled_pos.contains(&ipos) {
                commands.spawn((MapleEmptyItemSlot { ipos }, ChildOf(setup_tab_entity)));
            }
        }
        let cash_filled_slots: Vec<MapleItem> =
            spawn_item::spawn_item(commands, char.id, cash_item_model_map, cash_tab_entity);
        let cash_filled_pos: HashSet<i16> =
            cash_filled_slots.iter().filter_map(|s| s.ipos).collect();
        let Some(cash_tab_inv_capacity) = cash_tab_inv_capacity_map.get(&char.id) else {
            continue;
        };
        for ipos in 0..*cash_tab_inv_capacity {
            if !cash_filled_pos.contains(&ipos) {
                commands.spawn((MapleEmptyItemSlot { ipos }, ChildOf(cash_tab_entity)));
            }
        }
        equipped_filled_slots_map.insert(char.id, equipped_items);
    }
    equipped_filled_slots_map
}
