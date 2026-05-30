use bevy::ecs::message::MessageReader;
use net::packet::packet::model::Packet;

use crate::resource::custom_resource::{CustomSender, Sessions};

fn handle_login_request(
    mut messages: MessageReader<CredentialsMessage>,
    sessions: Res<Sessions>,
    command_tx: Res<CustomSender>,
) {
    for msg in messages.read() {
        let already_logged_in = sessions.0.values().any(|s| s.username == msg.username);
        if already_logged_in {
            command_tx
                .0
                .send(TcpCommand::SendPacket {
                    client_id: msg.client_id,
                    packet: Packet::new_empty().build_login_failed(FailedCode::AlreadyLoggedIn),
                })
                .unwrap();
            continue;
        }
        command_tx
            .0
            .send(TcpCommand::CheckCredentials {
                client_id: msg.client_id,
                username: msg.username,
                password: msg.password,
            })
            .unwrap();
    }
}
