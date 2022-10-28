use std::cell::Cell;
use std::sync::{Arc, Mutex};

// crate 相对于lib.rs
use crate::exercise::Mode;

pub struct Connection {}

#[derive(Clone)]
pub struct Client {
    // crate zone
    pub(crate) conn: Arc<Mutex<Connection>>,
    pub(crate) txn: Cell<Option<(u64, Mode)>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn main() {
        let _client = Client {
            conn: Arc::new(Mutex::new(Connection {})),
            txn: Cell::new(None),
        };
    }
}
