use std::sync::Arc;

use crate::{AnyhowResult, domain::Address, use_cases::repository::AddressRepository};

/// Address service
pub struct AddressService {
    repository: Arc<dyn AddressRepository>,
}

impl AddressService {
    /// Create a new [AddressService]
    pub fn new(repository: Arc<dyn AddressRepository>) -> Self {
        Self { repository }
    }

    /// Get an address by id
    pub fn get(&self, id: &str) -> AnyhowResult<Option<Address>> {
        self.repository.get(id)
    }

    /// List all addresses
    pub fn list(&self) -> AnyhowResult<Vec<(String, Address)>> {
        self.repository.list()
    }

    /// Add a new address
    pub fn add(&self, id: &str, address: &Address) -> AnyhowResult<()> {
        self.repository.save(id, address)
    }

    /// Update an address
    pub fn update(&self, id: &str, address: &Address) -> AnyhowResult<()> {
        self.repository.update(id, address)
    }

    /// Delete an address
    pub fn delete(&self, id: &str) -> AnyhowResult<()> {
        self.repository.delete(id)
    }
}
