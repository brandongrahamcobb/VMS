use crate::config::settings;
use crate::net::packet::handler::action::{ChannelAction, LoginAction};
use crate::net::packet::handler::handlers::ChannelHandler;
use crate::net::packet::handler::handlers::LoginHandler;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::handler::{
    cc, check_char_name, create_char, credentials, delete_char, enter_cash_shop, handshake, list_chars, list_worlds, login_start, move_player, party_search, play, player_map_transfer, register_pic, select_char, select_char_with_pic, server_status, tos
};
use crate::net::packet::io::{read::PacketReader, write::PacketWriter};
use crate::net::packet::packet::Packet;
use crate::op::recv::RecvOpcode;
use crate::prelude::*;
use crate::runtime::error::{RuntimeError, SessionError};
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;
use core::ops::ControlFlow;
use rand::{RngExt, rng};
use tokio::net::TcpStream;
use tracing::debug;

pub struct Runtime<T: RuntimeRelay> {
    pkt_reader: PacketReader,
    pkt_writer: PacketWriter,
    state: SharedState,
    relay: T,
    session_id: i32,
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
        let pkt_reader = PacketReader::new(read_half, &recv_iv)?;
        let mut pkt_writer = PacketWriter::new(write_half, &send_iv).await?;
        pkt_writer.send_unencrypted_packet(&mut handshake).await?;
        let session_id = {
            let state = state.lock().await;
            state.sessions.insert(Session {
                id: 0,
                acc_id: None,
                authenticated: false,
                hwid: None,
                valid_pic: false,
            })
        };
        Ok(Self {
            pkt_reader,
            pkt_writer,
            relay: T::default(),
            state: state.clone(),
            session_id,
        })
    }

    pub async fn run(&mut self) -> Result<(), RuntimeError> {
        loop {
            let session = {
                let state = self.state.lock().await;
                state
                    .sessions
                    .get(self.session_id as u32)
                    .ok_or(SessionError::NotFound(self.session_id))
                    .map_err(RuntimeError::from)?
            };
            let packet = self.pkt_reader.read_packet().await?;
            let result = self
                .relay
                .handle_packet(self.state.clone(), session, packet)
                .await?;
            match self.relay.execute(&mut self.pkt_writer, result).await? {
                ControlFlow::Break(_) => break Ok(()),
                _ => continue,
            }
        }
    }
}

#[allow(async_fn_in_trait)]
pub trait RuntimeRelay {
    type HandlerAction;

    async fn handle_packet(
        &mut self,
        state: SharedState,
        session: Session,
        packet: Packet,
    ) -> Result<HandlerResult<Self::HandlerAction>, RuntimeError>;

    async fn execute(
        &mut self,
        pkt_writer: &mut PacketWriter,
        result: HandlerResult<Self::HandlerAction>,
    ) -> Result<ControlFlow<()>, RuntimeError>;
}

#[derive(Default)]
pub struct LoginRelay;

impl RuntimeRelay for LoginRelay {
    type HandlerAction = LoginAction;

    async fn handle_packet(
        &mut self,
        state: SharedState,
        session: Session,
        packet: Packet,
    ) -> Result<HandlerResult<LoginAction>, RuntimeError> {
        let opcode = packet.opcode();
        let en = RecvOpcode::from_i16(opcode).ok_or(RuntimeError::UnsupportedOpcodeError(
            opcode,
            String::from("not expected during authentication"),
        ));
        debug!(
            "Received opcode in login: {} (0x{:02X}) ({:?}),",
            opcode, opcode, en
        );
        let handler = if !session.authenticated {
            match opcode {
                x if x == RecvOpcode::RequestLogin as i16 => Ok(LoginHandler::Credentials(
                    credentials::handler::CredentialsHandler::new(),
                )),
                x if x == RecvOpcode::LoginStarted as i16 => Ok(LoginHandler::LoginStarted(
                    login_start::handler::LoginStartHandler::new(),
                )),
                _ => Err(RuntimeError::UnsupportedOpcodeError(
                    opcode,
                    String::from("expected before authentication"),
                )),
            }
        } else {
            match opcode {
                x if x == RecvOpcode::LoginStarted as i16 => Ok(LoginHandler::LoginStarted(
                    login_start::handler::LoginStartHandler::new(),
                )),
                x if x == RecvOpcode::AcceptTOS as i16 => {
                    Ok(LoginHandler::TOS(tos::handler::TOSHandler::new()))
                }
                x if x == RecvOpcode::ServerListRequest as i16 => Ok(LoginHandler::ListWorlds(
                    list_worlds::handler::WorldListHandler::new(),
                )),
                x if x == RecvOpcode::ServerStatusRequest as i16 => Ok(LoginHandler::ServerStatus(
                    server_status::handler::ServerStatusHandler::new(),
                )),
                x if x == RecvOpcode::CharListRequest as i16 => Ok(LoginHandler::ListChars(
                    list_chars::handler::CharListHandler::new(),
                )),
                x if x == RecvOpcode::CreateChar as i16 => Ok(LoginHandler::CreateChar(
                    create_char::handler::CreateCharacterHandler::new(),
                )),
                x if x == RecvOpcode::CheckCharName as i16 => Ok(LoginHandler::CheckCharName(
                    check_char_name::handler::CheckCharNameHandler::new(),
                )),
                x if x == RecvOpcode::DeleteChar as i16 => Ok(LoginHandler::DeleteChar(
                    delete_char::handler::DeleteCharacterHandler::new(),
                )),
                x if x == RecvOpcode::CharSelect as i16 => Ok(LoginHandler::CharSelect(
                    select_char::handler::CharacterSelectHandler::new(),
                )),
                x if x == RecvOpcode::RegisterPic as i16 => {
                    Ok(LoginHandler::RegisterPic(register_pic::handler::RegisterPicHandler::new()))
                }
                x if x == RecvOpcode::CharSelectWithPic as i16 => Ok(
                    LoginHandler::CharSelectWithPic(select_char_with_pic::handler::SelectCharWithPicHandler::new()),
                ),
                _ => Err(RuntimeError::UnsupportedOpcodeError(
                    opcode,
                    String::from("expected after authentication"),
                )),
            }
        };
        handler?
            .handle(state.clone(), session, packet)
            .await
            .map_err(RuntimeError::from)
    }

