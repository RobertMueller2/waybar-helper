//! args-helper
//!
//! Tiny helper for peeling off the executable name, an optional “command”
//! and any remaining arguments from your `env::args()`.
pub mod args_helper;
pub use args_helper::ExecutableArgs;
