use crate::config::settings;
use crate::runtime::error::RuntimeError;
use crate::runtime::relay::{ChannelRelay, LoginRelay, Runtime};
use crate::runtime::state::SharedState;
use tracing::info;

pub enum ServerType {
    LoginServer,
    ChannelServer,
}

pub struct LoginServer;

impl LoginServer {
    pub async fn run(shared_state: SharedState) -> Result<(), RuntimeError> {
        let port = settings::get_login_port()?;
        let addr = settings::build_server_addr(&port)?;
        let listener = tokio::net::TcpListener::bind(&addr).await?;
        loop {
            match listener.accept().await {
                Ok((stream, _addr)) => {
                    let clone = shared_state.clone();
                    tokio::spawn(async move {
                        match Runtime::<LoginRelay>::new(clone, stream).await {
                            Ok(mut relay) => {
                                info!("Listening on port {}...", port);
                                if let Err(e) = relay.run().await {
                                    use std::error::Error;
                                    let mut current: Option<&dyn Error> = Some(&e);
                                    while let Some(err) = current {
                                        println!("{}", err);
                                        current = err.source();
                                    }
                                    info!(
                                        "Expected a successful core relay loop. Received an error. Error: {}",
                                        e.to_string(),
                                    );
                                }
                            }
                            Err(e) => info!(
                                "Expected valid core relay creation. Received an error. Error: {}",
                                e.to_string(),
                            ),
                        };
                    });
                }
                Err(e) => info!(
                    "Expected valid connection. Received an error. Error: {}",
                    e.to_string(),
                ),
            }
        }
    }
}

pub struct ChannelServer;

impl ChannelServer {
    pub async fn run(shared_state: SharedState, port: i16) -> Result<(), RuntimeError> {
        let addr = settings::build_server_addr(&port)?;
        let listener = tokio::net::TcpListener::bind(&addr).await?;
        loop {
            match listener.accept().await {
                Ok((stream, _addr)) => {
                    let clone = shared_state.clone();
                    tokio::spawn(async move {
                        match Runtime::<ChannelRelay>::new(clone, stream).await {
                            Ok(mut relay) => {
                                info!("Listening on port {}...", port);
                                if let Err(e) = relay.run().await {
                                    use std::error::Error;
                                    let mut current: Option<&dyn Error> = Some(&e);
                                    while let Some(err) = current {
                                        println!("{}", err);
                                        current = err.source();
                                    }
                                    info!(
                                        "Expected a successful channel relay loop on port {}. Received an error. Error: {}",
                                        port,
                                        e.to_string(),
                                    );
                                }
                            }
                            Err(e) => info!(
                                "Expected valid world relay creation on port {}. Received an error. Error: {}",
                                port,
                                e.to_string(),
                            ),
                        };
                    });
                }
                Err(e) => info!(
                    "Expected valid connection on port {}. Received an error. Error: {}",
                    port,
                    e.to_string(),
                ),
            }
        }
    }
}
