use crate::config::settings;
use crate::db::error::DatabaseError;
use crate::inc::helpers;
use crate::models::account;
use crate::models::channel;
use crate::models::channel::model::Channel;
use crate::models::error::ModelError;
use crate::models::world;
use crate::net::error::NetworkError;
use crate::net::packet::core::Packet;
use crate::net::packet::error::PacketError;
use crate::net::packet::handler::action::ChannelAction;
use crate::net::packet::handler::result::HandlerResult;
use crate::net::packet::io::error::IOError::{ReadError, WriteError};
use crate::op::send::SendOpcode;
use crate::prelude::*;
use crate::runtime::error::SessionError;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;
use std::io::Cursor;

pub struct ChangeChannelHandler;

impl ChangeChannelHandler {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle(
        self: &Self,
        state: SharedState,
        session: Session,
        packet: Packet,
    ) -> Result<HandlerResult<ChannelAction>, NetworkError> {
        let mut pkt_reader = Cursor::new(packet.bytes);
        let _op = pkt_reader
            .read_short()
            .map_err(ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        let channel_id = pkt_reader
            .read_byte()
            .map_err(ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        let _tick = pkt_reader
            .read_int()
            .map_err(ReadError)
            .map_err(PacketError::from)
            .map_err(NetworkError::from)?;
        let acc_id = session
            .acc_id
            .ok_or(SessionError::NoAccount)
            .map_err(NetworkError::from)?;
        let acc = account::query::get_account_by_id(state.clone(), acc_id)
            .await
            .map_err(DatabaseError::from)
            .map_err(NetworkError::from)?;
        let worlds = world::service::load_worlds()
            .map_err(ModelError::from)
            .map_err(NetworkError::from)?;
        let channel = channel::service::resolve_channel(
            channel_id as i16,
            acc.selected_world_id.ok_or(NetworkError::UnexpectedError)?,
            worlds,
        )
        .map_err(ModelError::from)
        .map_err(NetworkError::from)?;
        let mut result = HandlerResult::new();
        let packet = build_channel_change_packet(&channel)?;
        let action = ChannelAction::SendPacket { packet };
        result.add_action(action)?;
        Ok(result)
    }
}

pub fn build_channel_change_packet(channel: &Channel) -> Result<Packet, NetworkError> {
    let mut packet = Packet::new_empty();
    let addr = settings::get_address()?;
    let octets = helpers::convert_to_ip_array(addr);
    let op = SendOpcode::ChangeChannel as i16;
    packet
        .write_short(op)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_byte(1)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_bytes(&octets)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    packet
        .write_short(channel.port)
        .map_err(WriteError)
        .map_err(PacketError::from)
        .map_err(NetworkError::from)?;
    Ok(packet)
}
