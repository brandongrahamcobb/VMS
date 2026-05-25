use crate::error::HarnessError;
use crate::net::connection::TestConnection;
use entity::account::model::AccountModel;
use op::recv::RecvOpcode;
use packet::io::error::IOError::{ReadError, WriteError};
use packet::model::Packet;
use packet::prelude::*;
use state::model::SharedState;
use std::io::Cursor;
use std::time::SystemTime;

const USERNAME: &str = "admin";
const PASSWORD: &str = "admin";
const PIN: &str = "123456";
const PIC: &str = "654321";
pub const PHASE: &str = "credentials";
const LOGIN_PADDING_LEN: usize = 6;
const LOGIN_HWID: [u8; 4] = [0, 0, 0, 0];
pub const ACC_ID: i32 = 1;
pub const GENDER_WZ: i16 = 0;
const TOS_STATUS: i32 = 23;

struct CredentialsResult {
    pub status: i32,
    pub acc_id: Option<i32>,
    pub gender_wz: Option<i16>,
}

pub async fn assert_credentials(
    state: &SharedState,
    mut conn: TestConnection,
) -> Result<TestConnection, HarnessError> {
    dbg!(PHASE);
    let hashed =
        bcrypt::hash(PASSWORD, bcrypt::DEFAULT_COST).map_err(|e| HarnessError::AccountError(e))?;
    {
        let acc_model: AccountModel = AccountModel {
            id: None,
            username: USERNAME.to_string(),
            password: hashed,
            pin: Some(PIN.to_string()),
            pic: Some(PIC.to_string()),
            last_login_at: Some(SystemTime::now()),
            gender_wz: GENDER_WZ,
            accepted_tos: false,
            banned: false,
            admin: true,
            created_at: Some(SystemTime::now()),
            updated_at: SystemTime::now(),
        };
        db::account::setters::update_accounts(&state.lock().await.db.clone(), vec![acc_model])
            .await?;
    }
    conn.send_packet(build_credentials(USERNAME, PASSWORD)?, PHASE)
        .await?;
    assert_credentials_result(&mut conn).await?;
    Ok(conn)
}

pub fn build_credentials(username: &str, password: &str) -> Result<Packet, HarnessError> {
    let mut packet = Packet::new_empty();
    packet
        .write_short(RecvOpcode::RequestLogin as i16)
        .map_err(|e| HarnessError::PacketIOError(WriteError(e)))?;
    packet
        .write_str_with_length(username.to_string())
        .map_err(|e| HarnessError::PacketIOError(WriteError(e)))?;
    packet
        .write_str_with_length(password.to_string())
        .map_err(|e| HarnessError::PacketIOError(WriteError(e)))?;
    packet
        .write_bytes([0; LOGIN_PADDING_LEN].to_vec())
        .map_err(|e| HarnessError::PacketIOError(WriteError(e)))?;
    packet
        .write_bytes(LOGIN_HWID.to_vec())
        .map_err(|e| HarnessError::PacketIOError(WriteError(e)))?;
    Ok(packet)
}

fn read_credentials_packet(packet: &Packet) -> Result<CredentialsResult, HarnessError> {
    let mut cursor = Cursor::new(&packet.bytes[..]);
    cursor
        .read_short()
        .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
    let status = cursor
        .read_int()
        .map_err(|e| HarnessError::PacketIOError(ReadError(e)))?;
    if status != 0 {
        return Ok(CredentialsResult {
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
    Ok(CredentialsResult {
        status,
        acc_id: Some(acc_id),
        gender_wz: Some(gender_wz as i16),
    })
}

async fn assert_credentials_result(conn: &mut TestConnection) -> Result<(), HarnessError> {
    let packet = conn.read_packet(PHASE).await?;
    let result = read_credentials_packet(&packet)?;
    assert_eq!(result.status, TOS_STATUS);
    if result.status != 23 {
        assert_eq!(result.acc_id, Some(ACC_ID));
        assert_eq!(result.gender_wz, Some(GENDER_WZ));
    }
    Ok(())
}
