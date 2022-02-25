#![allow(dead_code)]
use std::collections::{BTreeMap, HashMap};
use std::sync::{Arc, Mutex};

use bytes::Bytes;
use tokio::sync::{broadcast, Notify};
use tokio::time::{Duration, Instant};
use tracing::debug;

#[derive(Debug)]
pub(crate) struct DbDropGuard {
    db: Db,
}

#[derive(Debug, Clone)]
pub(crate) struct Db {
    shared: Arc<Shared>,
}

#[derive(Debug)]
struct Shared {
    state: Mutex<State>,
    background_task: Notify,
}

#[derive(Debug)]
struct State {
    entries: HashMap<String, Entry>,
    pub_sub: HashMap<String, broadcast::Sender<Bytes>>,
    expirations: BTreeMap<(Instant, u64), String>,
    next_id: u64,
    shutdown: bool,
}

#[derive(Debug)]
struct Entry {
    id: u64,
    data: Bytes,
    expires_at: Option<Instant>,
}

impl DbDropGuard {
    pub(crate) fn new() -> DbDropGuard {
        DbDropGuard { db: Db::new() }
    }
    pub(crate) fn db(&self) -> Db {
        self.db.clone()
    }
}

impl Drop for DbDropGuard {
    fn drop(&mut self) {
        self.db.shutdown_purge_task();
    }
}

impl Db {
    pub(crate) fn new() -> Db {
        let shared = Arc::new(Shared {
            state: Mutex::new(State {
                entries: HashMap::new(),
                pub_sub: HashMap::new(),
                expirations: BTreeMap::new(),
                next_id: 0,
                shutdown: false,
            }),
            background_task: Notify::new(),
        });
        // background task for purge
        tokio::spawn(purge_expired_task(shared.clone()));

        Db { shared }
    }

    pub(crate) fn get(&self, key: &str) -> Option<Bytes> {
        let state = self.shared.state.lock().unwrap();
        state.entries.get(key).map(|entry| entry.data.clone())
    }

    pub(crate) fn set(&self, key: String, val: Bytes, expire: Option<Duration>) {
        let mut state = self.shared.state.lock().unwrap();
        let id = state.next_id;
        state.next_id += 1;

        let mut notify = false;

        let expires_at = expire.map(|duration| {
            let when = Instant::now() + duration;
            notify = state.next_expiration().map(|ex| ex > when).unwrap_or(true);
            state.expirations.insert((when, id), key.clone());
            when
        });

        let prev = state.entries.insert(
            key,
            Entry {
                id,
                data: val,
                expires_at,
            },
        );

        if let Some(prev) = prev {
            if let Some(when) = prev.expires_at {
                state.expirations.remove(&(when, prev.id));
            }
        }

        drop(state);

        if notify {
            self.shared.background_task.notify_one();
        }
    }

    fn shutdown_purge_task(&self) {
        let mut state = self.shared.state.lock().unwrap();
        state.shutdown = true;
        drop(state);
        self.shared.background_task.notify_one();
    }
}

impl Shared {
    fn purge_expired_keys(&self) -> Option<Instant> {
        if self.is_shutdown() {
            return None;
        }
        let mut state = self.state.lock().unwrap();

        let state = &mut *state;

        let now = Instant::now();
        while let Some((&(when, id), key)) = state.expirations.iter().next() {
            if when > now {
                return Some(when);
            }
            state.entries.remove(key);
            state.expirations.remove(&(when, id));
        }
        None
    }

    fn is_shutdown(&self) -> bool {
        self.state.lock().unwrap().shutdown
    }
}

impl State {
    fn next_expiration(&self) -> Option<Instant> {
        self.expirations.keys().next().map(|ex| ex.0)
    }
}

async fn purge_expired_task(shared: Arc<Shared>) {
    while !shared.is_shutdown() {
        if let Some(when) = shared.purge_expired_keys() {
            tokio::select! {
                _ = tokio::time::sleep_until(when) => {}
                _ = shared.background_task.notified() => {}
            }
        } else {
            shared.background_task.notified().await;
        }
    }
    debug!("Purge background task shut down")
}
