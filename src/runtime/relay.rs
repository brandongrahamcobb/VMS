use crate::config::settings;
use crate::net::error::NetworkError::UnsupportedOpcodeError;
use crate::net::packet::core::Packet;
use crate::net::packet::handler::action::Action;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::handler::{
    cc, char_select, check_char_name, create_char, credentials, handshake, list_chars, list_worlds,
    login_start, play, server_status, tos,
};
use crate::net::packet::io::{read::PacketReader, write::PacketWriter};
use crate::op::recv::RecvOpcode;
use crate::prelude::*;
use crate::runtime::error::RuntimeError;
use crate::runtime::state::SharedState;
use rand::{RngExt, rng};
use tokio::net::TcpStream;
use tracing::debug;

pub struct Runtime<T: RuntimeRelay> {
    reader: PacketReader,
    writer: PacketWriter,
    state: SharedState,
    relay: T,
}

impl<T: RuntimeRelay + Default + Send> Runtime<T> {
    pub async fn new(state: SharedState, stream: TcpStream) -> Result<Self, RuntimeError> {
        let (recv_iv, send_iv) = {
            let mut recv_iv = vec![0u8; 4];
            let mut send_iv = vec![0u8; 4];
            let mut rng = rng();
            rng.fill(&mut recv_iv[..]);
            rng.fill(&mut send_iv[..]);
            (recv_iv, send_iv)
        };
        let version = settings::get_version()?;
        let mut handshake = handshake::build_handshake_packet(&recv_iv, &send_iv, version).await?;
        let (read_half, write_half) = stream.into_split();
        let reader = PacketReader::new(read_half, &recv_iv)?;
        let mut writer = PacketWriter::new(write_half, &send_iv).await?;
        writer.send_unencrypted_packet(&mut handshake).await?;
        Ok(Self {
            reader,
            writer,
            relay: T::default(),
            state: state.clone(),
        })
    }

    pub async fn run(self: &mut Self) -> Result<(), RuntimeError> {
        loop {
            let packet = self.reader.read_packet().await?;
            let result = self.relay.handle_packet(self.state.clone(), packet).await?;
            self.relay.execute(&mut self.writer, result).await?;
        }
    }
}

#[allow(async_fn_in_trait)]
pub trait RuntimeRelay {
    async fn handle_packet(
        self: &mut Self,
        state: SharedState,
        packet: Packet,
    ) -> Result<HandlerResult<Action>, RuntimeError>;

    async fn execute(
        &mut self,
        writer: &mut PacketWriter,
        result: HandlerResult<Action>,
    ) -> Result<(), RuntimeError>;
}

#[derive(Default)]
pub struct Login;

impl RuntimeRelay for Login {
    async fn handle_packet(
        self: &mut Self,
        state: SharedState,
        packet: Packet,
    ) -> Result<HandlerResult<Action>, RuntimeError> {
        let opcode = packet.opcode();
        let en = RecvOpcode::from_i16(opcode).unwrap();
        debug!(
            "Received opcode in login: {} (0x{:02X}) ({:?}),",
            opcode, opcode, en
        );
        match opcode {
            x if x == RecvOpcode::RequestLogin as i16 => {
                let handler = credentials::CredentialsHandler::new();
                handler
                    .handle(state.clone(), packet)
                    .await
                    .map_err(RuntimeError::from)
            }
            x if x == RecvOpcode::AcceptTOS as i16 => {
                let handler = tos::TOSHandler::new();
                handler
                    .handle(state.clone(), packet)
                    .await
                    .map_err(RuntimeError::from)
            }
            x if x == RecvOpcode::LoginStarted as i16
                || x == RecvOpcode::ServerListRequest as i16 =>
            {
                let handler = list_worlds::WorldListHandler::new();
                handler
                    .handle(state.clone(), packet)
                    .await
                    .map_err(RuntimeError::from)
            }
            x if x == RecvOpcode::ServerStatusRequest as i16 => {
                let handler = server_status::ServerStatusHandler::new();
                handler
                    .handle(state.clone(), packet)
                    .await
                    .map_err(RuntimeError::from)
            }
            x if x == RecvOpcode::CharListRequest as i16 => {
                let handler = list_chars::CharListHandler::new();
                handler
                    .handle(state.clone(), packet)
                    .await
                    .map_err(RuntimeError::from)
            }
            x if x == RecvOpcode::CreateChar as i16 => {
                let handler = create_char::CreateCharacterHandler::new();
                handler
                    .handle(state.clone(), packet)
                    .await
                    .map_err(RuntimeError::from)
            }
            x if x == RecvOpcode::CheckCharName as i16 => {
                let handler = check_char_name::CheckCharNameHandler::new();
                handler
                    .handle(state.clone(), packet)
                    .await
                    .map_err(RuntimeError::from)
            }
            x if x == RecvOpcode::CharSelect as i16 => {
                let handler = char_select::CharacterSelectHandler::new();
                handler
                    .handle(state.clone(), packet)
                    .await
                    .map_err(RuntimeError::from)
            }
            _ => Err(RuntimeError::NetworkError(UnsupportedOpcodeError(opcode))),
        }
    }

    async fn execute(
        self: &mut Self,
        writer: &mut PacketWriter,
        result: HandlerResult<Action>,
    ) -> Result<(), RuntimeError> {
        let actions = result.actions;
        for action in actions {
            match action {
                Action::SendPacket { mut packet } => {
                    writer.send_encrypted_packet(&mut packet).await?
                }
            }
        }
        Ok(())
    }
}

#[derive(Default)]
pub struct World;

impl RuntimeRelay for World {
    async fn handle_packet(
        self: &mut Self,
        state: SharedState,
        packet: Packet,
    ) -> Result<HandlerResult<Action>, RuntimeError> {
        let opcode = packet.opcode();
        let en = RecvOpcode::from_i16(opcode).unwrap();
        debug!(
            "Received opcode in world: {} (0x{:02X}) ({:?})",
            opcode, opcode, en
        );
        match opcode {
            x if x == RecvOpcode::RequestLogin as i16 => {
                let handler = cc::ChangeChannelHandler::new();
                handler
                    .handle(state.clone(), packet)
                    .await
                    .map_err(RuntimeError::from)
            }
            x if x == RecvOpcode::PlayerLoggedIn as i16 => {
                let handler = play::PlayerLoggedInHandler::new();
                handler
                    .handle(state.clone(), packet)
                    .await
                    .map_err(RuntimeError::from)
            }
            _ => Err(RuntimeError::NetworkError(UnsupportedOpcodeError(opcode))),
        }
    }

    async fn execute(
        self: &mut Self,
        writer: &mut PacketWriter,
        result: HandlerResult<Action>,
    ) -> Result<(), RuntimeError> {
        let actions = result.actions;
        for action in actions {
            match action {
                Action::SendPacket { mut packet } => {
                    writer.send_encrypted_packet(&mut packet).await?
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_credentials_relay() {
        println!("Relay test is not implemented");
    }
}
