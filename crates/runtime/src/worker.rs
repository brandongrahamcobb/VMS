/* runtime/src/workers/login.rs
 * The purpose of this module is to handle asyncronous commands during login.
 *
 * Copyright (C) 2026  https://github.com/brandongrahamcobb/VMS.git
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use base::account::StatusCode;
use base::inventory::InventoryTab;
use base::skill::BaseSkill;
use base::{account::FailedCode, character::StatsUpdate};
use config::settings;
use db::inventory::model::InventoryCapacityModel;
use db::item::model::ItemModel;
use db::keybinding::model::KeybindingModel;
use db::pool::DbPool;
use db::{character::model::CharacterModel, skill::model::SkillModel};
use ipc::{asyncronous::db_command::DatabaseCommand, syncronous};
use std::collections::HashMap;
use std::sync::mpsc::Sender;
use std::time::SystemTime;
use tokio::sync::mpsc::Receiver;

use ipc::asyncronous::event::AsyncEvent;

use crate::error::RuntimeError;

pub async fn db_worker(
    mut db_rx: Receiver<DatabaseCommand>,
    pool: DbPool,
    event_tx: Sender<AsyncEvent>,
) -> Result<(), RuntimeError> {
    loop {
        let cmd = { db_rx.try_recv() };
        match cmd {
            Ok(DatabaseCommand::LoginRequest {
                client_id,
                username,
                password,
            }) => {
                let event =
                    match db::account::getters::get_acc_model_by_username(&pool, username.clone())
                        .await
                    {
                        Ok(acc_model) => {
                            let authenticated = syncronous::account::authenticate(
                                acc_model.password.clone(),
                                password.clone(),
                            );
                            let acc_id: i32 = acc_model.get_id()?;
                            match authenticated {
                                Ok(true) => {
                                    let status =
                                        syncronous::account::get_status_code_by_account(&acc_model);
                                    match status {
                                        StatusCode::Failed(code) => {
                                            AsyncEvent::LoginFailed { client_id, code }
                                        }
                                        _ => AsyncEvent::LoginSuccess {
                                            client_id,
                                            acc_id,
                                            acc_model,
                                        },
                                    }
                                }
                                Ok(false) => AsyncEvent::LoginFailed {
                                    client_id,
                                    code: FailedCode::InvalidCredentials,
                                },
                                Err(_) => AsyncEvent::LoginFailed {
                                    client_id,
                                    code: FailedCode::InvalidCredentials,
                                },
                            }
                        }
                        Err(_) => AsyncEvent::LoginFailed {
                            client_id,
                            code: FailedCode::UnknownCredentials,
                        },
                    };
                event_tx.send(event).unwrap();
            }
            Ok(DatabaseCommand::AcceptTosRequest { client_id, acc_id }) => {
                std::hint::black_box(client_id);
                db::account::setters::accept_tos_by_acc_id(&pool, acc_id).await?;
            }
            Ok(DatabaseCommand::ListCharsRequest {
                client_id,
                acc_id,
                channel_id,
                world_id,
            }) => {
                let slots: i16 = db::character::getters::get_char_max_by_account_and_world_id(
                    &pool, acc_id, world_id,
                )
                .await
                .unwrap_or(8);
                let char_models: Vec<CharacterModel> =
                    db::character::getters::get_char_models_by_acc_id_and_world_id(
                        &pool, acc_id, world_id,
                    )
                    .await?;
                let mut skill_model_map: HashMap<i32, Vec<SkillModel>> = HashMap::new();
                let mut keybinding_model_map: HashMap<i32, Vec<KeybindingModel>> = HashMap::new();
                let mut equipped_item_model_map: HashMap<i32, Vec<ItemModel>> = HashMap::new();
                let mut equip_item_model_map: HashMap<i32, Vec<ItemModel>> = HashMap::new();
                let mut use_item_model_map: HashMap<i32, Vec<ItemModel>> = HashMap::new();
                let mut etc_item_model_map: HashMap<i32, Vec<ItemModel>> = HashMap::new();
                let mut setup_item_model_map: HashMap<i32, Vec<ItemModel>> = HashMap::new();
                let mut cash_item_model_map: HashMap<i32, Vec<ItemModel>> = HashMap::new();
                let mut equip_tab_inv_capacity_map: HashMap<i32, i16> = HashMap::new();
                let mut use_tab_inv_capacity_map: HashMap<i32, i16> = HashMap::new();
                let mut etc_tab_inv_capacity_map: HashMap<i32, i16> = HashMap::new();
                let mut setup_tab_inv_capacity_map: HashMap<i32, i16> = HashMap::new();
                let mut cash_tab_inv_capacity_map: HashMap<i32, i16> = HashMap::new();
                for char_model in char_models.iter() {
                    let Some(char_id) = char_model.id else {
                        continue;
                    };
                    let skill_models: Vec<SkillModel> =
                        db::skill::getters::get_skill_models_by_char_id(&pool, char_id).await?;
                    skill_model_map.insert(char_id, skill_models);
                    let keybinding_models: Vec<KeybindingModel> =
                        db::keybinding::getters::get_keybinding_models_by_char_id(&pool, char_id)
                            .await?;
                    keybinding_model_map.insert(char_id, keybinding_models);
                    let equipped_item_models: Vec<ItemModel> =
                        db::item::getters::get_equipped_item_models_by_char_id(&pool, char_id)
                            .await?;
                    equipped_item_model_map.insert(char_id, equipped_item_models);
                    let unequipped_item_models: Vec<ItemModel> =
                        db::item::getters::get_unequipped_item_models_by_char_id(&pool, char_id)
                            .await?;
                    let inv_cap: InventoryCapacityModel =
                        db::inventory::getters::get_inventory_slot_capacities_by_char_id(
                            &pool, char_id,
                        )
                        .await?;
                    equip_tab_inv_capacity_map.insert(char_id, inv_cap.equip_slot_capacity);
                    use_tab_inv_capacity_map.insert(char_id, inv_cap.use_slot_capacity);
                    etc_tab_inv_capacity_map.insert(char_id, inv_cap.etc_slot_capacity);
                    setup_tab_inv_capacity_map.insert(char_id, inv_cap.setup_slot_capacity);
                    cash_tab_inv_capacity_map.insert(char_id, inv_cap.cash_slot_capacity);
                    let mut equip_tab_item_models: Vec<ItemModel> = Vec::new();
                    let mut use_tab_item_models: Vec<ItemModel> = Vec::new();
                    let mut etc_tab_item_models: Vec<ItemModel> = Vec::new();
                    let mut setup_tab_item_models: Vec<ItemModel> = Vec::new();
                    let mut cash_tab_item_models: Vec<ItemModel> = Vec::new();
                    for unequipped_item_model in unequipped_item_models {
                        match metadata::item::inventory::get_inventory_tab_by_wz(
                            unequipped_item_model.wz,
                        )? {
                            InventoryTab::Equip => {
                                equip_tab_item_models.push(unequipped_item_model)
                            }
                            InventoryTab::Use => use_tab_item_models.push(unequipped_item_model),
                            InventoryTab::Etc => etc_tab_item_models.push(unequipped_item_model),
                            InventoryTab::Setup => {
                                setup_tab_item_models.push(unequipped_item_model)
                            }
                            InventoryTab::Cash => cash_tab_item_models.push(unequipped_item_model),
                        }
                    }
                    equip_item_model_map.insert(char_id, equip_tab_item_models);
                    use_item_model_map.insert(char_id, use_tab_item_models);
                    etc_item_model_map.insert(char_id, etc_tab_item_models);
                    setup_item_model_map.insert(char_id, setup_tab_item_models);
                    cash_item_model_map.insert(char_id, cash_tab_item_models);
                }
                let event = AsyncEvent::ListCharsSuccess {
                    client_id,
                    channel_id,
                    char_models,
                    keybinding_model_map,
                    skill_model_map,
                    equipped_item_model_map,
                    equip_item_model_map,
                    use_item_model_map,
                    etc_item_model_map,
                    setup_item_model_map,
                    cash_item_model_map,
                    equip_tab_inv_capacity_map,
                    use_tab_inv_capacity_map,
                    etc_tab_inv_capacity_map,
                    setup_tab_inv_capacity_map,
                    cash_tab_inv_capacity_map,
                    slots,
                    world_id,
                };
                event_tx.send(event).unwrap();
            }
            Ok(DatabaseCommand::CharNameRequest { client_id, ign }) => {
                let exists = db::character::getters::get_char_model_by_name(&pool, ign.clone())
                    .await
                    .is_ok();
                let event = AsyncEvent::CheckCharName {
                    client_id,
                    exists,
                    ign,
                };
                event_tx.send(event).unwrap();
            }
            Ok(DatabaseCommand::SelectCharWithPicRequest {
                client_id,
                acc_id,
                char_id,
                mac,
                hwid,
                pic,
            }) => {
                std::hint::black_box(mac);
                std::hint::black_box(hwid);
                let Ok(acc_model) = db::account::getters::get_acc_model_by_id(&pool, acc_id).await
                else {
                    continue;
                };
                let status = syncronous::account::check_pic(acc_model.pic, pic);
                let event = AsyncEvent::SelectCharWithPic {
                    client_id,
                    char_id,
                    status,
                };
                event_tx.send(event).unwrap();
            }
            Ok(DatabaseCommand::CreateCharRequest {
                client_id,
                char_model,
                top_wz,
                bottom_wz,
                shoes_wz,
                weapon_wz,
            }) => {
                let char_models: Vec<CharacterModel> =
                    db::character::setters::update_characters(&pool, vec![char_model]).await?;
                let char_id: i32 = char_models[0].get_id()?;
                let equip_models = ipc::syncronous::char::create_new_char_equip_models(
                    char_id, top_wz, bottom_wz, shoes_wz, weapon_wz,
                )?;
                let mut equipped_item_model_map: HashMap<i32, Vec<ItemModel>> = HashMap::new();
                equipped_item_model_map.insert(
                    char_id,
                    db::item::setters::update_items(&pool, equip_models).await?,
                );
                let mut keybinding_model_map: HashMap<i32, Vec<KeybindingModel>> = HashMap::new();
                let keybinding_models: Vec<KeybindingModel> =
                    ipc::syncronous::char::create_new_char_keybinding_models(char_id);
                keybinding_model_map.insert(
                    char_id,
                    db::keybinding::setters::update_keybindings(&pool, keybinding_models).await?,
                );
                let mut skill_model_map: HashMap<i32, Vec<SkillModel>> = HashMap::new();
                let skill_models = ipc::syncronous::char::create_new_char_skill_models(
                    char_id,
                    char_models[0].job_wz,
                )?;
                skill_model_map.insert(
                    char_id,
                    db::skill::setters::update_skills(&pool, skill_models).await?,
                );
                let equip_item_model_map: HashMap<i32, Vec<ItemModel>> = HashMap::new();
                let use_item_model_map: HashMap<i32, Vec<ItemModel>> = HashMap::new();
                let etc_item_model_map: HashMap<i32, Vec<ItemModel>> = HashMap::new();
                let setup_item_model_map: HashMap<i32, Vec<ItemModel>> = HashMap::new();
                let cash_item_model_map: HashMap<i32, Vec<ItemModel>> = HashMap::new();
                let default_capacity: i16 = settings::get_inv_capacity()?;
                let inv_cap_model: InventoryCapacityModel = InventoryCapacityModel {
                    id: None,
                    char_id,
                    equip_slot_capacity: default_capacity,
                    use_slot_capacity: default_capacity,
                    etc_slot_capacity: default_capacity,
                    setup_slot_capacity: default_capacity,
                    cash_slot_capacity: default_capacity,
                    created_at: Some(SystemTime::now()),
                    updated_at: SystemTime::now(),
                };
                db::inventory::setters::update_inventory_capacity(&pool, &inv_cap_model).await?;
                let mut equip_tab_inv_capacity_map: HashMap<i32, i16> = HashMap::new();
                equip_tab_inv_capacity_map.insert(char_id, inv_cap_model.equip_slot_capacity);
                let mut use_tab_inv_capacity_map: HashMap<i32, i16> = HashMap::new();
                use_tab_inv_capacity_map.insert(char_id, inv_cap_model.use_slot_capacity);
                let mut etc_tab_inv_capacity_map: HashMap<i32, i16> = HashMap::new();
                etc_tab_inv_capacity_map.insert(char_id, inv_cap_model.etc_slot_capacity);
                let mut setup_tab_inv_capacity_map: HashMap<i32, i16> = HashMap::new();
                setup_tab_inv_capacity_map.insert(char_id, inv_cap_model.setup_slot_capacity);
                let mut cash_tab_inv_capacity_map: HashMap<i32, i16> = HashMap::new();
                cash_tab_inv_capacity_map.insert(char_id, inv_cap_model.cash_slot_capacity);
                event_tx
                    .send(AsyncEvent::CharCreationSuccess {
                        client_id,
                        char_model: char_models[0].clone(),
                        equipped_item_model_map,
                        equip_item_model_map,
                        use_item_model_map,
                        etc_item_model_map,
                        setup_item_model_map,
                        cash_item_model_map,
                        keybinding_model_map,
                        skill_model_map,
                        equip_tab_inv_capacity_map,
                        use_tab_inv_capacity_map,
                        etc_tab_inv_capacity_map,
                        setup_tab_inv_capacity_map,
                        cash_tab_inv_capacity_map,
                    })
                    .unwrap();
            }
            Ok(DatabaseCommand::FinishCharRequest {
                client_id,
                equip_models,
                keybinding_models,
                skill_models,
            }) => {
                std::hint::black_box(client_id);
                db::item::setters::update_items(&pool, equip_models).await?;
                db::keybinding::setters::update_keybindings(&pool, keybinding_models).await?;
                db::skill::setters::update_skills(&pool, skill_models).await?;
            }
            Ok(DatabaseCommand::DeleteCharRequest { client_id, char_id }) => {
                std::hint::black_box(client_id);
                db::character::setters::delete_char_by_id(&pool, char_id).await?;
            }
            Ok(DatabaseCommand::JoinRequest { client_id, char_id }) => {
                let char_model: CharacterModel =
                    db::character::getters::get_char_model_by_id(&pool, char_id).await?;
                let skill_models: Vec<SkillModel> =
                    db::skill::getters::get_skill_models_by_char_id(&pool, char_id).await?;
                let keybinding_models: Vec<KeybindingModel> =
                    db::keybinding::getters::get_keybinding_models_by_char_id(&pool, char_id)
                        .await?;
                let equipped_item_models: Vec<ItemModel> =
                    db::item::getters::get_equipped_item_models_by_char_id(&pool, char_id).await?;
                let unequipped_item_models: Vec<ItemModel> =
                    db::item::getters::get_unequipped_item_models_by_char_id(&pool, char_id)
                        .await?;
                let inv_cap: InventoryCapacityModel =
                    db::inventory::getters::get_inventory_slot_capacities_by_char_id(
                        &pool, char_id,
                    )
                    .await?;
                let mut equip_tab_item_models: Vec<ItemModel> = Vec::new();
                let mut use_tab_item_models: Vec<ItemModel> = Vec::new();
                let mut etc_tab_item_models: Vec<ItemModel> = Vec::new();
                let mut setup_tab_item_models: Vec<ItemModel> = Vec::new();
                let mut cash_tab_item_models: Vec<ItemModel> = Vec::new();
                for unequipped_item_model in unequipped_item_models {
                    match metadata::item::inventory::get_inventory_tab_by_wz(
                        unequipped_item_model.wz,
                    )? {
                        InventoryTab::Equip => equip_tab_item_models.push(unequipped_item_model),
                        InventoryTab::Use => use_tab_item_models.push(unequipped_item_model),
                        InventoryTab::Etc => etc_tab_item_models.push(unequipped_item_model),
                        InventoryTab::Setup => setup_tab_item_models.push(unequipped_item_model),
                        InventoryTab::Cash => cash_tab_item_models.push(unequipped_item_model),
                    }
                }
                let event = AsyncEvent::JoinSuccess {
                    client_id,
                    char_id,
                    map_wz: char_model.map_wz,
                    keybinding_models,
                    skill_models,
                    equipped_item_models,
                    equip_tab_item_models,
                    use_tab_item_models,
                    etc_tab_item_models,
                    setup_tab_item_models,
                    cash_tab_item_models,
                    equip_tab_capacity: inv_cap.equip_slot_capacity,
                    use_tab_capacity: inv_cap.use_slot_capacity,
                    etc_tab_capacity: inv_cap.etc_slot_capacity,
                    setup_tab_capacity: inv_cap.setup_slot_capacity,
                    cash_tab_capacity: inv_cap.cash_slot_capacity,
                };
                event_tx.send(event).unwrap();
            }
            Ok(DatabaseCommand::CloseAttackRequest {
                client_id,
                char_id,
                count,
                skill_id,
                display,
                toleft,
                stance,
                speed,
                mob_damages,
            }) => {
                let skill_model = db::skill::getters::get_skill_model_by_character_id_and_skill_id(
                    &pool, char_id, skill_id,
                )
                .await?;
                let base_skill = BaseSkill { wz: skill_model.wz };
                let event = AsyncEvent::CloseAttackSuccess {
                    client_id,
                    skill_model,
                    base_skill,
                    count,
                    display,
                    toleft,
                    stance,
                    speed,
                    mob_damages,
                };
                event_tx.send(event).unwrap();
            }
            Ok(DatabaseCommand::UpdateKeybindings { client_id, binds }) => {
                std::hint::black_box(client_id);
                db::keybinding::setters::update_keybindings(&pool, binds).await?;
            }
            Ok(DatabaseCommand::UpdateStats {
                client_id,
                char_id,
                updates,
            }) => {
                std::hint::black_box(client_id);
                let mut char_model =
                    db::character::getters::get_char_model_by_id(&pool, char_id).await?;
                for stat in updates {
                    match stat {
                        StatsUpdate::Exp { exp } => char_model.exp = exp,
                        StatsUpdate::Str { strength } => char_model.strength = strength,
                        StatsUpdate::Dex { dexterity } => char_model.dexterity = dexterity,
                        StatsUpdate::Luk { luck } => char_model.luck = luck,
                        StatsUpdate::Int { intelligence } => char_model.intelligence = intelligence,
                        StatsUpdate::Health { hp } => char_model.hp = hp,
                        StatsUpdate::Mana { mp } => char_model.mp = mp,
                        StatsUpdate::MaxHealth { max_hp } => char_model.max_hp = max_hp,
                        StatsUpdate::MaxMana { max_mp } => char_model.max_mp = max_mp,
                        StatsUpdate::AbilityPoints { ap } => char_model.ap = ap,
                        StatsUpdate::SkillPoints { sp } => char_model.sp = sp,
                        StatsUpdate::Level { level } => char_model.level = level,
                    }
                }
                db::character::setters::update_characters(&pool, vec![char_model]).await?;
            }
            Ok(DatabaseCommand::PickupItem {
                client_id,
                char_id,
                item_id,
                ipos,
                pet_pickup,
            }) => {
                std::hint::black_box(client_id);
                let mut item_model =
                    db::item::getters::get_item_model_by_item_id(&pool, item_id).await?;
                item_model.char_id = Some(char_id);
                item_model.ipos = Some(ipos);
                db::item::setters::update_items(&pool, vec![item_model]).await?;
                event_tx
                    .send(AsyncEvent::PickupSuccess {
                        client_id,
                        item_id,
                        ipos,
                        pet_pickup,
                    })
                    .unwrap();
            }
            Ok(DatabaseCommand::UpdateMapRequest {
                client_id,
                char_id,
                map_wz,
            }) => {
                std::hint::black_box(client_id);
                let mut char_model: CharacterModel =
                    db::character::getters::get_char_model_by_id(&pool, char_id).await?;
                char_model.map_wz = map_wz;
                db::character::setters::update_characters(&pool, vec![char_model]).await?;
            }

            Ok(_) => {
                tokio::task::yield_now().await;
                continue;
            }
            Err(_) => {
                tokio::task::yield_now().await;
                continue;
            }
        }
    }
}
