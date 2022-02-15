pub mod utils;

/// The current version of `mdbook`.
///
/// This is provided as a way for custom preprocessors and renderers to do
/// compatibility checks.
pub const MDBOOK_VERSION: &str = env!("CARGO_PKG_VERSION");

/// The error types used through out this crate.
pub mod errors {
    pub(crate) use anyhow::{bail, Context, ensure};
    pub use anyhow::{Error, Result};
}
