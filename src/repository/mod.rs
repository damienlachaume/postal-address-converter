use crate::{AnyhowResult, domain::Address};

pub trait AddressRepository {
    fn get(&self, id: &str) -> AnyhowResult<Option<Address>>;
    fn list(&self) -> AnyhowResult<Vec<(String, Address)>>;
    fn save(&self, id: &str, address: &Address) -> AnyhowResult<()>;
    fn update(&self, id: &str, address: &Address) -> AnyhowResult<()>;
    fn delete(&self, id: &str) -> AnyhowResult<()>;
}

mod in_memory_address_repository;

#[cfg(test)]
pub use in_memory_address_repository::*;
