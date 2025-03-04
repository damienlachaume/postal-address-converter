use std::sync::Arc;

use uuid::Uuid;

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
    pub fn get(&self, id: Uuid) -> AnyhowResult<Option<Address>> {
        self.repository.get(id)
    }

    /// List all addresses
    pub fn list(&self) -> AnyhowResult<Vec<(Uuid, Address)>> {
        self.repository.list()
    }

    /// Add a new address
    pub fn add(&self, address: &Address) -> AnyhowResult<Uuid> {
        self.repository.save(address)
    }

    /// Update an address
    pub fn update(&self, id: Uuid, address: &Address) -> AnyhowResult<()> {
        self.repository.update(id, address)
    }

    /// Delete an address
    pub fn delete(&self, id: Uuid) -> AnyhowResult<()> {
        self.repository.delete(id)
    }
}
