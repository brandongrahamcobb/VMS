use op::recv::RecvOpcode;
use net::packet::io::error::IOError::WriteError;
use net::packet::model::Packet;
use net::packet::io::prelude::*;

pub const PHASE: &str = "login started";
use crate::error::HarnessError;
use crate::net::connection::TestConnection;

fn build_login_started() -> Result<Packet, HarnessError> {
    dbg!(PHASE);
    let mut packet = Packet::new_empty();
    packet
        .write_short(RecvOpcode::LoginStarted as i16)
        .map_err(|e| HarnessError::PacketIOError(WriteError(e)))?;
    Ok(packet)
}

pub async fn assert_login_started(mut conn: TestConnection) -> Result<(), HarnessError> {
    conn.send_packet(build_login_started()?, "login started")
        .await?;
    Ok(())
}
