#[cfg(test)]

pub mod tests {
    use core::net::SocketAddr;
    use std::{net::UdpSocket, time::SystemTime};

    use bevy::app::App;
    use bevy::app::Update;
    use bevy::ecs::system::ResMut;
    use bevy_renet::netcode::{ClientAuthentication, NetcodeClientPlugin, NetcodeClientTransport};
    use bevy_renet::renet::ConnectionConfig;
    use bevy_renet::renet::DefaultChannel;
    use bevy_renet::{RenetClient, RenetClientPlugin};
    use config::settings;
    use inc::helpers;

    use crate::error::HarnessError;

    #[tokio::test]
    async fn main() -> Result<(), HarnessError> {
        dotenvy::dotenv().ok();
        let mut app = App::new();
        app.add_plugins(RenetClientPlugin);

        let client = RenetClient::new(ConnectionConfig::default());
        app.insert_resource(client);

        app.add_plugins(NetcodeClientPlugin);

        match get_server_addr() {
            Ok(addr) => {
                let authentication = ClientAuthentication::Unsecure {
                    server_addr: addr,
                    client_id: 0,
                    user_data: None,
                    protocol_id: 0,
                };
                let socket = UdpSocket::bind(get_server_addr()?).unwrap();
                let current_time = SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap();
                let transport =
                    NetcodeClientTransport::new(current_time, authentication, socket).unwrap();

                app.insert_resource(transport);

                app.add_systems(Update, (send_message_system, receive_message_system));
            }
            Err(e) => tracing::error!("Bevy client error in test harnes layer: {e}"),
        }
        Ok(())
    }

    fn send_message_system(mut client: ResMut<RenetClient>) {
        client.send_message(DefaultChannel::ReliableOrdered, "server message");
    }

    fn receive_message_system(mut client: ResMut<RenetClient>) {
        while let Some(message) = client.receive_message(DefaultChannel::ReliableOrdered) {
            std::hint::black_box(message);
        }
    }

    fn get_server_addr() -> Result<SocketAddr, HarnessError> {
        let port: i16 = settings::get_login_port()?;
        let bind: String = settings::get_bind_address()?;
        let addr: SocketAddr = helpers::build_server_addr(bind, port);
        Ok(addr)
    }
}
