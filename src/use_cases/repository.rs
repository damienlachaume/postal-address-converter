use uuid::Uuid;

use crate::{Address, AnyhowResult};

/// Repository for managing addresses
pub trait AddressRepository {
    /// Get an address by ID
    fn get(&self, id: Uuid) -> AnyhowResult<Option<Address>>;

    /// List all addresses
    fn list(&self) -> AnyhowResult<Vec<(Uuid, Address)>>;

    /// Save an address
    fn save(&self, address: &Address) -> AnyhowResult<Uuid>;

    /// Update an address
    fn update(&self, id: Uuid, address: &Address) -> AnyhowResult<()>;

    /// Delete an address
    fn delete(&self, id: Uuid) -> AnyhowResult<()>;
}
