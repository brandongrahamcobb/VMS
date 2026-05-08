use crate::net::action::{Action, SetAction};
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
use crate::net::packet::handler::tos::handler::TosHandler;
use crate::net::packet::io::{read::PacketReader, write::PacketWriter};
use crate::net::packet::model::Packet;
use crate::op::recv::RecvOpcode;
use crate::prelude::*;
use crate::runtime::error::{RuntimeError, SessionError};
use crate::runtime::scope::Scope;
use crate::runtime::state::SharedState;
use core::ops::ControlFlow;
use rand::{RngExt, rng};
use tokio::net::TcpStream;
use tokio::sync::mpsc::UnboundedReceiver;
use tracing::debug;

pub struct Runtime<T: RuntimeRelay> {
    pkt_reader: PacketReader,
    pkt_writer: PacketWriter,
    state: SharedState,
    relay: T,
    rx: UnboundedReceiver<Packet>,
}

impl<T: RuntimeRelay + Send> Runtime<T> {
    pub async fn new(
        state: SharedState,
        stream: TcpStream,
        session_id: i32,
        rx: UnboundedReceiver<Packet>,
    ) -> Result<Self, RuntimeError> {
        let (recv_iv, send_iv) = {
            let mut recv_iv = [0u8; 4];
            let mut send_iv = [0u8; 4];
            let mut rng = rng();
            rng.fill(&mut recv_iv[..]);
            rng.fill(&mut send_iv[..]);
            (recv_iv, send_iv)
        };
        let packet: Packet = Packet::new_empty()
            .build_handshake_packet(recv_iv, send_iv)
            .await?
            .finish();
        let (read_half, write_half) = stream.into_split();
        let pkt_reader = PacketReader::new(read_half, &recv_iv)?;
        let mut pkt_writer = PacketWriter::new(write_half, &send_iv).await?;
        pkt_writer.send_unencrypted_packet(&packet).await?;
        Ok(Self {
            pkt_reader,
            pkt_writer,
            rx,
            relay: T::new(session_id).await?,
            state,
        })
    }

    pub async fn run(&mut self) -> Result<Option<i32>, RuntimeError> {
        loop {
            tokio::select! {
                packet = self.pkt_reader.read_packet() => {
                    let packet = packet?;
                    let result = self.relay
                        .handle_packet(&self.state, &packet)
                        .await?;
                    match self.relay.execute(&self.state, result).await? {
                        ControlFlow::Break(_) => break Ok(Some(self.relay.session_id())),
                        _ => {}
                    }
                }
                packet = self.rx.recv() => {
                    match packet {
                        Some(mut packet) => {
                            self.pkt_writer.send_encrypted_packet(&mut packet).await?;
                        }
                        None => break Ok(None),
                    }
                }
            }
        }
    }
}

#[allow(async_fn_in_trait)]
pub trait RuntimeRelay: Sized {
    async fn new(session_id: i32) -> Result<Self, RuntimeError>;

    fn session_id(&self) -> i32;

    async fn handle_packet(
        &mut self,
        state: &SharedState,
        packet: &Packet,
    ) -> Result<HandlerResult, RuntimeError>;

