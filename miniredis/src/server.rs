use std::sync::Arc;

use tokio::{net::TcpListener, sync::Semaphore};

use crate::db::Db;

/// Server Listener state. Created in the `run` call. It includes a `run` method
/// which performs the TCP listening and initialization of per-connection state
#[derive(Debug)]
struct Listener {
    /// shared database handle
    ///
    /// Contains the key / value store as well as the broadcast channels for pub/sub
    ///
    /// This holds a wrapper around an `Arc`. The internal `Db` can be retrieved
    /// and passed into the per connection state (`Handler`)
    db_holder: String,

    /// Tcp Listener supplied by the `run` caller
    listener: TcpListener,

    /// Limit the max number of connections.
    ///
    /// A `Semaphore` is used to limit the max number of connections. Before
    /// attempting to accept a new connection, a permit is acquired from the
    /// semaphore. If none are available, the listener waits for one.
    ///
    /// When handlers complete processing a connection, the permit is returned
    /// to the semaphore.
    limit_connections: Arc<Semaphore>,
}
