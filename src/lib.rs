#![warn(missing_docs)]

//! # Postal Address Converter
//!
//! TODO

mod domain;
mod infrastructure;
mod use_cases;

pub use domain::*;
pub use infrastructure::*;
pub use use_cases::*;

/// Alias for [anyhow::Error]
pub type AnyhowError = anyhow::Error;

/// Alias for [anyhow::Result]
pub type AnyhowResult<T> = anyhow::Result<T, AnyhowError>;
