use crate::config::settings;
use crate::models::account;
use crate::net::action;
use crate::net::action::model::{LoginAction, PlayerAction};
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
use crate::net::packet::model::Packet;
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
            let mut recv_iv = [0u8; 4];
            let mut send_iv = [0u8; 4];
            let mut rng = rng();
            rng.fill(&mut recv_iv[..]);
            rng.fill(&mut send_iv[..]);
            (recv_iv, send_iv)
        };
        let version = settings::get_version()?;
        let packet: Packet = Packet::new_empty()
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
            state,
            tx,
        })
    }

    pub async fn run(&mut self) -> Result<(), RuntimeError> {
        loop {
            tokio::select! {
                packet = self.pkt_reader.read_packet() => {
                    let packet = packet?;
                    let result = self.relay
                        .handle_packet(&self.state, &packet)
                        .await?;
                    match self.relay.execute(&self.state, &result, &self.tx).await? {
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
        state: &SharedState,
        packet: &Packet,
    ) -> Result<HandlerResult<Self::HandlerAction>, RuntimeError>;

    async fn execute(
        &mut self,
        state: &SharedState,
        result: &HandlerResult<Self::HandlerAction>,
        tx: &UnboundedSender<Packet>,
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
        state: &SharedState,
        packet: &Packet,
    ) -> Result<HandlerResult<LoginAction>, RuntimeError> {
        let op = packet.opcode();
        let en = RecvOpcode::from_i16(op).ok_or(RuntimeError::UnsupportedOpcodeError(
            op,
            String::from("not expected during authentication"),
        ));
        debug!(
            "Received opcode in login: {} (0x{:02X}) ({:?}),",
            op, op, en
        );
        if self.session_id.is_none() {
            return match op {
                x if x == RecvOpcode::RequestLogin as i16 => {
                    let handler = CredentialsHandler::new();
                    handler
                        .handle(state, packet)
                        .await
                        .map_err(RuntimeError::from)
                }
                x if x == RecvOpcode::LoginStarted as i16 => {
                    let handler = LoginStartHandler::new();
                    handler
                        .handle(state, packet)
                        .await
                        .map_err(RuntimeError::from)
                }
                _ => Err(RuntimeError::UnsupportedOpcodeError(
                    op,
                    String::from("expected before authentication"),
                )),
            };
        }
        let session_id = self.session_id.unwrap();
        let session = {
            let state = state.lock().await;
            state
                .sessions
                .get(&session_id)
                .ok_or(SessionError::NotFound(session_id))
                .map_err(RuntimeError::from)?
        };
        match op {
            x if x == RecvOpcode::AcceptTOS as i16 => {
                let handler = TOSHandler::new();
                handler
                    .handle(state, &session, packet)
                    .await
                    .map_err(RuntimeError::from)
            }
            x if x == RecvOpcode::ServerListRequest as i16 => {
                let handler = ListWorldsHandler::new();
                handler
                    .handle(state, &session, packet)
                    .await
                    .map_err(RuntimeError::from)
            }
            x if x == RecvOpcode::ServerStatusRequest as i16 => {
                let handler = ServerStatusHandler::new();
                handler
                    .handle(state, &session, packet)
                    .await
                    .map_err(RuntimeError::from)
            }
            x if x == RecvOpcode::CharListRequest as i16 => {
                let handler = ListCharsHandler::new();
                handler
                    .handle(state, &session, packet)
                    .await
                    .map_err(RuntimeError::from)
            }
            x if x == RecvOpcode::CreateChar as i16 => {
                let handler = CreateCharHandler::new();
                handler
                    .handle(state, &session, packet)
                    .await
                    .map_err(RuntimeError::from)
            }
            x if x == RecvOpcode::CheckCharName as i16 => {
                let handler = CheckCharNameHandler::new();
                handler
                    .handle(state, &session, packet)
                    .await
                    .map_err(RuntimeError::from)
            }
            x if x == RecvOpcode::DeleteChar as i16 => {
                let handler = DeleteCharHandler::new();
                handler
                    .handle(state, &session, packet)
                    .await
                    .map_err(RuntimeError::from)
            }
            x if x == RecvOpcode::CharSelect as i16 => {
                let handler = SelectCharHandler::new();
                handler
                    .handle(state, &session, packet)
                    .await
                    .map_err(RuntimeError::from)
            }
            x if x == RecvOpcode::RegisterPic as i16 => {
                let handler = RegisterPicHandler::new();
                handler
                    .handle(state, &session, packet)
                    .await
                    .map_err(RuntimeError::from)
            }
            x if x == RecvOpcode::CharSelectWithPic as i16 => {
                let handler = SelectCharWithPicHandler::new();
                handler
                    .handle(state, &session, packet)
                    .await
                    .map_err(RuntimeError::from)
            }
            _ => Err(RuntimeError::UnsupportedOpcodeError(
                op,
                String::from("expected after authentication"),
            )),
        }
    }

    async fn execute(
        &mut self,
        state: &SharedState,
        result: &HandlerResult<LoginAction>,
        tx: &UnboundedSender<Packet>,
    ) -> Result<ControlFlow<()>, RuntimeError> {
        let model = &result.model;
        for action in model {
            match action {
                LoginAction::CreateSession { acc_id, hwid } => {
                    let session_id = {
                        let state = state.lock().await;
                        state.sessions.insert(Session {
                            id: 0,
                            acc_id: acc_id.clone(),
                            authenticated: true,
                            hwid: hwid.clone(),
                            world_id: None,
                            channel_id: None,
                            map_id: None,
                            char_id: None,
                            tx: tx.clone(),
                            playing: true,
                        })
                    };
                    let mut acc = account::query::get_account_by_id(state, &acc_id).await?;
                    acc.session_id = Some(session_id);
                    account::query::update(state, &acc).await?;
                    self.session_id = Some(session_id);
                }
                LoginAction::Simple => (),
                LoginAction::SendLocalPacket { packet } => {
                    tx.send(packet.clone())?;
                }
            }
        }
        return Ok(ControlFlow::Continue(()));
    }
}

#[derive(Default)]
pub struct PlayerRelay {
    session_id: Option<i32>,
}

impl RuntimeRelay for PlayerRelay {
    type HandlerAction = PlayerAction;

    async fn handle_packet(
        &mut self,
        state: &SharedState,
        packet: &Packet,
    ) -> Result<HandlerResult<PlayerAction>, RuntimeError> {
        let op = packet.opcode();
        let en = RecvOpcode::from_i16(op).ok_or(RuntimeError::UnsupportedOpcodeError(
            op,
            String::from("not expected in channel"),
        ));
        debug!(
            "Received opcode in channel: {} (0x{:02X}) ({:?})",
            op, op, en
        );
        if self.session_id.is_none() {
            return match op {
                x if x == RecvOpcode::PlayerLoggedIn as i16 => {
                    let handler = PlayerLoggedInHandler::new();
                    handler
                        .handle(state, packet)
                        .await
                        .map_err(RuntimeError::from)
                }
                _ => Err(RuntimeError::UnsupportedOpcodeError(
                    op,
                    String::from("expected in channel"),
                )),
            };
        }
        let session_id = self.session_id.unwrap();
        let session = {
            let state = state.lock().await;
            state
                .sessions
                .get(&session_id)
                .ok_or(SessionError::NotFound(session_id))
                .map_err(RuntimeError::from)?
        };
        match op {
            x if x == RecvOpcode::ChangeChannel as i16 => {
                let handler = ChangeChannelHandler::new();
                handler
                    .handle(state, &session, packet)
                    .await
                    .map_err(RuntimeError::from)
            }
            x if x == RecvOpcode::PartySearch as i16 => {
                let handler = PartySearchHandler::new();
                handler
                    .handle(state, &session, packet)
                    .await
                    .map_err(RuntimeError::from)
            }
            x if x == RecvOpcode::PlayerMapTransfer as i16 => {
                let handler = PlayerMapTransferHandler::new();
                handler
                    .handle(state, &session, packet)
                    .await
                    .map_err(RuntimeError::from)
            }
            x if x == RecvOpcode::PlayerMove as i16 => {
                let handler = MovePlayerHandler::new();
                handler
                    .handle(state, &session, packet)
                    .await
                    .map_err(RuntimeError::from)
            }
            x if x == RecvOpcode::EnterCashShop as i16 => {
                let handler = EnterCashShopHandler::new();
                handler
                    .handle(state, &session, packet)
                    .await
                    .map_err(RuntimeError::from)
            }
            x if x == RecvOpcode::ChangeKeymap as i16 => {
                let handler = ChangeKeymapHandler::new();
                handler
                    .handle(state, &session, packet)
                    .await
                    .map_err(RuntimeError::from)
            }
            x if x == RecvOpcode::CloseAttack as i16 => {
                let handler = CloseAttackHandler::new();
                handler
                    .handle(state, &session, packet)
                    .await
                    .map_err(RuntimeError::from)
            }
            _ => Err(RuntimeError::UnsupportedOpcodeError(
                op,
                String::from("expected in channel"),
            )),
        }
    }

    async fn execute(
        &mut self,
        state: &SharedState,
        result: &HandlerResult<PlayerAction>,
        tx: &UnboundedSender<Packet>,
    ) -> Result<ControlFlow<()>, RuntimeError> {
        let model = &result.model;
        for action in model {
            match action {
                PlayerAction::Simple => (),
                PlayerAction::Connect { session_id } => {
                    self.session_id = Some(*session_id);
                    {
                        let state = state.lock().await;
                        state.sessions.update(*session_id, |s| s.tx = tx.clone());
                    }
                }
                PlayerAction::SendLocalPacket { packet } => {
                    tx.send(packet.clone())?;
                }
                PlayerAction::FieldMove { session, packet } => {
                    action::broadcast::transmit_action(state, &session, &packet).await?;
                }
                PlayerAction::EnterMap { session, packet } => {
                    let world_id = session
                        .world_id
                        .ok_or(SessionError::MissingField(session.id))?;
                    let channel_id = session
                        .channel_id
                        .ok_or(SessionError::MissingField(session.id))?;
                    let map_id = session
                        .map_id
                        .ok_or(SessionError::MissingField(session.id))?;
                    {
                        let mut state = state.lock().await;
                        state.set_location(&session.id, &world_id, &channel_id, &map_id);
                    }
                    action::broadcast::transmit_action(state, &session, &packet).await?;
                    action::enter_map::transmit_action(state, &session, &tx).await?;
                }
                PlayerAction::ExitMap { session, packet } => {
                    action::broadcast::transmit_action(state, &session, &packet).await?;
                    action::exit_map::transmit_action(state, &session, &tx).await?;
                }
            }
        }
        return Ok(ControlFlow::Continue(()));
    }
}
