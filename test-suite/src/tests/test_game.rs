use config::settings;
use state::model::{SharedState, State};
use std::sync::Arc;
use tokio::net::lookup_host;
use tokio::sync::Mutex;

use crate::error::HarnessError;
use crate::net::connection::TestConnection;
use crate::tests::{
    test_char_list, test_create_char, test_credentials, test_handshake, test_last_connected_world,
    test_player_logged_in, test_recommended_world, test_server_list, test_server_redirect,
    test_tos,
};

pub async fn login_until_redirect() -> Result<i16, HarnessError> {
    let state: SharedState = Arc::new(Mutex::new(State::new()?));
    let port: i16 = settings::get_login_port()?;
    let host: String = settings::get_host()?;
    let bind = lookup_host(format!("{host}:{port}"))
        .await
        .map_err(|e| HarnessError::EndpointError(e.to_string()))?
        .next()
        .ok_or(HarnessError::ConnectionError)?;
    let conn = TestConnection::connect(bind, "login handshake").await?;
    test_handshake::assert_handshake(conn.handshake.version, conn.handshake.locale)?;
    let conn = test_credentials::assert_credentials(&state, conn).await?;
    let conn = test_tos::assert_accept_tos(conn).await?;
    let conn = test_server_list::assert_server_list_request(conn).await?;
    let conn = test_last_connected_world::assert_last_connected_world(conn).await?;
    let conn = test_recommended_world::assert_recommended_world(conn).await?;
    let conn = test_char_list::assert_char_list_request(&state, conn).await?;
    let conn = test_create_char::assert_create_char(conn).await?;
    let port = test_server_redirect::assert_server_redirect(conn).await?;
    Ok(port)
}

pub async fn play(port: i16) -> Result<(), HarnessError> {
    // let state: SharedState = Arc::new(Mutex::new(State::new()?));
    let host: String = settings::get_host()?;
    let bind = lookup_host(format!("{host}:{port}"))
        .await
        .map_err(|e| HarnessError::EndpointError(e.to_string()))?
        .next()
        .ok_or(HarnessError::ConnectionError)?;
    let conn = TestConnection::connect(bind, "world handshake").await?;
    test_handshake::assert_handshake(conn.handshake.version, conn.handshake.locale)?;
    let conn = test_player_logged_in::assert_player_logged_in(conn).await?;
    Ok(())
}

#[cfg(test)]
mod tests {

    use crate::error::HarnessError;
    use crate::tests::test_game;
    use config::settings;
    #[tokio::test]
    async fn test() -> Result<(), HarnessError> {
        dotenvy::dotenv().ok();
        println!("{}", settings::get_db_url()?);
        let port: i16 = test_game::login_until_redirect().await?;
        dbg!(port);
        test_game::play(port).await?;
        Ok(())
    }
}
