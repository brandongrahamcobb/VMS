use crate::error::HarnessError;
use crate::net::connection::TestConnection;
use crate::tests::test_close_attack;

pub const PHASE: &str = "mob attack";

pub async fn assert_mob_attack(
    mut conn: TestConnection,
    char_id: i32,
    mob_id: u32,
) -> Result<TestConnection, HarnessError> {
    dbg!(PHASE);
    conn.send_packet(test_close_attack::build_close_attack(mob_id)?, PHASE)
        .await?;
    test_close_attack::assert_close_attack_result(&mut conn, char_id).await?;
    Ok(conn)
}
