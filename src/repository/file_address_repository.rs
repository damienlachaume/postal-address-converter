use std::{
    collections::HashMap,
    fs::{self, File},
    io::{Read, Write},
    path::{Path, PathBuf},
    sync::{Arc, RwLock},
};

use anyhow::{Context, anyhow};
use serde::{Deserialize, Serialize};

use crate::{AnyhowResult, domain::Address};

use super::AddressRepository;

#[derive(Serialize, Deserialize, Default)]
struct AddressesData {
    addresses: HashMap<String, Address>,
}

pub struct FileAddressRepository {
    file_path: PathBuf,
    cache: Arc<RwLock<HashMap<String, Address>>>,
}

impl FileAddressRepository {
    pub fn new<P: AsRef<Path>>(file_path: P) -> AnyhowResult<Self> {
        let file_path = file_path.as_ref().to_path_buf();

        if !file_path.exists() {
            let empty_data = AddressesData::default();
            let json = serde_json::to_string_pretty(&empty_data)?;
            fs::write(&file_path, json)?;
        }

        let mut file = File::open(&file_path)
            .with_context(|| format!("Failed to open file at {:?}", file_path))?;

        let mut content = String::new();
        file.read_to_string(&mut content)
            .with_context(|| format!("Failed to read content from file at {:?}", file_path))?;

        let data: AddressesData = if content.trim().is_empty() {
            AddressesData::default()
        } else {
            serde_json::from_str(&content)
                .with_context(|| format!("Failed to parse JSON from file at {:?}", file_path))?
        };

        Ok(Self {
            file_path,
            cache: Arc::new(RwLock::new(data.addresses)),
        })
    }

    fn persist_cache(&self) -> AnyhowResult<()> {
        let cache = self.cache.read().unwrap();
        let data = AddressesData {
            addresses: cache.clone(),
        };

        let json = serde_json::to_string_pretty(&data)?;

        let mut file = File::create(&self.file_path)
            .with_context(|| format!("Failed to create file at {:?}", self.file_path))?;

        file.write_all(json.as_bytes())
            .with_context(|| format!("Failed to write to file at {:?}", self.file_path))?;

        Ok(())
    }
}

impl AddressRepository for FileAddressRepository {
    fn get(&self, id: &str) -> AnyhowResult<Option<Address>> {
        let cache = self.cache.read().unwrap();

        Ok(cache.get(id).cloned())
    }

    fn list(&self) -> AnyhowResult<Vec<(String, Address)>> {
        let cache = self.cache.read().unwrap();

        Ok(cache
            .iter()
            .map(|(id, address)| (id.clone(), address.clone()))
            .collect())
    }

    fn save(&self, id: &str, address: &Address) -> AnyhowResult<()> {
        {
            // Use a scope block to ensure the write lock is released before the I/O operation (potentially slow)
            let mut cache = self.cache.write().unwrap();
            cache.insert(id.to_string(), address.clone());
        }

        self.persist_cache()?;

        Ok(())
    }

    fn update(&self, id: &str, address: &Address) -> AnyhowResult<()> {
        {
            // Use a scope block to ensure the write lock is released before the I/O operation (potentially slow)
            let mut cache = self.cache.write().unwrap();

            if !cache.contains_key(id) {
                return Err(anyhow!("Address with ID '{}' not found", id));
            }

            cache.insert(id.to_string(), address.clone());
        }

        self.persist_cache()?;

        Ok(())
    }

