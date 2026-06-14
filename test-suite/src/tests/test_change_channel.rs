use crate::error::HarnessError;
use crate::net::connection::TestConnection;
use crate::tests::test_player_logged_in;
use config::settings;
use inc::helpers;
use net::packet::io::error::IOError::{ReadError, WriteError};
use net::packet::io::prelude::*;
use net::packet::model::Packet;
use op::recv::RecvOpcode;
use op::send::SendOpcode;
use std::io::Cursor;

pub const MAP_WZ: i32 = 10000;
pub const PHASE: &str = "change channel";
pub const CHANNEL_ID: u8 = 2;
pub const PORT: i16 = 8588;

struct ChangeChannelResult {
    octets: Vec<u8>,
    port: i16,
}

pub async fn assert_change_channel(
    mut conn: TestConnection,
    char_ign: &str,
    char_id: i32,
) -> Result<TestConnection, HarnessError> {
    dbg!(PHASE);
    conn.send_packet(build_change_channel(CHANNEL_ID)?, PHASE)
        .await?;
    let conn = assert_change_channel_result(&mut conn, char_ign, char_id, PORT).await?;
    Ok(conn)
}

fn read_change_channel_packet(packet: &Packet) -> Result<ChangeChannelResult, HarnessError> {
    let mut cursor = Cursor::new(&packet.bytes[..]);
    let _op = cursor
        .read_short()
        .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
    cursor
        .read_byte()
        .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
    let octets: Vec<u8> = cursor
        .read_bytes(4)
        .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
    let port: i16 = cursor
        .read_short()
        .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;

    Ok(ChangeChannelResult { octets, port })
}

fn read_despawn_player_packet(packet: &Packet) -> Result<i32, HarnessError> {
    let mut cursor = Cursor::new(&packet.bytes[..]);
    let _op = cursor
        .read_short()
        .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
    let char_id = cursor
        .read_int()
        .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
    Ok(char_id)
}

async fn assert_change_channel_result(
    conn: &mut TestConnection,
    char_ign: &str,
    char_id: i32,
    port: i16,
) -> Result<TestConnection, HarnessError> {
    let mut conn = loop {
        let packet = conn.read_packet(PHASE).await?;
        let mut cursor = Cursor::new(&packet.bytes[..]);
        let op = cursor
            .read_short()
            .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
        match op {
            x if x == SendOpcode::DespawnPlayer as i16 => {
                let result_char_id = read_despawn_player_packet(&packet)?;
                assert_eq!(result_char_id, char_id);
            }
            x if x == SendOpcode::ChangeChannel as i16 => {
                let addr: String = settings::get_routing_address()?;
                let octets: [u8; 4] = helpers::convert_to_ip_array(addr);
                let result = read_change_channel_packet(&packet)?;
                assert_eq!(result.port, port);
                assert_eq!(result.octets, octets.to_owned());
                let host: String = settings::get_host()?;
                let addr_str: String = format!("{}:{}", host, port);
                let bind = tokio::net::lookup_host(addr_str)
                    .await
                    .map_err(|e| HarnessError::EndpointError(e.to_string()))?
                    .next()
                    .ok_or(HarnessError::ConnectionError)?;
                let mut conn = TestConnection::connect(bind, "changed channel").await?;
                conn.send_packet(
                    test_player_logged_in::build_player_logged_in(char_id, CHANNEL_ID)?,
                    PHASE,
                )
                .await?;
                conn.send_packet(test_player_logged_in::build_player_map_transfer()?, PHASE)
                    .await?;
                break conn;
            }
            _ => {}
        }
    };
    let mut world_entry = None;
    let mut saw_keymap = false;
    loop {
        let packet = conn.read_packet(PHASE).await?;
        let mut cursor = Cursor::new(&packet.bytes[..]);
        let op = cursor
            .read_short()
            .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
        match op {
            x if x == SendOpcode::KeyMap as i16 => {
                saw_keymap = true;
            }
            x if x == SendOpcode::SetField as i16 => {
                let result = test_player_logged_in::read_set_field_packet(&packet)?;
                assert_eq!(result.char_name, char_ign);
                assert_eq!(result.char_id, char_id);
                assert_eq!(result.map_wz, MAP_WZ);
                world_entry = Some(result);
            }
            _ => {}
        }
        if saw_keymap && world_entry.is_some() {
            break;
        }
    }
    assert!(world_entry.is_some());
    assert!(saw_keymap);
    Ok(conn)
}

fn build_change_channel(channel_id: u8) -> Result<Packet, HarnessError> {
    let mut packet = Packet::new_empty();
    packet
        .write_short(RecvOpcode::ChangeChannel as i16)
        .map_err(|e| HarnessError::PacketIOError(WriteError(e)))?;
    packet
        .write_byte(channel_id as i16)
        .map_err(|e| HarnessError::PacketIOError(WriteError(e)))?;
    packet
        .write_int(0)
        .map_err(|e| HarnessError::PacketIOError(WriteError(e)))?;
    Ok(packet)
}
