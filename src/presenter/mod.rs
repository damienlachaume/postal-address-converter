//! Presenter module
//!
//! It contains the presenter for the application (CLI).

pub mod cli;

pub use cli::*;

// EXTENSIBILITY NOTE: Adding a new presenter
// To add a REST API presenter:
// 1. Create a new module (e.g., `api/`) alongside the existing CLI module
// 2. Create necessary controllers/handlers that use the same use cases
// 3. Example:
//
// ```
// pub mod api {
//     use crate::AddressService;
//
//     pub struct ApiServer {
//         service: AddressService,
//     }
//
//     impl ApiServer {
//         pub fn new(service: AddressService) -> Self {
//             Self { service }
//         }
//
//         pub fn start(self, port: u16) -> AnyhowResult<()> {
//             // Initialize web framework like warp, axum, or actix-web
//             // Register endpoints that call the service methods
//         }
//     }
// }
// ```
