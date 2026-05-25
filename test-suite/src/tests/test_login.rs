use config::settings;
use inc::helpers;
use state::model::{SharedState, State};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::error::HarnessError;
use crate::net::connection::TestConnection;
use crate::tests::{
    test_char_list, test_create_char, test_credentials, test_handshake, test_last_connected_world,
    test_server_list, test_server_redirect, test_tos,
};

pub async fn login_until_redirect() -> Result<(), HarnessError> {
    let state: SharedState = Arc::new(Mutex::new(State::new()?));
    let addr: String = settings::get_bind_address()?;
    let port: i16 = settings::get_login_port()?;
    let bind = helpers::build_server_addr(addr, port);
    let conn = TestConnection::connect(bind, "login handshake").await?;
    test_handshake::assert_handshake(conn.handshake.version, conn.handshake.locale)?;
    let conn = test_credentials::assert_credentials(&state, conn).await?;
    let conn = test_tos::assert_accept_tos(conn).await?;
    let conn = test_server_list::assert_server_list_request(conn).await?;
    let conn = test_last_connected_world::assert_last_connected_world(conn).await?;
    let conn = test_char_list::assert_char_list_request(&state, conn).await?;
    let conn = test_create_char::assert_create_char(conn).await?;
    test_server_redirect::assert_server_redirect(conn).await?;
    Ok(())
}

#[cfg(test)]
mod tests {

    use crate::error::HarnessError;
    use crate::net::connection::login_until_redirect;
    #[tokio::test]
    async fn test_login() -> Result<(), HarnessError> {
        login_until_redirect().await?;
        Ok(())
    }
}
