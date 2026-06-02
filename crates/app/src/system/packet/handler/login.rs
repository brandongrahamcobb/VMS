use crate::component::account::InAccount;
use crate::component::account::MapleAccount;
use crate::message::packet::login_started::LoginRequestMessage;
use crate::message::packet::login_started::LoginResponseMessage;
use crate::message::result::HandlerResult;
use crate::resource::custom_resource::ClientMap;
use crate::resource::custom_resource::CustomSender;
use crate::system::packet::build::codec;
use action::model::{Action, SessionAction};
use action::scope::SessionScope;
use base::account::FailedCode;
use bevy::ecs::message::MessageReader;
use bevy::ecs::message::MessageWriter;
use bevy::ecs::system::Commands;
use bevy::ecs::system::{Query, Res};
use ipc::asyncronous::db_command::DatabaseCommand;

fn handle_login_request(
    command_tx: CustomSender,
    client_map: Res<ClientMap>,
    accounts: Query<&MapleAccount>,
    mut messages: MessageReader<LoginRequestMessage>,
    mut results: MessageWriter<HandlerResult>,
) {
    for msg in messages.read() {
        let Some(&client_entity) = client_map.0.get(&msg.client_id) else {
            continue;
        };
        let already_logged_in = accounts.iter().find(|a| a.username == msg.username) else {
            continue;
        };
        if already_logged_in.is_some() {
            let Ok(mut login_failed_packet) =
                codec::login::builder::build_failed_login_packet(FailedCode::Playing as i16);
            results.write(HandlerResult {
                client_id: msg.client_id,
                actions: vec![Action::Session(SessionAction::Send {
                    packet: login_failed_packet.finish(),
                    scope: SessionScope::Local,
                })],
            });
        }
        command_tx
            .0
            .send(DatabaseCommand::RequestLogin {
                client_id: msg.client_id,
                username: msg.username,
                password: msg.password,
            })
            .unwrap();
    }
}

fn handle_login_success_response(
    commands: Commands,
    client_map: Res<ClientMap>,
    mut messages: MessageReader<LoginSuccessResponseMessage>,
    mut results: MessageWriter<HandlerResult>,
) {
    for msg in messages.read() {
        let Some(&client_entity) = client_map.0.get(&msg.client_id) else {
            continue;
        };
        let acc: MapleAccount = MapleAccount::from((msg.acc_model, msg.acc_id));
        let acc_entity = commands.spawn(acc).id();
        commands.entity(client_entity).insert(InAccount(acc_entity));

        let Ok(mut credentials_packet) = codec::login::builder::build_successful_login_packet(&acc)
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
        let Ok(mut login_failed_packet) =
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
