mod btree;

mod ch;
mod closure;
mod col;
mod dyn_;
mod future;
mod hash_map_usage;
mod itr;
mod mem_leak;
mod mini_tokio;
mod mini_tokio_old;
mod mpsc;

mod onebrc;

mod panik;
mod partial_ord;
mod reference_tests;

mod std_test;

mod thread;

mod unsafer;
mod vector;

pub use future::Delay;

pub use hash_map_usage::process_or_default;
