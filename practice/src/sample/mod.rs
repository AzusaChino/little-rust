mod car;
mod col;
mod hash_map_usage;
mod reference_tests;

// only expose one struct
pub use car::{Age, Car, Color, Transmission};
pub use hash_map_usage::process_or_default;
