mod file_address_repository;
mod in_memory_address_repository;

pub use file_address_repository::*;
#[cfg(test)]
pub use in_memory_address_repository::*;
