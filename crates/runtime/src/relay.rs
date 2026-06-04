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

use crate::error::RuntimeError;
use crate::handshake;
use ipc::asyncronous::event::AsyncEvent;
use net::packet::io::{read::PacketReader, write::PacketWriter};
use net::packet::model::Packet;
use rand::{RngExt, rng};
use std::sync::mpsc::Sender;
use tokio::net::TcpStream;
use tokio::sync::mpsc::Receiver;

pub struct Runtime {
    pub pkt_reader: PacketReader,
    pub pkt_writer: PacketWriter,
}

impl Runtime {
    pub async fn new(stream: TcpStream, packet: Option<Packet>) -> Result<Self, RuntimeError> {
        let (recv_iv, send_iv) = {
            let mut recv_iv = [0u8; 4];
            let mut send_iv = [0u8; 4];
            let mut rng = rng();
            rng.fill(&mut recv_iv[..]);
            rng.fill(&mut send_iv[..]);
            (recv_iv, send_iv)
        };
        let handshake: Packet = handshake::build_handshake_packet(recv_iv, send_iv)
            .await?
            .finish();
        let (read_half, write_half) = stream.into_split();
        let pkt_reader = PacketReader::new(read_half, &recv_iv)?;
        let mut pkt_writer = PacketWriter::new(write_half, &send_iv).await?;
        pkt_writer.send_unencrypted_packet(&handshake).await?;
        if let Some(mut packet) = packet {
            pkt_writer.send_encrypted_packet(&mut packet).await?;
        }
        Ok(Self {
            pkt_reader,
            pkt_writer,
        })
    }

    pub async fn run(
        mut self,
        cid: i32,
        event_tx: Sender<AsyncEvent>,
        mut rx: Receiver<Packet>,
    ) -> Result<(), RuntimeError> {
        let read_task = tokio::spawn(async move {
            loop {
                match self.pkt_reader.read_packet().await {
                    Ok(raw) => {
                        event_tx
                            .send(AsyncEvent::PacketReceived {
                                client_id: cid,
                                packet: raw,
                            })
                            .unwrap();
                    }
                    Err(_) => {
                        event_tx
                            .send(AsyncEvent::ClientDisconnected { client_id: cid })
                            .unwrap();
                        break;
                    }
                }
            }
        });
        let write_task = tokio::spawn(async move {
            while let Some(mut packet) = rx.recv().await {
                self.pkt_writer
                    .send_encrypted_packet(&mut packet)
                    .await
                    .unwrap();
            }
        });
        let _ = tokio::try_join!(read_task, write_task);
        Ok(())
    }
}