    async fn execute(
        &mut self,
        state: &SharedState,
        result: HandlerResult,
    ) -> Result<ControlFlow<()>, RuntimeError> {
        let session = {
            let state = state.lock().await;
            state
                .sessions
                .get(self.session_id())
                .ok_or(SessionError::NotFound(self.session_id()))?
        };
        let model = &result.model;
        for action in model {
            match action {
                Action::Send { packet, scope } => match scope {
                    Scope::Local => {
                        let _ = session.tx.send(packet.clone());
                    }
                    Scope::Map => {
                        let state = state.lock().await;
                        for s in state.sessions.iter() {
                            if s.get_map()?.model.id == session.get_map()?.model.id
                                && s.id != session.id
                            {
                                s.tx.send(packet.clone())?;
                            }
                        }
                    }
                    Scope::Channel => {
                        let state = state.lock().await;
                        for s in state.sessions.iter() {
                            if s.get_channel()?.model.id == session.get_channel()?.model.id
                                && s.id != session.id
                            {
                                s.tx.send(packet.clone())?;
                            }
                        }
                    }
                    Scope::World => {
                        let state = state.lock().await;
                        for s in state.sessions.iter() {
                            if s.get_world()?.model.id == session.get_world()?.model.id
                                && s.id != session.id
                            {
                                s.tx.send(packet.clone())?;
                            }
                        }
                    }
                    Scope::Global => {
                        let state = state.lock().await;
                        for s in state.sessions.iter() {
                            if s.id != session.id {
                                s.tx.send(packet.clone())?;
                            }
                        }
                    }
                },
                Action::Set(set_action) => match set_action {
                    SetAction::SetMap { map, scope } => match scope {
                        Scope::Local => {
                            let state = state.lock().await;
                            state.sessions.update(session.id, |s| {
                                s.map = Some(map.clone());
                            });
                        }

                        Scope::Map => {
                            let state = state.lock().await;
                            for s in state.sessions.iter() {
                                if s.get_map()?.model.id == session.get_map()?.model.id
                                    && s.id != session.id
                                {
                                    state.sessions.update(s.id, |s| {
                                        s.map = Some(map.clone());
                                    });
                                }
                            }
                        }
                        Scope::Channel => {
                            let state = state.lock().await;
                            for s in state.sessions.iter() {
                                if s.get_channel()?.model.id == session.get_channel()?.model.id
                                    && s.id != session.id
                                {
                                    state.sessions.update(s.id, |s| {
                                        s.map = Some(map.clone());
                                    });
                                }
                            }
                        }
                        Scope::World => {
                            let state = state.lock().await;
                            for s in state.sessions.iter() {
                                if s.get_world()?.model.id == session.get_world()?.model.id
                                    && s.id != session.id
                                {
                                    state.sessions.update(s.id, |s| {
                                        s.map = Some(map.clone());
                                    });
                                }
                            }
                        }

                        Scope::Global => {
                            let state = state.lock().await;
                            for s in state.sessions.iter() {
                                if s.id != session.id {
                                    state.sessions.update(s.id, |s| {
                                        s.map = Some(map.clone());
                                    });
                                }
                            }
                        }
                    },
                    SetAction::SetChannel { channel, scope } => match scope {
                        Scope::Local => {
                            let state = state.lock().await;
                            state.sessions.update(session.id, |s| {
                                s.channel = Some(channel.clone());
                            });
                        }
                        Scope::Map => {
                            let state = state.lock().await;
                            for s in state.sessions.iter() {
                                if s.get_map()?.model.id == session.get_map()?.model.id
                                    && s.id != session.id
                                {
                                    state.sessions.update(s.id, |s| {
                                        s.channel = Some(channel.clone());
                                    });
                                }
                            }
                        }
                        Scope::Channel => {
                            let state = state.lock().await;
                            for s in state.sessions.iter() {
                                if s.get_channel()?.model.id == session.get_channel()?.model.id
                                    && s.id != session.id
                                {
                                    state.sessions.update(s.id, |s| {
                                        s.channel = Some(channel.clone());
                                    });
                                }
                            }
                        }
                        Scope::World => {
                            let state = state.lock().await;
                            for s in state.sessions.iter() {
                                if s.get_world()?.model.id == session.get_world()?.model.id
                                    && s.id != session.id
                                {
                                    state.sessions.update(s.id, |s| {
                                        s.channel = Some(channel.clone());
                                    });
                                }
                            }
                        }
                        Scope::Global => {
                            let state = state.lock().await;
                            for s in state.sessions.iter() {
                                if s.id != session.id {
                                    state.sessions.update(s.id, |s| {
                                        s.channel = Some(channel.clone());
                                    });
                                }
                            }
                        }
                    },
                    SetAction::SetWorld { world, scope } => match scope {
                        Scope::Local => {
                            let state = state.lock().await;
                            state.sessions.update(session.id, |s| {
                                s.world = Some(world.clone());
                            });
                        }
                        Scope::Map => {
                            let state = state.lock().await;
                            for s in state.sessions.iter() {
                                if s.get_map()?.model.id == session.get_map()?.model.id
                                    && s.id != session.id
                                {
                                    state.sessions.update(s.id, |s| {
                                        s.world = Some(world.clone());
                                    });
                                }
                            }
                        }
                        Scope::Channel => {
                            let state = state.lock().await;
                            for s in state.sessions.iter() {
                                if s.get_channel()?.model.id == session.get_channel()?.model.id
                                    && s.id != session.id
                                {
                                    state.sessions.update(s.id, |s| {
                                        s.world = Some(world.clone());
                                    });
                                }
                            }
                        }
                        Scope::World => {
                            let state = state.lock().await;
                            for s in state.sessions.iter() {
                                if s.get_world()?.model.id == session.get_world()?.model.id
                                    && s.id != session.id
                                {
                                    state.sessions.update(s.id, |s| {
                                        s.world = Some(world.clone());
                                    });
                                }
                            }
                        }
                        Scope::Global => {
                            let state = state.lock().await;
                            for s in state.sessions.iter() {
                                if s.id != session.id {
                                    state.sessions.update(s.id, |s| {
                                        s.world = Some(world.clone());
                                    });
                                }
                            }
                        }
                    },
                    SetAction::SetAccount { acc } => {
                        let state = state.lock().await;
                        state.sessions.update(session.id, |s| {
                            s.acc = Some(acc.clone());
                        });
                    }
                    SetAction::SetChar { char } => {
                        let state = state.lock().await;
                        state.sessions.update(session.id, |s| {
                            s.char = Some(char.clone());
                        });
                    }
                },
            }
        }
        return Ok(ControlFlow::Continue(()));
    }
}

