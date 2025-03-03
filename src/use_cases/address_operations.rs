use std::sync::Arc;

use crate::{AnyhowResult, domain::Address, use_cases::repository::AddressRepository};

pub fn get_address(
    repository: Arc<dyn AddressRepository>,
    id: &str,
) -> AnyhowResult<Option<Address>> {
    repository.get(id)
}

pub fn list_addresses(
    repository: Arc<dyn AddressRepository>,
) -> AnyhowResult<Vec<(String, Address)>> {
    repository.list()
}

pub fn add_address(
    repository: Arc<dyn AddressRepository>,
    id: &str,
    address: &Address,
) -> AnyhowResult<()> {
    repository.save(id, address)
}

pub fn update_address(
    repository: Arc<dyn AddressRepository>,
    id: &str,
    address: &Address,
) -> AnyhowResult<()> {
    repository.update(id, address)
}

pub fn delete_address(repository: Arc<dyn AddressRepository>, id: &str) -> AnyhowResult<()> {
    repository.delete(id)
}
