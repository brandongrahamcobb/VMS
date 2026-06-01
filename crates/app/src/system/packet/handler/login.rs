use crate::component::account::MapleAccount;
use crate::message::packet::login_started::LoginRequestMessage;
use crate::message::packet::login_started::LoginResponseMessage;
use crate::resource::custom_resource::ClientMap;
use crate::resource::custom_resource::CustomSender;
use crate::system::packet::build::codec;
use crate::system::packet::handler::result::HandlerResult;
use bevy::ecs::message::MessageReader;
use bevy::ecs::message::MessageWriter;
use bevy::ecs::system::Commands;
use bevy::ecs::system::{Query, Res};
use net::packet::packet::model::Packet;

fn handle_login_request(
    client_map: Res<ClientMap>,
    mut messages: MessageReader<LoginRequestMessage>,
    command_tx: CustomSender<AsyncCommand>,
    mut results: MessageWriter<HandlerResult>,
    accounts: Query<(Entity, &MapleAccount)>,
) {
    for msg in messages.read() {
        let Some(&client_entity) = client_map.0.get(&msg.client_id) else {
            continue;
        };
        let Some(already_logged_in) = accounts.iter().find(|a| a.username == msg.username) else {
            continue;
        };
        if already_logged_in {
            let Ok(login_failed_packet) = codec::login::builder::build_failed_login_packet(
                FailedCode::AlreadyLoggedIn as i16,
            );
            results.write(HandlerResult {
                client_id: msg.client_id,
                actions: vec![Action::Session(SessionAction::Send {
                    login_failed_packet.finish(),
                    scope: SessionScope::Local,
                })],
            });
        }
        command_tx
            .0
            .send(AsyncCommand::RequestLogin {
                client_id: msg.client_id,
                username: msg.username,
                password: msg.password,
            })
            .unwrap();
    }
}

fn handle_login_success_response(
    commands: Commands,
    mut messages: MessageReader<LoginSuccessResponseMessage>,
    mut results: MessageWriter<HandlerResult>,
) {
    for msg in messages.read() {
        let acc: MapleAccount = MapleAccount::from((msg.acc_model, msg.acc_id));
        commands.spawn((acc));
        let Ok(credentials_packet) =
            codec::login::builder::build_credentials_handler_successful_login_packet(acc)
        else {
            continue;
        };
        results.write(HandlerResult {
            client_id: msg.client_id,
            actions: vec![Action::Session(SessionAction::Send {
                packet: credentials_packet.finish(),
                scope: SessionScope::Local,
            })],
        });
    }
}

fn handle_login_failed_response(
    mut messages: MessageReader<LoginFailedResponseMessage>,
    mut results: MessageWriter<HandlerResult>,
) {
    for msg in messages.read() {
        let Ok(login_failed_packet) =
            codec::login::builder::build_failed_login_packet(msg.status as i16);
        results.write(HandlerResult {
            client_id: msg.client_id,
            actions: vec![Action::Session(SessionAction::Send {
                login_failed_packet.finish(),
                scope: SessionScope::Local,
            })],
        });
    }
}
