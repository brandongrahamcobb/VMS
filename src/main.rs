use futures::future::try_join_all;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::info;
use tracing_subscriber::EnvFilter;
use vms::config::settings;
use vms::models::error::ModelError;
use vms::models::world;
use vms::runtime::error::RuntimeError;
use vms::runtime::server::{ChannelServer, LoginServer};
use vms::runtime::state::{SharedState, State};

#[tokio::main]
async fn main() -> Result<(), RuntimeError> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("vms=debug".parse().unwrap()))
        .init();
    info!("Loading Shared State...");
    let shared_state: SharedState = Arc::new(Mutex::new(State::new()?));
    let port = settings::get_login_port()?;
    let login_state = shared_state.clone();
    let login_fut = LoginServer::run(login_state);
    let worlds = world::service::load_worlds()
        .map_err(ModelError::from)
        .map_err(RuntimeError::from)?;
    let mut channel_futs = Vec::new();
    for world in worlds {
        for channel in world.channels {
            info!("Loading {}:{}...", world.name, channel.id);
            let channel_state = shared_state.clone();
            channel_futs.push(ChannelServer::run(channel_state, channel.port));
        }
    }
    let joined_channel_futs = try_join_all(channel_futs);
    tokio::try_join!(login_fut, joined_channel_futs)?;
    Ok(())
}
