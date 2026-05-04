use crate::config::settings;
use crate::db::error::DatabaseError;
use crate::models::account;
use crate::net::packet::handler::action::{ChannelAction, LoginAction};
use crate::net::packet::handler::cc::handler::ChangeChannelHandler;
use crate::net::packet::handler::change_keymap::handler::ChangeKeymapHandler;
use crate::net::packet::handler::check_char_name::handler::CheckCharNameHandler;
use crate::net::packet::handler::close_attack::handler::CloseAttackHandler;
use crate::net::packet::handler::create_char::handler::CreateCharHandler;
use crate::net::packet::handler::credentials::handler::CredentialsHandler;
use crate::net::packet::handler::delete_char::handler::DeleteCharHandler;
use crate::net::packet::handler::enter_cash_shop::handler::EnterCashShopHandler;
use crate::net::packet::handler::list_chars::handler::ListCharsHandler;
use crate::net::packet::handler::list_worlds::handler::ListWorldsHandler;
use crate::net::packet::handler::login_start::handler::LoginStartHandler;
use crate::net::packet::handler::move_player::handler::MovePlayerHandler;
use crate::net::packet::handler::party_search::handler::PartySearchHandler;
use crate::net::packet::handler::player_logged_in::handler::PlayerLoggedInHandler;
use crate::net::packet::handler::player_map_transfer::handler::PlayerMapTransferHandler;
use crate::net::packet::handler::register_pic::handler::RegisterPicHandler;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::handler::select_char::handler::SelectCharHandler;
use crate::net::packet::handler::select_char_with_pic::handler::SelectCharWithPicHandler;
use crate::net::packet::handler::server_status::handler::ServerStatusHandler;
use crate::net::packet::handler::tos::handler::TOSHandler;
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
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::sync::mpsc::UnboundedSender;
use tracing::debug;

