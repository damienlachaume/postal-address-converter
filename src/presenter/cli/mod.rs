//! CLI module
//!
//! It contains the CLI for managing addresses.

mod commands;
mod formatter;
mod handlers;

pub use commands::*;
pub use formatter::*;
pub use handlers::*;
