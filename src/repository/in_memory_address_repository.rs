use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use anyhow::anyhow;
use uuid::Uuid;

use crate::{AnyhowResult, domain::Address, use_cases::AddressRepository};

/// Repository for managing addresses in memory
pub struct InMemoryAddressRepository {
    addresses: Arc<Mutex<HashMap<Uuid, Address>>>,
}

impl InMemoryAddressRepository {
    /// Create a new [InMemoryAddressRepository]
    pub fn new() -> Self {
        Self {
            addresses: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl Default for InMemoryAddressRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl AddressRepository for InMemoryAddressRepository {
    fn get(&self, id: Uuid) -> AnyhowResult<Option<Address>> {
        let addresses = self.addresses.lock().unwrap();

        Ok(addresses.get(&id).cloned())
    }

    fn list(&self) -> AnyhowResult<Vec<(Uuid, Address)>> {
        let addresses = self.addresses.lock().unwrap();

        Ok(addresses
            .iter()
            .map(|(id, address)| (*id, address.clone()))
            .collect())
    }

    fn save(&self, address: &Address) -> AnyhowResult<Uuid> {
        let new_id = Uuid::new_v4();
        let mut addresses = self.addresses.lock().unwrap();
        addresses.insert(new_id, address.clone());

        Ok(new_id)
    }

    fn update(&self, id: Uuid, address: &Address) -> AnyhowResult<()> {
        let mut addresses = self.addresses.lock().unwrap();

        if let Some(existing_address) = addresses.get_mut(&id) {
            *existing_address = address.clone();

            Ok(())
        } else {
            Err(anyhow!("Address with ID '{}' not found", id))
        }
    }
    fn delete(&self, id: Uuid) -> AnyhowResult<()> {
        let mut addresses = self.addresses.lock().unwrap();
        addresses.remove(&id);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_should_return_none_for_nonexistent_id() {
        let repository = InMemoryAddressRepository::new();

        let address = repository.get(Uuid::new_v4()).unwrap();

        assert!(address.is_none());
    }

    #[test]
    fn save_should_store_address() {
        let repository = InMemoryAddressRepository::new();
        let address = Address::dummy();

        assert!(repository.list().unwrap().is_empty());

        let id = repository.save(&address).unwrap();

        let retrieved_address = repository.get(id).unwrap();

        assert_eq!(retrieved_address.unwrap(), address);
    }

    #[test]
    fn list_should_return_all_addresses() {
        let repository = InMemoryAddressRepository::new();
        let address1 = Address::dummy();
        let address2 = Address::dummy();

        let id1 = repository.save(&address1).unwrap();
        let id2 = repository.save(&address2).unwrap();

        let addresses = repository.list().unwrap();

        assert_eq!(addresses.len(), 2);
        assert!(
            addresses
                .iter()
                .any(|(id, addr)| id == &id1 && addr == &address1)
        );
        assert!(
            addresses
                .iter()
                .any(|(id, addr)| id == &id2 && addr == &address2)
        );
    }

    #[test]
    fn update_should_update_existing_address() {
        let repository = InMemoryAddressRepository::new();
        let address = Address {
            country: "FR".to_string(),
            ..Address::dummy()
        };

        let id = repository.save(&address).unwrap();

        let updated_address = Address {
            country: "IT".to_string(),
            ..Address::dummy()
        };
        repository.update(id, &updated_address).unwrap();

        let retrieved_address = repository.get(id).unwrap();

        assert_eq!(retrieved_address.unwrap(), updated_address);
    }

    #[test]
    fn update_should_fail_for_nonexistent_id() {
        let repository = InMemoryAddressRepository::new();
        let address = Address::dummy();

        repository
            .update(Uuid::new_v4(), &address)
            .expect_err("Update should fail for nonexistent ID");
    }

    #[test]
    fn delete_should_remove_address() {
        let repository = InMemoryAddressRepository::new();
        let address = Address::dummy();
        let id = repository.save(&address).unwrap();

        repository.delete(id).unwrap();

        let retrieved_address = repository.get(id).unwrap();

        assert!(retrieved_address.is_none());
    }
}
