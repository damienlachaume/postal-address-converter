mod file_address_repository;
mod in_memory_address_repository;

pub use file_address_repository::*;
#[cfg(test)]
pub use in_memory_address_repository::*;

// EXTENSIBILITY NOTE: Adding a new repository implementation
// To add a database repository:
// 1. Create a new module (e.g., `db_address_repository.rs`) in this directory
// 2. Implement the `AddressRepository` trait for your database
// 3. Example:
//
// ```
// pub struct DatabaseAddressRepository {
//     connection_pool: ConnectionPool,
// }
//
// impl AddressRepository for DatabaseAddressRepository {
//     fn get(&self, id: Uuid) -> AnyhowResult<Option<Address>> {
//         // Database-specific implementation here
//     }
//
//     // Implement other required methods...
// }
// ```
