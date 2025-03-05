use anyhow::Context;
use uuid::Uuid;

use crate::{Address, AddressService, AnyhowResult, FrenchAddress, ISO20022Address};

use super::AddressFormat;

/// CLI handler for address operations
pub struct AddressHandler {
    service: AddressService,
}

impl AddressHandler {
    /// Create a new [AddressHandler]
    pub fn new(service: AddressService) -> Self {
        Self { service }
    }

    /// Get an address by id and format it accordingly.
    pub fn handle_get(&self, id: Uuid, address_format: Option<AddressFormat>) -> AnyhowResult<()> {
        let formatted_output = self.get(id, address_format)?;

        println!("{}", formatted_output);

        Ok(())
    }

    fn get(&self, id: Uuid, address_format: Option<AddressFormat>) -> AnyhowResult<String> {
        let address = self
            .service
            .get(id)?
            .ok_or(anyhow::anyhow!("Address not found"))?;

        let formatted_output = match address_format {
            Some(AddressFormat::French) => {
                let french: FrenchAddress = address.try_into()?;
                format!("id: {id}\n{french}")
            }
            Some(AddressFormat::Iso20022) => {
                let iso: ISO20022Address = address.into();
                format!("id: {id}\n{iso}")
            }
            None => serde_json::to_string_pretty(&address)?,
        };

        Ok(formatted_output)
    }

    /// List all addresses and format them accordingly.
    pub fn handle_list(&self, address_format: Option<AddressFormat>) -> AnyhowResult<()> {
        let formatted_output = self.list(address_format)?;

        println!("{}", formatted_output);

        Ok(())
    }

    fn list(&self, address_format: Option<AddressFormat>) -> AnyhowResult<String> {
        let addresses = self
            .service
            .list()
            .with_context(|| "Failed to list addresses")?;

        let formatted_output = match address_format {
            Some(AddressFormat::French) => {
                let addresses: Vec<String> = addresses
                    .iter()
                    .map(|(id, address)| {
                        address
                            .clone()
                            .try_into()
                            .map(|french: FrenchAddress| format!("id: {id}\n{french}"))
                    })
                    .collect::<Result<_, _>>()?;

                addresses.join("\n\n")
            }
            Some(AddressFormat::Iso20022) => {
                let addresses: Vec<String> = addresses
                    .iter()
                    .map(|(id, address)| {
                        let iso: ISO20022Address = address.clone().into();

                        Ok(format!("id: {id}\n{iso}"))
                    })
                    .collect::<Result<_, anyhow::Error>>()?;

                addresses.join("\n\n")
            }
            None => serde_json::to_string_pretty(&addresses)
                .with_context(|| "Failed to list addresses")?,
        };
        Ok(formatted_output)
    }

    /// Add a new address
    pub fn handle_add(&self, data: String) -> AnyhowResult<()> {
        let id = self.add(data)?;

        println!("Address added with id: {}", id);

        Ok(())
    }

    fn add(&self, data: String) -> AnyhowResult<Uuid> {
        let address: Address = serde_json::from_str(&data)?;

        let id = self.service.add(&address)?;

        Ok(id)
    }

    /// Update an address
    pub fn handle_update(&self, id: Uuid, data: String) -> AnyhowResult<()> {
        let address: Address = serde_json::from_str(&data)?;
        self.service
            .update(id, &address)
            .with_context(|| "Failed to update address")?;

        println!("Address updated with id: {}", id);

        Ok(())
    }

    /// Delete an address
    pub fn handle_delete(&self, id: Uuid) -> AnyhowResult<()> {
        self.service
            .delete(id)
            .with_context(|| "Failed to delete address")?;

        println!("Address deleted with id: {}", id);

        Ok(())
    }

    // /// Convert an address between formats
    // pub fn handle_convert(
    //     &self,
    //     data: String,
    //     from: AddressFormat,
    //     to: AddressFormat,
    // ) -> AnyhowResult<()> {
    //     if from == to {
    //         return Err(anyhow!("Source and target formats are the same"));
    //     }

    //     match (from, to) {
    //         (AddressFormat::French, AddressFormat::Iso20022) => {
    //             let address: FrenchAddress = serde_json::from_str(&data)?;
    //             let iso: ISO20022Address = AddressConverter::french_to_iso(address)?;

    //             println!("{}", iso);
    //         }
    //         (AddressFormat::Iso20022, AddressFormat::French) => {
    //             let address: ISO20022Address = serde_json::from_str(&data)?;
    //             let french: FrenchAddress = AddressConverter::iso_to_french(address)?;

    //             println!("{}", french);
    //         }
    //         _ => {
    //             return Err(anyhow!("Unsupported format conversion"));
    //         }
    //     }

    //     Ok(())
    // }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{Address, AddressRepository, InMemoryAddressRepository};

    use super::*;

    #[test]
    fn get_existing_address_with_french_format() {
        let repository = Arc::new(InMemoryAddressRepository::default());
        let address = Address::dummy();
        let id = repository.save(&address).unwrap();
        let service = AddressService::new(repository);
        let handler = AddressHandler::new(service);

        let output = handler.get(id, Some(AddressFormat::French)).unwrap();

        let expected_french = FrenchAddress::try_from(address.clone()).unwrap();
        assert!(output.contains(&id.to_string()));
        assert!(output.contains(&expected_french.to_string()));
    }

