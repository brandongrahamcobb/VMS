use std::sync::atomic::{AtomicI32, Ordering};

static CLIENT_ID_COUNTER: AtomicI32 = AtomicI32::new(0);

pub fn next_client_id() -> i32 {
    CLIENT_ID_COUNTER.fetch_add(1, Ordering::Relaxed)
}