    async fn execute(
        &mut self,
        pkt_writer: &mut PacketWriter,
        result: HandlerResult<LoginAction>,
    ) -> Result<ControlFlow<()>, RuntimeError> {
        let actions = result.actions;
        for action in actions {
            match action {
                LoginAction::Simple => (),
                LoginAction::SendPacket { mut packet } => {
                    pkt_writer.send_encrypted_packet(&mut packet).await?
                }
                LoginAction::CloseConnection => return Ok(ControlFlow::Break(())),
            }
        }
        return Ok(ControlFlow::Continue(()));
    }
}

#[derive(Default)]
pub struct ChannelRelay;

impl RuntimeRelay for ChannelRelay {
    type HandlerAction = ChannelAction;

    async fn handle_packet(
        &mut self,
        state: SharedState,
        session: Session,
        packet: Packet,
    ) -> Result<HandlerResult<ChannelAction>, RuntimeError> {
        let opcode = packet.opcode();
        let en = RecvOpcode::from_i16(opcode).ok_or(RuntimeError::UnsupportedOpcodeError(
            opcode,
            String::from("not expected in channel"),
        ));
        debug!(
            "Received opcode in channel: {} (0x{:02X}) ({:?})",
            opcode, opcode, en
        );
        let handler = match opcode {
            x if x == RecvOpcode::ChangeChannel as i16 => Ok(ChannelHandler::ChangeChannel(
                cc::handler::ChangeChannelHandler::new(),
            )),
            x if x == RecvOpcode::PlayerLoggedIn as i16 => Ok(ChannelHandler::PlayerLoggedIn(
                play::handler::PlayerLoggedInHandler::new(),
            )),
            x if x == RecvOpcode::PartySearch as i16 => Ok(ChannelHandler::PartySearch(
                party_search::handler::PartySearchHandler::new(),
            )),
            x if x == RecvOpcode::PlayerMapTransfer as i16 => {
                Ok(ChannelHandler::PlayerMapTransfer(
                    player_map_transfer::handler::PlayerMapTransferHandler::new(),
                ))
            }
            x if x == RecvOpcode::PlayerMove as i16 => Ok(ChannelHandler::MovePlayer(
                move_player::handler::MovePlayerHandler::new(),
            )),
            x if x == RecvOpcode::EnterCashShop as i16 => Ok(ChannelHandler::EnterCashShop(
                enter_cash_shop::handler::EnterCashShopHandler::new(),
            )),
            _ => Err(RuntimeError::UnsupportedOpcodeError(
                opcode,
                String::from("expected in channel"),
            )),
        };
        handler?
            .handle(state.clone(), session, packet)
            .await
            .map_err(RuntimeError::from)
    }

    async fn execute(
        &mut self,
        pkt_writer: &mut PacketWriter,
        result: HandlerResult<ChannelAction>,
    ) -> Result<ControlFlow<()>, RuntimeError> {
        let actions = result.actions;
        for action in actions {
            match action {
                ChannelAction::Simple => (),
                ChannelAction::SendPacket { mut packet } => {
                    pkt_writer.send_encrypted_packet(&mut packet).await?
                }
                ChannelAction::FieldMove { movement_bytes } => {
                    debug!("{:?}", movement_bytes);
                    () // not implemented
                }
            }
        }
        return Ok(ControlFlow::Continue(()));
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_credentials_relay() {
        println!("Relay test is not implemented");
    }
}