    #[test]
    fn get_existing_address_with_iso_format() {
        let repository = Arc::new(InMemoryAddressRepository::default());
        let address = Address::dummy();
        let id = repository.save(&address).unwrap();
        let service = AddressService::new(repository);
        let handler = AddressHandler::new(service);

        let output = handler.get(id, Some(AddressFormat::Iso20022)).unwrap();

        let expected_iso = ISO20022Address::from(address.clone());
        assert!(output.contains(&id.to_string()));
        assert!(output.contains(&expected_iso.to_string()));
    }

    #[test]
    fn get_existing_address_with_french_format_without_format() {
        let repository = Arc::new(InMemoryAddressRepository::default());
        let address = Address::dummy();
        let id = repository.save(&address).unwrap();
        let service = AddressService::new(repository);
        let handler = AddressHandler::new(service);

        let output = handler.get(id, None).unwrap();

        let expected_json = serde_json::to_string_pretty(&address).unwrap();
        assert!(output.contains(&expected_json));
    }

    #[test]
    fn get_non_existing_address_returns_error() {
        let service = AddressService::new(Arc::new(InMemoryAddressRepository::default()));
        let handler = AddressHandler::new(service);

        handler
            .handle_get(Uuid::new_v4(), None)
            .expect_err("Should return an error");
    }

    #[test]
    fn list_addresses_with_french_format() {
        let repository = Arc::new(InMemoryAddressRepository::default());
        let address = Address::dummy();
        let id = repository.save(&address).unwrap();
        let service = AddressService::new(repository);
        let handler = AddressHandler::new(service);

        let output = handler.list(Some(AddressFormat::French)).unwrap();

        let expected_french = FrenchAddress::try_from(address.clone()).unwrap();
        assert!(output.contains(&id.to_string()));
        assert!(output.contains(&expected_french.to_string()));
    }

    #[test]
    fn list_addresses_with_iso_format() {
        let repository = Arc::new(InMemoryAddressRepository::default());
        let address = Address::dummy();
        let id = repository.save(&address).unwrap();
        let service = AddressService::new(repository);
        let handler = AddressHandler::new(service);

        let output = handler.list(Some(AddressFormat::Iso20022)).unwrap();

        let expected_iso = ISO20022Address::from(address.clone());
        assert!(output.contains(&id.to_string()));
        assert!(output.contains(&expected_iso.to_string()));
    }

    #[test]
    fn add_stores_address() {
        let repository = Arc::new(InMemoryAddressRepository::default());
        let service = AddressService::new(repository.clone());
        let handler = AddressHandler::new(service);
        let address = r#"{
            "name": "Alice",
            "street_name": "123 Main St",
            "town_name": "Anytown",
            "post_code": "12345",
            "country": "FR"
        }"#;

        let id = handler.add(address.to_string()).unwrap();

        let address = repository.get(id).unwrap().unwrap();

        assert_eq!(address.name, Some("Alice".to_string()));
        assert_eq!(address.street_name, "123 Main St".to_string());
        assert_eq!(address.town_name, "Anytown".to_string());
        assert_eq!(address.post_code, "12345".to_string());
        assert_eq!(address.country, "FR".to_string());
    }

    #[test]
    fn add_returns_error_for_invalid_internal_address_format() {
        let repository = Arc::new(InMemoryAddressRepository::default());
        let service = AddressService::new(repository);
        let handler = AddressHandler::new(service);

        handler
            .add("invalid internal address format".to_string())
            .expect_err("Should return an error for invalid internal address format");
    }

    #[test]
    fn update_stores_new_address_values() {
        let repository = Arc::new(InMemoryAddressRepository::default());
        let service = AddressService::new(repository.clone());
        let handler = AddressHandler::new(service);

        let initial_address = Address {
            name: Some("Alice".to_string()),
            ..Address::dummy()
        };
        let id = repository.save(&initial_address).unwrap();

        let updated_address = Address {
            name: Some("Bob".to_string()),
            ..initial_address
        };
        let updated_address_json = serde_json::to_string(&updated_address).unwrap();

        handler.handle_update(id, updated_address_json).unwrap();

        let updated_address = repository.get(id).unwrap().unwrap();
        assert_eq!(updated_address, updated_address);
    }

    #[test]
    fn update_returns_error_for_invalid_internal_address_format() {
        let repository = Arc::new(InMemoryAddressRepository::default());
        let service = AddressService::new(repository.clone());
        let handler = AddressHandler::new(service);

        let initial_address = Address {
            name: Some("Alice".to_string()),
            ..Address::dummy()
        };
        let id = repository.save(&initial_address).unwrap();

        handler
            .handle_update(id, "invalid internal address format".to_string())
            .expect_err("Should return an error for invalid internal address format");
    }

    #[test]
    fn delete_removes_address() {
        let repository = Arc::new(InMemoryAddressRepository::default());
        let service = AddressService::new(repository.clone());
        let handler = AddressHandler::new(service);

        let address = Address::dummy();
        let id = repository.save(&address).unwrap();

        handler.handle_delete(id).unwrap();

        assert!(repository.get(id).unwrap().is_none());
    }
}
