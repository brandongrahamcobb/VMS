use std::time::SystemTime;

use base::{inventory::InventoryTab, item::BaseItem};
use db::item::model::ItemModel;
use inc::helpers;

use crate::syncronous::error::SyncDomainError;

pub fn create_item_model_by_wz(wz: i32) -> Result<ItemModel, SyncDomainError> {
    let itab: InventoryTab = metadata::item::inventory::get_inventory_tab_by_wz(wz)?;
    let item_model = {
        match itab {
            InventoryTab::Equip => {
                let item_wz_info: BaseItem =
                    metadata::item::equip::build_equip_item_wz_info_by_wz(wz)?;
                ItemModel {
                    id: None,
                    char_id: None,
                    ipos: None,
                    strength: helpers::calculate_rand_stat(item_wz_info.strength, 5),
                    dexterity: helpers::calculate_rand_stat(item_wz_info.dexterity, 5),
                    intelligence: helpers::calculate_rand_stat(item_wz_info.intelligence, 5),
                    luck: helpers::calculate_rand_stat(item_wz_info.luck, 5),
                    attack: helpers::calculate_rand_stat(item_wz_info.attack, 5),
                    weapon_defense: helpers::calculate_rand_stat(item_wz_info.weapon_defense, 10),
                    magic: helpers::calculate_rand_stat(item_wz_info.magic, 5),
                    magic_defense: helpers::calculate_rand_stat(item_wz_info.magic_defense, 10),
                    hp: helpers::calculate_rand_stat(item_wz_info.hp, 10),
                    mp: helpers::calculate_rand_stat(item_wz_info.mp, 10),
                    accuracy: helpers::calculate_rand_stat(item_wz_info.accuracy, 5),
                    avoid: helpers::calculate_rand_stat(item_wz_info.avoid, 5),
                    hands: helpers::calculate_rand_stat(item_wz_info.hands, 5),
                    speed: helpers::calculate_rand_stat(item_wz_info.speed, 5),
                    jump: helpers::calculate_rand_stat(item_wz_info.jump, 5),
                    wz,
                    slots: 0,      //placeholder
                    expire: 0,     //placeholder
                    level: 0,      //placeholder
                    flag: 0,       //placeholder
                    item_level: 0, //placeholder
                    item_exp: 0,   //placeholder
                    vicious: 0,    //placeholder
                    equipped: false,
                    created_at: Some(SystemTime::now()),
                    updated_at: SystemTime::now(),
                }
            }
            _ => ItemModel {
                id: None,
                char_id: None,
                ipos: None,
                strength: 0,
                dexterity: 0,
                intelligence: 0,
                luck: 0,
                attack: 0,
                weapon_defense: 0,
                magic: 0,
                magic_defense: 0,
                hp: 0,
                mp: 0,
                accuracy: 0,
                avoid: 0,
                hands: 0,
                speed: 0,
                jump: 0,
                wz,
                slots: 0,
                expire: 0,
                level: 0,
                item_level: 0,
                flag: 0,
                item_exp: 0,
                vicious: 0,
                equipped: false,
                created_at: Some(SystemTime::now()),
                updated_at: SystemTime::now(),
            },
        }
    };
    Ok(item_model)
}
