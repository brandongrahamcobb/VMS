use std::io::Cursor;

use net::packet::model::Packet;
use op::send::SendOpcode;

use crate::error::HarnessError;
use crate::net::connection::TestConnection;
use crate::tests::test_close_attack::{self, CloseAttackResult};
use net::packet::io::error::IOError::ReadError;
use net::packet::io::prelude::*;

pub const PHASE: &str = "mob attack";
pub const COUNTS: i16 = 0x11;

pub async fn assert_mob_attack(
    mut conn: TestConnection,
    char_id: i32,
    mob_id: u32,
) -> Result<TestConnection, HarnessError> {
    dbg!(PHASE);
    conn.send_packet(
        test_close_attack::build_close_attack(COUNTS, mob_id)?,
        PHASE,
    )
    .await?;
    assert_mob_attack_result(&mut conn, char_id, mob_id).await?;
    Ok(conn)
}

pub async fn assert_mob_attack_result(
    conn: &mut TestConnection,
    char_id: i32,
    mob_id: u32,
) -> Result<(), HarnessError> {
    let mut attacked_close_seen: bool = false;
    let mut show_mob_hp_seen: bool = false;
    loop {
        let packet = conn.read_packet(PHASE).await?;
        let mut cursor = Cursor::new(&packet.bytes[..]);
        let op = cursor
            .read_short()
            .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
        if op == SendOpcode::AttackedClose as i16 {
            let result: CloseAttackResult = test_close_attack::read_close_attack_packet(&packet)?;
            assert_eq!(result.char_id, char_id);
            attacked_close_seen = true;
        } else if op == SendOpcode::ShowMobHp as i16 {
            let mob_oid: u32 = read_show_mob_hp_packet(&packet)?;
            assert_eq!(mob_oid, mob_id);
            show_mob_hp_seen = true;
        }
        if attacked_close_seen && show_mob_hp_seen {
            break;
        }
    }
    Ok(())
}
pub fn read_show_mob_hp_packet(packet: &Packet) -> Result<u32, HarnessError> {
    let mut cursor = Cursor::new(&packet.bytes[..]);
    let _op = cursor
        .read_short()
        .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
    let mob_id = cursor
        .read_int()
        .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
    let _hp_percent = cursor
        .read_byte()
        .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
    Ok(mob_id as u32)
}
