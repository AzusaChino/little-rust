use std::cell::Cell;
use std::sync::{Arc, Mutex};

// crate 相对于lib.rs
use crate::exercise::Mode;

#[derive(Debug)]
pub struct Connection {}

impl Default for Connection {
    fn default() -> Self {
        Self {}
    }
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct Client {
    // crate zone
    pub(crate) conn: Arc<Mutex<Connection>>,
    pub(crate) txn: Cell<Option<(u64, Mode)>>,
}

impl Default for Client {
    fn default() -> Self {
        Self {
            conn: Arc::new(Mutex::new(Connection::default())),
            txn: Cell::new(None),
        }
    }
}
