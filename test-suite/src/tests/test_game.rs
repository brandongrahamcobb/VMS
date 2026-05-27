use config::settings;
use state::model::SharedState;
use std::sync::Arc;
use tokio::sync::Semaphore;

use crate::error::HarnessError;
use crate::net::connection::TestConnection;
use crate::tests::{
    test_char_list, test_close_attack, test_create_char, test_credentials, test_handshake,
    test_last_connected_world, test_move_player, test_player_logged_in, test_recommended_world,
    test_server_list, test_server_redirect, test_tos,
};

#[derive(Clone)]
pub struct LoginDetails {
    pub acc_id: i32,
    pub char_id: i32,
    pub char_ign: String,
    pub port: i16,
    pub world_id: i16,
}

pub async fn login_until_redirect(
    state: &SharedState,
    acc_username: &str,
    char_ign: &str,
) -> Result<LoginDetails, HarnessError> {
    let port: i16 = settings::get_login_port()?;
    let host: String = settings::get_host()?;
    let bind = tokio::net::lookup_host(format!("{host}:{port}"))
        .await
        .map_err(|e| HarnessError::EndpointError(e.to_string()))?
        .next()
        .ok_or(HarnessError::ConnectionError)?;
    let conn = TestConnection::connect(bind, "login handshake").await?;
    test_handshake::assert_handshake(conn.handshake.version, conn.handshake.locale)?;
    let (acc_id, conn) = test_credentials::assert_credentials(state, conn, acc_username).await?;
    let conn = test_tos::assert_accept_tos(conn, acc_id).await?;
    let conn = test_server_list::assert_server_list_request(conn).await?;
    let conn = test_last_connected_world::assert_last_connected_world(conn).await?;
    let conn = test_recommended_world::assert_recommended_world(conn).await?;
    let (world_id, conn) = test_char_list::assert_char_list_request(state, conn, acc_id).await?;
    let (char_id, conn) = test_create_char::assert_create_char(conn, char_ign).await?;
    let port = test_server_redirect::assert_server_redirect(conn, char_id).await?;
    Ok(LoginDetails {
        acc_id,
        char_id,
        char_ign: char_ign.to_string(),
        port,
        world_id,
    })
}

pub async fn play(details: &LoginDetails) -> Result<TestConnection, HarnessError> {
    // let state: SharedState = Arc::new(Mutex::new(State::new()?));
    let host: String = settings::get_host()?;
    let addr_str: String = format!("{}:{}", host, details.port);
    let bind = tokio::net::lookup_host(addr_str)
        .await
        .map_err(|e| HarnessError::EndpointError(e.to_string()))?
        .next()
        .ok_or(HarnessError::ConnectionError)?;
    let conn = TestConnection::connect(bind, "world handshake").await?;
    test_handshake::assert_handshake(conn.handshake.version, conn.handshake.locale)?;
    let conn =
        test_player_logged_in::assert_player_logged_in(conn, details.char_id, &details.char_ign)
            .await?;
    Ok(conn)
}

pub async fn send_player_test(
    conn: TestConnection,
    a_turn: Arc<Semaphore>,
    b_turn: Arc<Semaphore>,
) -> Result<TestConnection, HarnessError> {
    a_turn.acquire().await.unwrap().forget();
    let conn = test_move_player::send_move_player(conn).await?;
    b_turn.add_permits(1);
    a_turn.acquire().await.unwrap().forget();
    let conn = test_close_attack::send_close_attack(conn).await?;
    b_turn.add_permits(1);
    a_turn.acquire().await.unwrap().forget();
    Ok(conn)
}

pub async fn receive_player_test(
    state: &SharedState,
    conn: TestConnection,
    first_char_details: LoginDetails,
    a_turn: Arc<Semaphore>,
    b_turn: Arc<Semaphore>,
) -> Result<(), HarnessError> {
    b_turn.acquire().await.unwrap().forget();
    let conn = test_move_player::assert_move_player(conn, first_char_details.char_id).await?;
    a_turn.add_permits(1);
    b_turn.acquire().await.unwrap().forget();
    let conn = test_close_attack::assert_close_attack(conn, first_char_details.char_id).await?;
    a_turn.add_permits(1);
    Ok(())
}

#[cfg(test)]
mod tests {

    use crate::net::connection::TestConnection;
    use crate::tests::test_game;
    use crate::{error::HarnessError, tests::test_change_channel};
    use state::model::{SharedState, State};
    use std::sync::Arc;
    use tokio::sync::{Mutex, Semaphore};

    #[tokio::test]
    async fn main_test() -> Result<(), HarnessError> {
        dotenvy::dotenv().ok();
        let state: SharedState = Arc::new(Mutex::new(State::new()?));
        let a_turn = Arc::new(Semaphore::new(1));
        let b_turn = Arc::new(Semaphore::new(0));
        let a_turn_clone = a_turn.clone();
        let b_turn_clone = b_turn.clone();
        let first_char_details = {
            let acc_username: &str = "admin1";
            let char_ign: &str = "Test1";
            test_game::login_until_redirect(&state, acc_username, char_ign).await?
        };
        let second_char_details = {
            let acc_username: &str = "admin2";
            let char_ign: &str = "Test2";
            test_game::login_until_redirect(&state, acc_username, char_ign).await?
        };
        let first_details_clone = first_char_details.clone();
        let first_char = tokio::spawn(async move {
            let conn: TestConnection = test_game::play(&first_details_clone).await?;
            test_game::send_player_test(conn, a_turn, b_turn).await?;
            Ok::<_, HarnessError>(())
        });
        let state_clone = state.clone();
        let second_char = tokio::spawn(async move {
            let conn: TestConnection = test_game::play(&second_char_details).await?;
            test_game::receive_player_test(
                &state_clone,
                conn,
                first_char_details,
                a_turn_clone,
                b_turn_clone,
            )
            .await?;
            Ok::<_, HarnessError>(())
        });
        first_char
            .await
            .map_err(|_| HarnessError::ConnectionError)??;
        second_char
            .await
            .map_err(|_| HarnessError::ConnectionError)??;
        side_tests(&state).await?;
        Ok(())
    }

    async fn side_tests(state: &SharedState) -> Result<(), HarnessError> {
        let details = {
            let acc_username: &str = "admin3";
            let char_ign: &str = "Test3";
            test_game::login_until_redirect(&state, acc_username, char_ign).await?
        };
        let conn: TestConnection = test_game::play(&details).await?;
        let conn = test_change_channel::send_change_channel(conn).await?;
        let conn = test_change_channel::assert_change_channel(
            &state,
            conn,
            details.world_id,
            details.char_id,
        )
        .await?;
        Ok(())
    }
}
