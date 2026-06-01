#[cfg(test)]

pub mod tests {
    use std::{net::UdpSocket, time::SystemTime};

    use bevy::app::App;
    use bevy::app::Update;
    use bevy_renet::netcode::{NetcodeClientPlugin, NetcodeServerTransport, ServerConfig};
    use bevy_renet::renet::ConnectionConfig;
    use bevy_renet::{RenetServer, RenetServerPlugin};

    use crate::error::HarnessError;

    #[tokio::test]
    async fn main() -> Result<(), HarnessError> {
        dotenvy::dotenv().ok();
        let mut app = App::new();
        app.add_plugins(RenetServerPlugin);

        let server = RenetServer::new(ConnectionConfig::default());
        app.insert_resource(server);

        app.add_plugins(NetcodeClientPlugin);
        let server_addr = super::get_server_addr()?;
        let socket = UdpSocket::bind(server_addr).unwrap();
        let server_config = ServerConfig {
            current_time: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap(),
            max_clients: 255,
            protocol_id: 0,
            public_addresses: vec![server_addr],
            authentication: bevy_renet::netcode::ServerAuthentication::Unsecure,
        };
        let transport = NetcodeServerTransport::new(server_config, socket).unwrap();

        app.insert_resource(transport);

        Ok(())
    }
}
