use crate::net::error::NetworkError;
use crate::net::packet::handler::login_start::reader::LoginStartReader;
use crate::runtime::session::Session;
use crate::runtime::state::SharedState;

#[derive(Clone)]
pub struct LoginStartStore;

impl LoginStartStore {
    pub async fn store_login_start(
        state: &SharedState,
        session: Session,
        reader: LoginStartReader,
    ) -> Result<Self, NetworkError> {
        std::hint::black_box(state);
        std::hint::black_box(session);
        std::hint::black_box(reader.clone());
        Ok(Self)
    }
}
