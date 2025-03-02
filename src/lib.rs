#![warn(missing_docs)]

//! # Postal Address Converter
//!
//! TODO

mod domain;

pub use domain::Address;

/// Alias for [anyhow::Error]
pub type AnyhowError = anyhow::Error;

/// Alias for [anyhow::Result]
pub type AnyhowResult<T> = anyhow::Result<T, AnyhowError>;
