/* app/src/server/custom_server.rs
 * The purpose of this module is to provide the Bevy renet server.
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
use core::net::SocketAddr;

use app::plugin::dispatch_plugin::PacketDispatchPlugin;
use app::plugin::handler_plugin::{GamePacketHandlerPlugin, LoginPacketHandlerPlugin};
use app::plugin::request_plugin::RequestPlugin;
use app::plugin::response_plugin::ResponsePlugin;
use app::plugin::server_plugin::CustomServerPlugin;
use bevy::MinimalPlugins;
use bevy::app::App;
use bevy_renet::netcode::{NetcodeClientPlugin, NetcodeServerTransport, ServerConfig};
use bevy_renet::renet::ConnectionConfig;
use bevy_renet::{RenetServer, RenetServerPlugin};
use config::error::ConfigError;
use config::settings;
use inc::helpers;
use std::{net::UdpSocket, time::SystemTime};
use tracing_subscriber::EnvFilter;

fn main() -> () {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::DEBUG.into()))
        .init();
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .add_plugins(RenetServerPlugin)
        // .add_plugins(NetcodeClientPlugin)
        .add_plugins(RequestPlugin)
        .add_plugins(ResponsePlugin)
        .add_plugins(PacketDispatchPlugin)
        .add_plugins(LoginPacketHandlerPlugin)
        .add_plugins(GamePacketHandlerPlugin)
        .add_plugins(CustomServerPlugin);
    let server = RenetServer::new(ConnectionConfig::default());
    app.insert_resource(server);
    match get_server_addr() {
        Ok(addr) => {
            let socket = UdpSocket::bind(addr).unwrap();
            let server_config = ServerConfig {
                current_time: SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap(),
                max_clients: 255,
                protocol_id: 0,
                public_addresses: vec![addr],
                authentication: bevy_renet::netcode::ServerAuthentication::Unsecure,
            };
            let transport = NetcodeServerTransport::new(server_config, socket).unwrap();
            app.insert_resource(transport);
        }
        Err(e) => tracing::error!("Failed to get a valid server address in Bevy app layer: {e}"),
    };
    app.run();
}

fn get_server_addr() -> Result<SocketAddr, ConfigError> {
    let port: i16 = settings::get_login_port()?;
    let bind: String = settings::get_bind_address()?;
    let addr = helpers::build_server_addr(bind, port);
    Ok(addr)
}
