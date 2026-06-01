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

use crate::error::HarnessError;
use crate::plugin::tcp::AsyncEvent;
use crate::resource::custom_resource::CustomReceiver;
use crate::resource::custom_resource::CustomSender;
use bevy::app::App;
use bevy::app::Update;
use bevy::ecs::observer::On;
use bevy::ecs::system::ResMut;
use bevy::prelude::{Add, Remove};
use bevy_renet::netcode::{NetcodeClientPlugin, NetcodeServerTransport, ServerConfig};
use bevy_renet::renet::ConnectionConfig;
use bevy_renet::renet::DefaultChannel;
use bevy_renet::{RenetServer, RenetServerPlugin};
use bevy_replicon::prelude::ConnectedClient;
use config::settings;
use inc::helpers;
use plugins::custom_plugin::CustomPlugin;
use server::error::VMSError;
use std::{net::UdpSocket, time::SystemTime};

use crate::error::HarnessError;

async fn main() -> Result<(), VMSError> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::DEBUG.into()))
        .init();
    let mut app = App::new();
    app.add_plugins(RenetServerPlugin);
    app.add_plugins(CustomPlugin);

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

    app.add_systems(
        Update,
        (super::send_message_system, super::receive_message_system),
    );
    app.add_observer(super::on_client_connected);
    app.add_observer(super::on_client_disconnected);
    Ok(())
}

fn send_message_system(mut server: ResMut<RenetServer>) {
    let channel_id = 0;
    server.broadcast_message(DefaultChannel::ReliableOrdered, "server message");
}

fn receive_message_system(mut server: ResMut<RenetServer>) {
    for client_id in server.clients_id() {
        while let Some(message) = server.receive_message(client_id, DefaultChannel::ReliableOrdered)
        {
            std::hint::black_box(message);
        }
    }
}

fn on_client_connected(trigger: On<Add, ConnectedClient>) {
    let client_entity = trigger.entity;
    std::hint::black_box(client_entity);
}

fn on_client_disconnected(trigger: On<Remove, ConnectedClient>) {
    let client_entity = trigger.entity;
    std::hint::black_box(client_entity);
}

fn get_server_addr() -> Result<SocketAddr, HarnessError> {
    let port: i16 = settings::get_login_port()?;
    let bind: String = settings::get_bind_address()?;
    let addr = helpers::build_server_addr(bind, port);
    Ok(addr)
}
