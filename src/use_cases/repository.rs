use crate::{Address, AnyhowResult};

/// Repository for managing addresses
pub trait AddressRepository {
    /// Get an address by ID
    fn get(&self, id: &str) -> AnyhowResult<Option<Address>>;

    /// List all addresses
    fn list(&self) -> AnyhowResult<Vec<(String, Address)>>;

    /// Save an address
    fn save(&self, id: &str, address: &Address) -> AnyhowResult<()>;

    /// Update an address
    fn update(&self, id: &str, address: &Address) -> AnyhowResult<()>;

    /// Delete an address
    fn delete(&self, id: &str) -> AnyhowResult<()>;
}
