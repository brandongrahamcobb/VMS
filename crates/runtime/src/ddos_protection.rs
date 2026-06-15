/* runtime/src/ip_guard.rs
 * The purpose of this module is to provide the run and execution logic of the relay.
 *
 * Copyright (C) 2026  https://github.com/brandongrahamcobb/VMS.gitrand::
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use std::collections::HashMap;
use std::collections::HashSet;
use std::net::IpAddr;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

const MAX_ATTEMPTS: u32 = 5;
const WINDOW: Duration = Duration::from_secs(10);
const BAN_DURATION: Duration = Duration::from_secs(60);

pub struct IpEntry {
    count: u32,
    window_start: Instant,
    banned_until: Option<Instant>,
}

pub type RateLimiter = Arc<Mutex<HashMap<IpAddr, IpEntry>>>;

pub fn check_rate_limit(limiter: &RateLimiter, ip: IpAddr) -> bool {
    let mut map = limiter.lock().unwrap();
    let now = Instant::now();
    let entry = map.entry(ip).or_insert_with(|| IpEntry {
        count: 0,
        window_start: now,
        banned_until: None,
    });
    if let Some(banned_until) = entry.banned_until {
        if now < banned_until {
            return false;
        }
        entry.banned_until = None;
        entry.count = 0;
        entry.window_start = now;
    }
    if now.duration_since(entry.window_start) > WINDOW {
        entry.window_start = now;
        entry.count = 0;
    }
    entry.count += 1;
    if entry.count > MAX_ATTEMPTS {
        entry.banned_until = Some(now + BAN_DURATION);
        return false;
    }

    true
}

pub fn sweep(limiter: &RateLimiter) {
    let mut map = limiter.lock().unwrap();
    let now = Instant::now();
    map.retain(|_, e| {
        e.banned_until.map_or(true, |b| now < b) || now.duration_since(e.window_start) < WINDOW * 2
    });
}

pub type ConnectedIps = Arc<Mutex<HashSet<IpAddr>>>;

pub struct IpGuard {
    pub ip: IpAddr,
    pub set: ConnectedIps,
}

impl Drop for IpGuard {
    fn drop(&mut self) {
        self.set.lock().unwrap().remove(&self.ip);
    }
}
