/* main.rs
 * The purpose of this program is to run the VMS Mushroom-based private server in Rust.
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

use runtime::server::{LoginServer, PlayerServer};
use server::error::VMSError;
use state::model::{SharedState, State};
use std::sync::Arc;
use tick::manager::TickManager;
use tokio::{sync::Mutex, try_join};
use tracing::info;
use tracing_subscriber::EnvFilter;

#[tokio::main]
#[warn(dead_code)]
async fn main() -> Result<(), VMSError> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::DEBUG.into()))
        .init();
    let state: SharedState = Arc::new(Mutex::new(State::new().map_err(Box::new)?));
    {
        let tick_manager = TickManager::new();
        tick_manager.spawn_ticks(&state).await.map_err(Box::new)?;
    }
    info!("Loading login port...");
    let login = LoginServer::run(state.clone());
    info!("Loading player ports...");
    let player = PlayerServer::run(state.clone());
    let (_, _) = try_join!(login, player).map_err(Box::new)?;
    Ok(())
}
