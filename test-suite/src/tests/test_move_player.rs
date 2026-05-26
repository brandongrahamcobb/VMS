use crate::error::HarnessError;
use crate::net::connection::TestConnection;
use op::recv::RecvOpcode;
use packet::io::error::IOError::{ReadError, WriteError};
use packet::model::Packet;
use packet::prelude::*;
use std::io::Cursor;

pub const SEND_PHASE: &str = "send move player";
pub const RECEIVE_PHASE: &str = "receive move player";
struct MovePlayerResult {
    char_id: i32,
    movement: Vec<u8>,
}

pub async fn assert_move_player(
    mut conn: TestConnection,
    char_id: i32,
) -> Result<TestConnection, HarnessError> {
    dbg!(RECEIVE_PHASE);
    assert_move_player_result(&mut conn, char_id).await?;
    Ok(conn)
}

fn read_move_player_packet(packet: &Packet) -> Result<MovePlayerResult, HarnessError> {
    let mut cursor = Cursor::new(&packet.bytes[..]);
    cursor
        .read_short()
        .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
    let char_id = cursor
        .read_int()
        .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
    cursor
        .read_int()
        .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
    let mvmt_bytes = cursor
        .read_bytes(0)
        .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
    Ok(MovePlayerResult {
        char_id,
        movement: mvmt_bytes,
    })
}

async fn assert_move_player_result(
    conn: &mut TestConnection,
    char_id: i32,
) -> Result<(), HarnessError> {
    let packet: Packet = conn.read_packet(RECEIVE_PHASE).await?;
    let result: MovePlayerResult = read_move_player_packet(&packet)?;
    assert_eq!(result.char_id, char_id);
    Ok(())
}

pub async fn send_move_player(mut conn: TestConnection) -> Result<TestConnection, HarnessError> {
    dbg!(SEND_PHASE);
    conn.send_packet(build_move_player()?, SEND_PHASE).await?;
    Ok(conn)
}

fn build_move_player() -> Result<Packet, HarnessError> {
    let mut packet = Packet::new_empty();
    packet
        .write_short(RecvOpcode::PlayerMove as i16)
        .map_err(|e| HarnessError::PacketIOError(WriteError(e)))?;
    packet
        .write_bytes(vec![0u8; 9])
        .map_err(|e| HarnessError::PacketIOError(WriteError(e)))?;
    packet
        .write_byte(0)
        .map_err(|e| HarnessError::PacketIOError(WriteError(e)))?;
    Ok(packet)
}
