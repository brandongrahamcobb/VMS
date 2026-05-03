use crate::net::error::NetworkError;
use crate::net::packet::error::PacketError;
use crate::net::packet::handler::error::HandlerError;

pub fn get_map_id_for_job(job: i16) -> Result<i32, NetworkError> {
    match job {
        1 => Ok(0),
        1000 => Ok(130000000),
        2000 => Ok(140000000),
        _ => Err(NetworkError::from(PacketError::from(
            HandlerError::LoginError,
        ))),
    }
}