    fn delete(&self, id: &str) -> AnyhowResult<()> {
        {
            // Use a scope block to ensure the write lock is released before the I/O operation (potentially slow)
            let mut cache = self.cache.write().unwrap();
            cache.remove(id);
        }

        self.persist_cache()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_temp_dir(sub_dir: &str) -> PathBuf {
        let temp_dir = std::env::temp_dir();
        let dir = temp_dir.join("file_address_repository").join(sub_dir);

        if dir.exists() {
            fs::remove_dir_all(&dir).unwrap();
        }

        fs::create_dir_all(&dir).unwrap();

        dir
    }

    #[test]
    fn new_repository_should_create_valid_empty_json_file() {
        let target_directory = create_temp_dir("new_empty_file");
        let file_path = target_directory.join("addresses.json");

        assert!(!file_path.exists());

        FileAddressRepository::new(&file_path).unwrap();

        assert!(file_path.exists());

        let content = fs::read_to_string(&file_path).unwrap();
        let data: AddressesData = serde_json::from_str(&content).unwrap();
        assert!(data.addresses.is_empty());
    }

    #[test]
    fn new_repository_should_load_existing_file_content() {
        let target_directory = create_temp_dir("new_existing_file");
        let file_path = target_directory.join("addresses.json");
        let id = "existing-id";
        let address = Address::dummy();
        {
            let mut addresses = HashMap::new();
            addresses.insert(id.to_string(), address.clone());
            let initial_data = AddressesData { addresses };

            let json = serde_json::to_string_pretty(&initial_data).unwrap();
            fs::write(&file_path, json).unwrap();
        }

        let repository = FileAddressRepository::new(&file_path).unwrap();

        let loaded_address = repository.get(id).unwrap();
        assert_eq!(loaded_address.unwrap(), address);
    }

    #[test]
    fn list_should_return_empty_collection_when_no_addresses() {
        let target_directory = create_temp_dir("list_empty");
        let file_path = target_directory.join("addresses.json");
        let repository = FileAddressRepository::new(&file_path).unwrap();

        let addresses = repository.list().unwrap();

        assert!(addresses.is_empty());
    }

    #[test]
    fn list_should_return_all_stored_addresses() {
        let target_directory = create_temp_dir("list_addresses");
        let file_path = target_directory.join("addresses.json");

        let repository = FileAddressRepository::new(&file_path).unwrap();
        let address1 = Address {
            country: "FR".to_string(),
            ..Address::dummy()
        };
        let address2 = Address {
            country: "IT".to_string(),
            ..Address::dummy()
        };
        repository.save("id1", &address1).unwrap();
        repository.save("id2", &address2).unwrap();

        let addresses = repository.list().unwrap();

        assert_eq!(addresses.len(), 2);
        assert!(
            addresses
                .iter()
                .any(|(id, addr)| id == "id1" && addr == &address1)
        );
        assert!(
            addresses
                .iter()
                .any(|(id, addr)| id == "id2" && addr == &address2)
        );
    }

    #[test]
    fn save_should_store_address_in_memory_cache() {
        let target_directory = create_temp_dir("save_memory_cache");
        let file_path = target_directory.join("addresses.json");
        let repository = FileAddressRepository::new(&file_path).unwrap();
        let id = "id-1";
        let address = Address::dummy();

        assert!(repository.get(id).unwrap().is_none());

        repository.save(id, &address).unwrap();

        let retrieved_address = repository.get(id).unwrap();
        assert_eq!(retrieved_address.unwrap(), address);
    }

    #[test]
    fn save_should_persist_address_to_file() {
        let target_directory = create_temp_dir("save_persist_file");
        let file_path = target_directory.join("addresses.json");
        let repository = FileAddressRepository::new(&file_path).unwrap();
        let id = "id-1";
        let address = Address::dummy();
        let initial_content = fs::read_to_string(&file_path).unwrap();
        let initial_data: AddressesData = serde_json::from_str(&initial_content).unwrap();

        assert!(!initial_data.addresses.contains_key(id));

        repository.save(id, &address).unwrap();

        let content = fs::read_to_string(&file_path).unwrap();
        let data: AddressesData = serde_json::from_str(&content).unwrap();
        assert_eq!(data.addresses.get(id).unwrap(), &address);
    }

    #[test]
    fn update_should_update_address_in_memory_cache() {
        let target_directory = create_temp_dir("update_memory_cache");
        let file_path = target_directory.join("addresses.json");
        let repository = FileAddressRepository::new(&file_path).unwrap();

        let id = "id-1";
        let initial_address = Address {
            country: "FR".to_string(),
            ..Address::dummy()
        };
        let updated_address = Address {
            country: "IT".to_string(),
            ..initial_address.clone()
        };
        repository.save(id, &initial_address).unwrap();

        repository.update(id, &updated_address).unwrap();

        let retrieved = repository.get(id).unwrap();
        assert_eq!(retrieved.unwrap(), updated_address);
    }

    #[test]
    fn update_should_persist_address_to_file() {
        let target_directory = create_temp_dir("update_persist_file");
        let file_path = target_directory.join("addresses.json");
        let repository = FileAddressRepository::new(&file_path).unwrap();

        let id = "id-1";
        let initial_address = Address {
            country: "FR".to_string(),
            ..Address::dummy()
        };
        let updated_address = Address {
            country: "IT".to_string(),
            ..initial_address.clone()
        };
        repository.save(id, &initial_address).unwrap();

        repository.update(id, &updated_address).unwrap();

        let content = fs::read_to_string(&file_path).unwrap();
        let data: AddressesData = serde_json::from_str(&content).unwrap();
        assert_eq!(data.addresses.get(id).unwrap(), &updated_address);
    }

    #[test]
    fn update_should_fail_for_nonexistent_id() {
        let target_directory = create_temp_dir("update_nonexistent");
        let file_path = target_directory.join("addresses.json");
        let repository = FileAddressRepository::new(&file_path).unwrap();

        repository
            .update("nonexistent-id", &Address::dummy())
            .expect_err("Should fail for nonexistent id");
    }

    #[test]
    fn delete_should_remove_address_from_memory_cache() {
        let target_directory = create_temp_dir("delete_memory");
        let file_path = target_directory.join("addresses.json");

        let repository = FileAddressRepository::new(&file_path).unwrap();
        let id = "id-1";
        repository.save(id, &Address::dummy()).unwrap();

        repository.delete(id).unwrap();

        let retrieved_address = repository.get(id).unwrap();
        assert!(retrieved_address.is_none());
    }

    #[test]
    fn delete_should_remove_address_from_file() {
        let target_directory = create_temp_dir("delete_file");
        let file_path = target_directory.join("addresses.json");

        let repository = FileAddressRepository::new(&file_path).unwrap();
        let id = "id-1";
        repository.save(id, &Address::dummy()).unwrap();

        repository.delete(id).unwrap();

        let content = fs::read_to_string(&file_path).unwrap();
        let data: AddressesData = serde_json::from_str(&content).unwrap();
        assert!(!data.addresses.contains_key(id));
    }

    #[test]
    fn delete_should_not_fail_for_nonexistent_id() {
        let target_directory = create_temp_dir("delete_nonexistent");
        let file_path = target_directory.join("addresses.json");
        let repository = FileAddressRepository::new(&file_path).unwrap();

        repository
            .delete("nonexistent-id")
            .expect("Deleting non-existent ID should not fail");
    }
}
