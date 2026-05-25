use crate::error::HarnessError;
use crate::net::connection::TestConnection;
use crate::tests::test_char_list;
use crate::tests::test_char_list::CHANNEL_ID;
use crate::tests::test_create_char::{CHAR_ID, IGN};
use op::recv::RecvOpcode;
use op::send::SendOpcode;
use packet::io::error::IOError::{ReadError, WriteError};
use packet::model::Packet;
use packet::prelude::*;
use std::io::Cursor;

pub const PHASE: &str = "player logged in";

pub struct SetFieldResult {
    pub char_id: i32,
    pub char_name: String,
    pub map_wz: i32,
}

pub async fn assert_player_logged_in(
    mut conn: TestConnection,
) -> Result<TestConnection, HarnessError> {
    dbg!(PHASE);
    conn.send_packet(build_player_logged_in(CHAR_ID, CHANNEL_ID)?, PHASE)
        .await?;
    assert_player_logged_in_result(&mut conn).await?;
    Ok(conn)
}

pub fn build_player_logged_in(character_id: i32, channel_id: u8) -> Result<Packet, HarnessError> {
    let mut packet = Packet::new_empty();
    packet
        .write_short(RecvOpcode::PlayerLoggedIn as i16)
        .map_err(|e| HarnessError::PacketIOError(WriteError(e)))?;
    packet
        .write_int(character_id)
        .map_err(|e| HarnessError::PacketIOError(WriteError(e)))?;
    packet
        .write_byte(channel_id as i16)
        .map_err(|e| HarnessError::PacketIOError(WriteError(e)))?;
    Ok(packet)
}

async fn assert_player_logged_in_result(conn: &mut TestConnection) -> Result<(), HarnessError> {
    let packet = conn.read_packet(PHASE).await?;
    let mut saw_keymap = false;
    let mut world_entry = None;
    let max_chars: i32 = 8;
    for _ in 0..max_chars {
        let mut cursor = Cursor::new(&packet.bytes[..]);
        let op = cursor
            .read_short()
            .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
        match op {
            x if x == SendOpcode::KeyMap as i16 => {
                saw_keymap = true;
            }
            x if x == SendOpcode::SetField as i16 => {
                let result = read_set_field_packet(&packet)?;
                assert_eq!(result.char_name, IGN);
                assert_eq!(result.char_id, CHAR_ID);
                assert_eq!(result.map_wz, CHAR_ID);
                world_entry = Some(result);
            }
            _ => (),
        }
        if saw_keymap && world_entry.is_some() {
            break;
        }
    }
    assert!(world_entry.is_some());
    assert!(saw_keymap);
    Ok(())
}

pub fn read_set_field_packet(packet: &Packet) -> Result<SetFieldResult, HarnessError> {
    let mut cursor = Cursor::new(&packet.bytes[..]);
    cursor
        .read_short()
        .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
    let skip: usize = 20;
    cursor
        .read_bytes(skip)
        .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
    let skip: usize = 9;
    cursor
        .read_bytes(skip)
        .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
    let meta = test_char_list::read_char_meta(&mut cursor)?;
    Ok(SetFieldResult {
        char_id: meta.id,
        char_name: meta.name,
        map_wz: meta.map_wz,
    })
}
