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
use std::sync::Mutex;
use std::sync::mpsc::channel;

use bevy::app::{App, Plugin, Startup, Update};
use ipc::asyncronous::command::AsyncCommand;
use ipc::asyncronous::event::AsyncEvent;

use crate::resource::custom_resource::{CustomReceiver, CustomSender, Pool};
use crate::system::{event_handler, packet_dispatch, startup};

pub struct CustomPlugin;

impl Plugin for CustomPlugin {
    fn build(&self, app: &mut App) {
        let (command_tx, command_rx) = channel::<AsyncCommand>();
        let (event_tx, event_rx) = channel::<AsyncEvent>();
        std::thread::spawn(move || {
            tokio::runtime::Runtime::new().unwrap().block_on(async {
                runtime::server::start_server(command_rx, pool, event_tx)
                    .await
                    .unwrap();
            });
        });

        app.insert_resource(CustomReceiver(Mutex::new(event_rx)))
            .insert_resource(CustomSender(Mutex::new(command_tx)))
            .insert_resource(Pool(pool::new()))
            .add_systems(Startup, startup::spawn_worlds)
            .add_systems(Update, event_handler::handle_events_system)
            .add_systems(Update, packet_dispatch::packet_dispatch_system);
    }
}