pub struct LoginRelay {
    session_id: i32,
}

impl RuntimeRelay for LoginRelay {
    async fn new(session_id: i32) -> Result<Self, RuntimeError> {
        Ok(Self { session_id })
    }

    fn session_id(&self) -> i32 {
        self.session_id
    }

    async fn handle_packet(
        &mut self,
        state: &SharedState,
        packet: &Packet,
    ) -> Result<HandlerResult, RuntimeError> {
        let session = {
            let state = state.lock().await;
            state
                .sessions
                .get(self.session_id())
                .ok_or(SessionError::NotFound(self.session_id()))?
        };
        let op = packet.opcode();
        let en = RecvOpcode::from_i16(op).ok_or(RuntimeError::UnsupportedOpcodeError(
            op,
            String::from("not expected during authentication"),
        ));
        debug!(
            "Received opcode in login: {} (0x{:02X}) ({:?}),",
            op, op, en
        );
        match op {
            x if x == RecvOpcode::RequestLogin as i16 => {
                let handler = CredentialsHandler::new();
                Ok(handler.handle(state, session.clone(), packet).await?)
            }
            x if x == RecvOpcode::LoginStarted as i16 => {
                let handler = LoginStartHandler::new();
                Ok(handler.handle(state, session.clone(), packet).await?)
            }
            x if x == RecvOpcode::AcceptTOS as i16 => {
                let handler = TosHandler::new();
                Ok(handler.handle(state, session.clone(), packet).await?)
            }
            x if x == RecvOpcode::ServerListRequest as i16 => {
                let handler = ListWorldsHandler::new();
                Ok(handler.handle(state, session.clone(), packet).await?)
            }
            x if x == RecvOpcode::ServerStatusRequest as i16 => {
                let handler = ServerStatusHandler::new();
                Ok(handler.handle(state, session.clone(), packet).await?)
            }
            x if x == RecvOpcode::CharListRequest as i16 => {
                let handler = ListCharsHandler::new();
                Ok(handler.handle(state, session.clone(), packet).await?)
            }
            x if x == RecvOpcode::CreateChar as i16 => {
                let handler = CreateCharHandler::new();
                Ok(handler.handle(state, session.clone(), packet).await?)
            }
            x if x == RecvOpcode::CheckCharName as i16 => {
                let handler = CheckCharNameHandler::new();
                Ok(handler.handle(state, session.clone(), packet).await?)
            }
            x if x == RecvOpcode::DeleteChar as i16 => {
                let handler = DeleteCharHandler::new();
                Ok(handler.handle(state, session.clone(), packet).await?)
            }
            x if x == RecvOpcode::CharSelect as i16 => {
                let handler = SelectCharHandler::new();
                Ok(handler.handle(state, session.clone(), packet).await?)
            }
            x if x == RecvOpcode::RegisterPic as i16 => {
                let handler = RegisterPicHandler::new();
                Ok(handler.handle(state, session.clone(), packet).await?)
            }
            x if x == RecvOpcode::CharSelectWithPic as i16 => {
                let handler = SelectCharWithPicHandler::new();
                Ok(handler.handle(state, session.clone(), packet).await?)
            }
            _ => Err(RuntimeError::UnsupportedOpcodeError(
                op,
                String::from("expected after authentication"),
            )),
        }
    }
}

