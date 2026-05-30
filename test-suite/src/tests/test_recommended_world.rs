use crate::error::HarnessError;
use crate::net::connection::TestConnection;
use net::packet::io::error::IOError::ReadError;
use net::packet::model::Packet;
use net::packet::prelude::*;
use std::io::Cursor;

pub const PHASE: &str = "recommended world";

pub async fn assert_recommended_world(
    mut conn: TestConnection,
) -> Result<TestConnection, HarnessError> {
    dbg!(PHASE);
    assert_recommended_world_result(&mut conn).await?;
    Ok(conn)
}

fn read_recommended_world_packet(packet: &Packet) -> Result<(), HarnessError> {
    let mut cursor = Cursor::new(&packet.bytes[..]);
    cursor
        .read_short()
        .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
    Ok(())
}

async fn assert_recommended_world_result(conn: &mut TestConnection) -> Result<(), HarnessError> {
    let packet: Packet = conn.read_packet(PHASE).await?;
    read_recommended_world_packet(&packet)?;
    Ok(())
}
