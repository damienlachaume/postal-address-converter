use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use anyhow::anyhow;

use crate::{AnyhowResult, domain::Address};

use super::AddressRepository;

pub struct InMemoryAddressRepository {
    addresses: Arc<Mutex<HashMap<String, Address>>>,
}

impl InMemoryAddressRepository {
    pub fn new() -> Self {
        Self {
            addresses: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl AddressRepository for InMemoryAddressRepository {
    fn get(&self, id: &str) -> AnyhowResult<Option<Address>> {
        let addresses = self.addresses.lock().unwrap();

        Ok(addresses.get(id).cloned())
    }

    fn list(&self) -> AnyhowResult<Vec<(String, Address)>> {
        let addresses = self.addresses.lock().unwrap();

        Ok(addresses
            .iter()
            .map(|(id, address)| (id.clone(), address.clone()))
            .collect())
    }

    fn save(&self, id: &str, address: &Address) -> AnyhowResult<()> {
        let mut addresses = self.addresses.lock().unwrap();
        addresses.insert(id.to_string(), address.clone());

        Ok(())
    }

    fn update(&self, id: &str, address: &Address) -> AnyhowResult<()> {
        let mut addresses = self.addresses.lock().unwrap();

        if addresses.contains_key(id) {
            addresses.insert(id.to_string(), address.clone());
            Ok(())
        } else {
            Err(anyhow!("Address with ID '{}' not found", id))
        }
    }
    fn delete(&self, id: &str) -> AnyhowResult<()> {
        let mut addresses = self.addresses.lock().unwrap();
        addresses.remove(id);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_should_return_none_for_nonexistent_id() {
        let repository = InMemoryAddressRepository::new();

        let address = repository.get("nonexistent-id").unwrap();

        assert!(address.is_none());
    }

    #[test]
    fn save_should_store_address() {
        let repository = InMemoryAddressRepository::new();
        let address = Address::dummy();
        let id = "id-1";

        assert!(repository.get(id).unwrap().is_none());

        repository.save(id, &address).unwrap();

        let retrieved_address = repository.get(id).unwrap();

        assert_eq!(retrieved_address.unwrap(), address);
    }

    #[test]
    fn save_should_update_existing_address() {
        let repository = InMemoryAddressRepository::new();
        let address1 = Address {
            country: "FR".to_string(),
            ..Address::dummy()
        };
        let address2 = Address {
            country: "IT".to_string(),
            ..Address::dummy()
        };
        let id = "id-1";

        repository.save(id, &address1).unwrap();
        repository.save(id, &address2).unwrap();

        let retrieved_address = repository.get(id).unwrap();

        assert_eq!(retrieved_address.unwrap(), address2);
    }

    #[test]
    fn list_should_return_all_addresses() {
        let repository = InMemoryAddressRepository::new();
        let address1 = Address::dummy();
        let address2 = Address::dummy();

        repository.save("id-1", &address1).unwrap();
        repository.save("id-2", &address2).unwrap();

        let addresses = repository.list().unwrap();

        assert_eq!(addresses.len(), 2);
        assert!(
            addresses
                .iter()
                .any(|(id, addr)| id == "id-1" && addr == &address1)
        );
        assert!(
            addresses
                .iter()
                .any(|(id, addr)| id == "id-2" && addr == &address2)
        );
    }

    #[test]
    fn update_should_update_existing_address() {
        let repository = InMemoryAddressRepository::new();
        let address = Address {
            country: "FR".to_string(),
            ..Address::dummy()
        };
        let id = "id-1";

        repository.save(id, &address).unwrap();

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
            .update("nonexistent-id", &address)
            .expect_err("Update should fail for nonexistent ID");
    }

    #[test]
    fn delete_should_remove_address() {
        let repository = InMemoryAddressRepository::new();
        let address = Address::dummy();
        let id = "id-1";
        repository.save(id, &address).unwrap();

        repository.delete(id).unwrap();

        let retrieved_address = repository.get(id).unwrap();

        assert!(retrieved_address.is_none());
    }
}