pub struct Runtime<T: RuntimeRelay> {
    pkt_reader: PacketReader,
    pkt_writer: PacketWriter,
    state: SharedState,
    relay: T,
    rx: UnboundedReceiver<Packet>,
    tx: UnboundedSender<Packet>,
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
        let mut packet = Packet::new_empty();
        let packet = packet
            .build_handshake_packet(&recv_iv, &send_iv, &version)
            .await?
            .finish();
        let (read_half, write_half) = stream.into_split();
        let pkt_reader = PacketReader::new(read_half, &recv_iv)?;
        let mut pkt_writer = PacketWriter::new(write_half, &send_iv).await?;
        pkt_writer.send_unencrypted_packet(&packet).await?;
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        Ok(Self {
            pkt_reader,
            pkt_writer,
            rx,
            relay: T::default(),
            state: state.clone(),
            tx,
        })
    }

    pub async fn run(&mut self) -> Result<(), RuntimeError> {
        loop {
            tokio::select! {
                packet = self.pkt_reader.read_packet() => {
                    let packet = packet?;
                    let result = self.relay
                        .handle_packet(self.state.clone(), packet)
                        .await?;
                    match self.relay.execute(self.state.clone(), &mut self.pkt_writer, result, self.tx.clone()).await? {
                        ControlFlow::Break(_) => break Ok(()),
                        _ => {}
                    }
                }
                packet = self.rx.recv() => {
                    match packet {
                        Some(mut packet) => {
                            self.pkt_writer.send_encrypted_packet(&mut packet).await?;
                        }
                        None => break Ok(()),
                    }
                }
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
        packet: Packet,
    ) -> Result<HandlerResult<Self::HandlerAction>, RuntimeError>;

    async fn execute(
        &mut self,
        state: SharedState,
        pkt_writer: &mut PacketWriter,
        result: HandlerResult<Self::HandlerAction>,
        tx: UnboundedSender<Packet>,
    ) -> Result<ControlFlow<()>, RuntimeError>;
}

#[derive(Default)]
pub struct LoginRelay {
    session_id: Option<i32>,
}

impl RuntimeRelay for LoginRelay {
    type HandlerAction = LoginAction;

    async fn handle_packet(
        &mut self,
        state: SharedState,
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
        if self.session_id.is_none() {
            return match opcode {
                x if x == RecvOpcode::RequestLogin as i16 => {
                    let handler = CredentialsHandler::new();
                    handler
                        .handle(state.clone(), packet)
                        .await
                        .map_err(RuntimeError::from)
                }
                x if x == RecvOpcode::LoginStarted as i16 => {
                    let handler = LoginStartHandler::new();
                    handler
                        .handle(state.clone(), packet)
                        .await
                        .map_err(RuntimeError::from)
                }
                _ => Err(RuntimeError::UnsupportedOpcodeError(
                    opcode,
                    String::from("expected before authentication"),
                )),
            };
        }
        let session_id = self.session_id.unwrap();
        let session = {
            let state = state.lock().await;
            state
                .sessions
                .get(session_id)
                .ok_or(SessionError::NotFound(session_id))
                .map_err(RuntimeError::from)?
        };
        match opcode {
            x if x == RecvOpcode::LoginStarted as i16 => {
                let handler = LoginStartHandler::new();
                handler
                    .handle(state.clone(), packet)
                    .await
                    .map_err(RuntimeError::from)
            }
            x if x == RecvOpcode::AcceptTOS as i16 => {
                let handler = TOSHandler::new();
                handler
                    .handle(state.clone(), session, packet)
                    .await
                    .map_err(RuntimeError::from)
            }
            x if x == RecvOpcode::ServerListRequest as i16 => {
                let handler = ListWorldsHandler::new();
                handler
                    .handle(state.clone(), session, packet)
                    .await
                    .map_err(RuntimeError::from)
            }
            x if x == RecvOpcode::ServerStatusRequest as i16 => {
                let handler = ServerStatusHandler::new();
                handler
                    .handle(state.clone(), session, packet)
                    .await
                    .map_err(RuntimeError::from)
            }
            x if x == RecvOpcode::CharListRequest as i16 => {
                let handler = ListCharsHandler::new();
                handler
                    .handle(state.clone(), session, packet)
                    .await
                    .map_err(RuntimeError::from)
            }
            x if x == RecvOpcode::CreateChar as i16 => {
                let handler = CreateCharHandler::new();
                handler
                    .handle(state.clone(), session, packet)
                    .await
                    .map_err(RuntimeError::from)
            }
            x if x == RecvOpcode::CheckCharName as i16 => {
                let handler = CheckCharNameHandler::new();
                handler
                    .handle(state.clone(), session, packet)
                    .await
                    .map_err(RuntimeError::from)
            }
            x if x == RecvOpcode::DeleteChar as i16 => {
                let handler = DeleteCharHandler::new();
                handler
                    .handle(state.clone(), session, packet)
                    .await
                    .map_err(RuntimeError::from)
            }
            x if x == RecvOpcode::CharSelect as i16 => {
                let handler = SelectCharHandler::new();
                handler
                    .handle(state.clone(), session, packet)
                    .await
                    .map_err(RuntimeError::from)
            }
            x if x == RecvOpcode::RegisterPic as i16 => {
                let handler = RegisterPicHandler::new();
                handler
                    .handle(state.clone(), session, packet)
                    .await
                    .map_err(RuntimeError::from)
            }
            x if x == RecvOpcode::CharSelectWithPic as i16 => {
                let handler = SelectCharWithPicHandler::new();
                handler
                    .handle(state.clone(), session, packet)
                    .await
                    .map_err(RuntimeError::from)
            }
            _ => Err(RuntimeError::UnsupportedOpcodeError(
                opcode,
                String::from("expected after authentication"),
            )),
        }
    }

    async fn execute(
        &mut self,
        state: SharedState,
        pkt_writer: &mut PacketWriter,
        result: HandlerResult<LoginAction>,
        tx: UnboundedSender<Packet>,
    ) -> Result<ControlFlow<()>, RuntimeError> {
        let actions = result.actions;
        for action in actions {
            match action {
                LoginAction::CreateSession { mut acc, hwid } => {
                    let session_id = {
                        let state = state.lock().await;
                        state.sessions.insert(Session {
                            id: 0,
                            acc_id: acc.id,
                            authenticated: true,
                            hwid: hwid,
                            world_id: None,
                            channel_id: None,
                            map_id: None,
                            char_id: None,
                            tx: tx.clone(),
                            playing: true,
                        })
                    };
                    acc.session_id = Some(session_id as i32);
                    account::query::update(state.clone(), &acc)
                        .await
                        .map_err(DatabaseError::from)
                        .map_err(RuntimeError::from)?;
                    self.session_id = Some(session_id);
                }
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
pub struct ChannelRelay {
    session_id: Option<i32>,
}

impl RuntimeRelay for ChannelRelay {
    type HandlerAction = ChannelAction;

    async fn handle_packet(
        &mut self,
        state: SharedState,
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
        if self.session_id.is_none() {
            return match opcode {
                x if x == RecvOpcode::PlayerLoggedIn as i16 => {
                    let handler = PlayerLoggedInHandler::new();
                    handler
                        .handle(state.clone(), packet)
                        .await
                        .map_err(RuntimeError::from)
                }
                _ => Err(RuntimeError::UnsupportedOpcodeError(
                    opcode,
                    String::from("expected in channel"),
                )),
            };
        }
        let session_id = self.session_id.unwrap();
        let session = {
            let state = state.lock().await;
            state
                .sessions
                .get(session_id)
                .ok_or(SessionError::NotFound(session_id))
                .map_err(RuntimeError::from)?
        };
        match opcode {
            x if x == RecvOpcode::ChangeChannel as i16 => {
                let handler = ChangeChannelHandler::new();
                handler
                    .handle(state.clone(), session, packet)
                    .await
                    .map_err(RuntimeError::from)
            }
            x if x == RecvOpcode::PartySearch as i16 => {
                let handler = PartySearchHandler::new();
                handler
                    .handle(state.clone(), session, packet)
                    .await
                    .map_err(RuntimeError::from)
            }
            x if x == RecvOpcode::PlayerMapTransfer as i16 => {
                let handler = PlayerMapTransferHandler::new();
                handler
                    .handle(state.clone(), session, packet)
                    .await
                    .map_err(RuntimeError::from)
            }
            x if x == RecvOpcode::PlayerMove as i16 => {
                let handler = MovePlayerHandler::new();
                handler
                    .handle(state.clone(), session, packet)
                    .await
                    .map_err(RuntimeError::from)
            }
            x if x == RecvOpcode::EnterCashShop as i16 => {
                let handler = EnterCashShopHandler::new();
                handler
                    .handle(state.clone(), session, packet)
                    .await
                    .map_err(RuntimeError::from)
            }
            x if x == RecvOpcode::ChangeKeymap as i16 => {
                let handler = ChangeKeymapHandler::new();
                handler
                    .handle(state.clone(), session, packet)
                    .await
                    .map_err(RuntimeError::from)
            }
            x if x == RecvOpcode::CloseAttack as i16 => {
                let handler = CloseAttackHandler::new();
                handler
                    .handle(state.clone(), session, packet)
                    .await
                    .map_err(RuntimeError::from)
            }
            _ => Err(RuntimeError::UnsupportedOpcodeError(
                opcode,
                String::from("expected in channel"),
            )),
        }
    }

    async fn execute(
        &mut self,
        state: SharedState,
        pkt_writer: &mut PacketWriter,
        result: HandlerResult<ChannelAction>,
        _tx: UnboundedSender<Packet>,
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
                ChannelAction::BroadcastPacket { session, packet } => {
                    let world = session
                        .world_id
                        .ok_or(SessionError::MissingField(session.id))
                        .map_err(RuntimeError::from)?;
                    let channel = session
                        .channel_id
                        .ok_or(SessionError::MissingField(session.id))
                        .map_err(RuntimeError::from)?;
                    let map = session
                        .map_id
                        .ok_or(SessionError::MissingField(session.id))
                        .map_err(RuntimeError::from)?;
                    let state = state.lock().await;
                    if let Some(session_ids) = state.map_index.get(&(world, channel, map)) {
                        for id in session_ids {
                            if *id == session.id {
                                continue;
                            }
                            if let Some(target) = state.sessions.get(*id) {
                                let _ = target.tx.send(packet.clone());
                            }
                        }
                    }
                }
                ChannelAction::Connect { session_id } => {
                    self.session_id = Some(session_id);
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