pub struct PlayerRelay {
    session_id: i32,
}

impl RuntimeRelay for PlayerRelay {
    async fn new(session_id: i32) -> Result<Self, RuntimeError> {
        Ok(Self { session_id })
    }

    fn session_id(&self) -> i32 {
        self.session_id
    }

    async fn handle_packet(
        &mut self,
        state: &SharedState,
        packet: &Packet,
    ) -> Result<HandlerResult, RuntimeError> {
        let session = {
            let state = state.lock().await;
            state
                .sessions
                .get(self.session_id())
                .ok_or(SessionError::NotFound(self.session_id()))?
        };
        let op = packet.opcode();
        let en = RecvOpcode::from_i16(op).ok_or(RuntimeError::UnsupportedOpcodeError(
            op,
            String::from("not expected in channel"),
        ));
        debug!(
            "Received opcode in channel: {} (0x{:02X}) ({:?})",
            op, op, en
        );
        match op {
            x if x == RecvOpcode::PlayerLoggedIn as i16 => {
                let handler = PlayerLoggedInHandler::new();
                Ok(handler.handle(state, session.clone(), packet).await?)
            }
            x if x == RecvOpcode::ChangeChannel as i16 => {
                let handler = ChangeChannelHandler::new();
                Ok(handler.handle(state, session.clone(), packet).await?)
            }
            x if x == RecvOpcode::PartySearch as i16 => {
                let handler = PartySearchHandler::new();
                Ok(handler.handle(state, session.clone(), packet).await?)
            }
            x if x == RecvOpcode::PlayerMapTransfer as i16 => {
                let handler = PlayerMapTransferHandler::new();
                Ok(handler.handle(state, session.clone(), packet).await?)
            }
            x if x == RecvOpcode::PlayerMove as i16 => {
                let handler = MovePlayerHandler::new();
                Ok(handler.handle(state, session.clone(), packet).await?)
            }
            x if x == RecvOpcode::EnterCashShop as i16 => {
                let handler = EnterCashShopHandler::new();
                Ok(handler.handle(state, session.clone(), packet).await?)
            }
            x if x == RecvOpcode::ChangeKeymap as i16 => {
                let handler = ChangeKeymapHandler::new();
                Ok(handler.handle(state, session.clone(), packet).await?)
            }
            x if x == RecvOpcode::CloseAttack as i16 => {
                let handler = CloseAttackHandler::new();
                Ok(handler.handle(state, session.clone(), packet).await?)
            }
            _ => Err(RuntimeError::UnsupportedOpcodeError(
                op,
                String::from("expected in channel"),
            )),
        }
    }
}
