use std::sync::Arc;
use tracing::info;
use tracing_subscriber::EnvFilter;
use vms::runtime::error::RuntimeError;
use vms::runtime::server::{CoreServer, WorldServer};
use vms::runtime::state::{SharedState, State};

#[tokio::main]
async fn main() -> Result<(), RuntimeError> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("vms=debug".parse().unwrap()))
        .init();
    info!("Loading Shared State...");
    let shared_state: SharedState = Arc::new(State::new()?);
    info!("Starting Core Server...");
    let core = CoreServer::run(&shared_state); //.await?;
    info!("Starting World Server...");
    let world = WorldServer::run(&shared_state); //.await?;
    tokio::try_join!(core, world)?;
    Ok(())
}
