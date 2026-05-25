use core::sync::atomic::AtomicU64;
use core::time::Duration;
use state::model::SharedState;
use tokio::sync::Notify;

use crate::{error::TickError, mob_respawn::tick::MobRespawnTick};

pub const MS_PER_TICK: u8 = 50;

pub struct TickManager {
    clock: AtomicU64,
    notify: Notify,
}

pub struct Tick(pub u64);

impl Tick {
    pub fn from(d: Duration) -> Self {
        Self((d.as_millis() / MS_PER_TICK as u128) as u64)
    }
}

impl TickManager {
    pub fn new() -> Self {
        Self {
            clock: AtomicU64::new(0),
            notify: Notify::new(),
        }
    }

    pub fn current_tick(&self) -> u64 {
        self.clock.load(std::sync::atomic::Ordering::Relaxed)
    }

    pub fn tick(&self) {
        self.clock
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn notify(&self) {
        self.notify.notify_one();
    }

    pub async fn wait(&self) {
        self.notify.notified().await;
    }

    pub async fn spawn_ticks(&self, state: &SharedState) -> Result<(), TickError> {
        let mob_respawn: MobRespawnTick = MobRespawnTick::new();
        mob_respawn.spawn(state).await?;
        Ok(())
    }
}
