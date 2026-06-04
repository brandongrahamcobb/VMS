/* app/src/plugin/custom_plugin.rs
 * The purpose of this module is to cross the thread boundary between Bevy and the TCP server.
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

use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::mpsc::channel;

use bevy::app::{App, Plugin, Startup, Update};
use config::settings;
use diesel::PgConnection;
use diesel::r2d2::ConnectionManager;
use ipc::asyncronous::command::AsyncCommand;
use ipc::asyncronous::event::AsyncEvent;

use crate::resource::custom_resource::{ClientMap, CustomReceiver, CustomSender};
use crate::system::{event_handler, result_handler, startup};

pub struct CustomServerPlugin;

impl Plugin for CustomServerPlugin {
    fn build(&self, app: &mut App) {
        let (command_tx, command_rx) = channel::<AsyncCommand>();
        let (event_tx, event_rx) = channel::<AsyncEvent>();
        match settings::get_db_url() {
            Ok(db_url) => {
                let manager = ConnectionManager::<PgConnection>::new(db_url);
                match diesel::r2d2::Pool::builder().build(manager) {
                    Ok(pool) => {
                        std::thread::spawn(move || {
                            tokio::runtime::Runtime::new().unwrap().block_on(async {
                                runtime::tcp::start_server(command_rx, event_tx, pool)
                                    .await
                                    .unwrap();
                            });
                        });
                        app.insert_resource(CustomReceiver(Mutex::new(event_rx)))
                            .insert_resource(CustomSender(Mutex::new(command_tx)))
                            .insert_resource(ClientMap(HashMap::new()))
                            .add_systems(Startup, startup::spawn_worlds)
                            .add_systems(Update, event_handler::handle_events_system)
                            .add_systems(Update, result_handler::result_handler_system);
                    }
                    Err(e) => tracing::error!("App startup error: {e}"),
                }
            }
            _ => {}
        }
    }
}
