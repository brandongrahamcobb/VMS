use futures::future::try_join_all;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::info;
use tracing_subscriber::EnvFilter;
use vms::runtime::error::RuntimeError;
use vms::runtime::server::{LoginServer, PlayerServer};
use vms::runtime::state::{SharedState, State};

#[tokio::main]
async fn main() -> Result<(), RuntimeError> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("vms=debug".parse().unwrap()))
        .init();
    let shared_state: SharedState = Arc::new(Mutex::new(State::new()?));
    info!("Loading Server...");
    LoginServer::run(shared_state).await
}
