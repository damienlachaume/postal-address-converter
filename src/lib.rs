#![warn(missing_docs)]

//! # Postal Address Converter
//!
//! A CLI tool for converting postal addresses between the French format (NF Z10-011) and ISO 20022.
//!
//! ## Modules
//!
//! - **domain**: Defines the core `Address` model and conversion logic.
//! - **presenter**: Provides the CLI interface.
//! - **repository**: Implements persistence (in-memory and file-based).
//! - **use_cases**: Contains application-specific operations.
//!
//! ## Error Handling
//!
//! Uses [`anyhow`](https://crates.io/crates/anyhow) for error management with these aliases:
//!
//! - `AnyhowError`: alias for `anyhow::Error`
//! - `AnyhowResult<T>`: alias for `anyhow::Result<T, AnyhowError>`
//!
//! ## Getting Started
//!
//! For more details on installation, usage (including CLI commands), testing,
//! please refer to the project's README file.
//!

mod domain;
pub mod presenter;
mod repository;
mod use_cases;

pub use domain::*;
pub use presenter::*;
pub use repository::*;
pub use use_cases::*;

/// Alias for [anyhow::Error]
pub type AnyhowError = anyhow::Error;

/// Alias for [anyhow::Result]
pub type AnyhowResult<T> = anyhow::Result<T, AnyhowError>;
