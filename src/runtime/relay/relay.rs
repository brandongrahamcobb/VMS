/* relay.rs
 * The purpose of this module is to provide the run and execution logic of the relay.
 *
 * Copyright (C) 2026  https://github.com/brandongrahamcobb/VMS.gitrand::
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

use crate::net::packet::handler::error::PacketHandlerError;
use crate::net::packet::io::{read::PacketReader, write::PacketWriter};
use crate::net::packet::model::Packet;
use crate::runtime::relay::error::RelayError;
use crate::runtime::relay::model::Runtime;
use crate::runtime::relay::types::shared::RuntimeRelay;
use crate::runtime::state::SharedState;
use core::ops::ControlFlow;
use rand::{RngExt, rng};
use tokio::net::TcpStream;
use tokio::sync::mpsc::UnboundedReceiver;

impl<T: RuntimeRelay + Send> Runtime<T> {
    pub async fn new(
        state: SharedState,
        stream: TcpStream,
        session_id: i32,
        rx: UnboundedReceiver<Packet>,
    ) -> Result<Self, RelayError> {
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
            .await
            .map_err(PacketHandlerError::from)?
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

    pub async fn run(mut self) -> Result<Option<(Self, Packet)>, RelayError> {
        loop {
            let tick = async {
                match self.relay.tick_rx() {
                    Some(rx) => rx.recv().await.ok(),
                    None => std::future::pending().await,
                }
            };
            tokio::select! {
                packet = self.pkt_reader.read_packet() => {
                    let packet = packet?;
                    let result = self.relay
                        .handle_packet(&self.state, &packet)
                        .await?;
                    match self.relay.execute_with_session(&self.state, result).await? {
                        ControlFlow::Break(packet) => break Ok(Some((self, packet))),
                        _ => {}
                    }
                }
                packet = self.rx.recv() => {
                    match packet {
                        Some(mut packet) => {
                            self.pkt_writer.send_encrypted_packet(&mut packet).await?;
                        }
                        None => {},
                    }
                }
                result = tick => {
                    match result {
                        Some(result) => {
                            match self.relay.execute_via_tick(&self.state, result).await? {
                                ControlFlow::Break(packet) => break Ok(Some((self, packet))),
                                _ => {}
                            }
                        }
                        None => {},
                    }
                }
            }
        }
    }
}
