use crate::error::HarnessError;
use crate::net::connection::TestConnection;
use op::recv::RecvOpcode;
use net::packet::io::error::IOError::{ReadError, WriteError};
use net::packet::model::Packet;
use net::packet::io::prelude::*;
use std::io::Cursor;

pub const PHASE: &str = "tos";
const GENDER_WZ: i16 = 0;
const SUCCESS_STATUS: i32 = 0;

struct TOSResult {
    pub status: i32,
    pub acc_id: Option<i32>,
    pub gender_wz: Option<i16>,
}

pub async fn assert_accept_tos(
    mut conn: TestConnection,
    acc_id: i32,
) -> Result<TestConnection, HarnessError> {
    dbg!(PHASE);
    conn.send_packet(build_accept_tos()?, PHASE).await?;
    assert_accept_tos_result(&mut conn, acc_id).await?;
    Ok(conn)
}

pub fn build_accept_tos() -> Result<Packet, HarnessError> {
    let mut packet = Packet::new_empty();
    packet
        .write_short(RecvOpcode::AcceptTOS as i16)
        .map_err(|e| HarnessError::PacketIOError(WriteError(e)))?;
    packet
        .write_byte(0x01)
        .map_err(|e| HarnessError::PacketIOError(WriteError(e)))?;
    Ok(packet)
}

fn read_accept_tos_packet(packet: &Packet) -> Result<TOSResult, HarnessError> {
    let mut cursor = Cursor::new(&packet.bytes[..]);
    cursor
        .read_short()
        .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
    let status = cursor
        .read_int()
        .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
    if status != 0 {
        return Ok(TOSResult {
            status,
            acc_id: None,
            gender_wz: None,
        });
    }
    cursor
        .read_short()
        .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
    let acc_id = cursor
        .read_int()
        .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
    let gender_wz = cursor
        .read_byte()
        .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
    Ok(TOSResult {
        status,
        acc_id: Some(acc_id),
        gender_wz: Some(gender_wz as i16),
    })
}

async fn assert_accept_tos_result(
    conn: &mut TestConnection,
    acc_id: i32,
) -> Result<(), HarnessError> {
    let packet = conn.read_packet(PHASE).await?;
    let result = read_accept_tos_packet(&packet)?;
    assert_eq!(result.status, SUCCESS_STATUS);
    assert_eq!(result.acc_id, Some(acc_id));
    assert_eq!(result.gender_wz, Some(GENDER_WZ));
    Ok(())
}
