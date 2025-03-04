use anyhow::{Context, Result};
use uuid::Uuid;

use crate::{AddressService, FrenchAddress, ISO20022Address};

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

    /// Get an address by id
    pub fn get(&self, id: Uuid, address_format: Option<AddressFormat>) -> Result<()> {
        let address = self
            .service
            .get(id)?
            .ok_or(anyhow::anyhow!("Address not found"))?;

        let address_json = match address_format {
            Some(AddressFormat::French) => {
                let address: FrenchAddress = address.try_into()?;

                serde_json::to_string_pretty(&address)?
            }
            Some(AddressFormat::Iso20022) => {
                let address: ISO20022Address = address.into();

                serde_json::to_string_pretty(&address)?
            }
            None => serde_json::to_string_pretty(&address)?,
        };

        println!("{:?}", address_json);

        Ok(())
    }

    /// List all addresses
    pub fn list(&self, address_format: Option<AddressFormat>) -> Result<()> {
        let addresses = self
            .service
            .list()
            .with_context(|| "Failed to list addresses")?;

        let addresses_json = match address_format {
            Some(AddressFormat::French) => {
                let addresses: Vec<(Uuid, FrenchAddress)> = addresses
                    .iter()
                    .map(|(id, address)| {
                        address
                            .clone()
                            .try_into()
                            .map(|french_address| (*id, french_address))
                    })
                    .collect::<Result<_, _>>()?;

                serde_json::to_vec_pretty(&addresses)?
            }
            Some(AddressFormat::Iso20022) => {
                let addresses: Vec<(Uuid, ISO20022Address)> = addresses
                    .iter()
                    .map(|(id, address)| Ok((*id, address.clone().into())))
                    .collect::<Result<_, anyhow::Error>>()?;

                serde_json::to_vec_pretty(&addresses)?
            }
            None => serde_json::to_vec_pretty(&addresses)?,
        };

        println!("{:?}", addresses_json);

        Ok(())
    }

    /// Add a new address
    pub fn add(&self, address_format: AddressFormat, data: String) -> Result<()> {
        let address = match address_format {
            AddressFormat::French => {
                let address: FrenchAddress = serde_json::from_str(&data)?;

                address.try_into()?
            }
            AddressFormat::Iso20022 => {
                let address: ISO20022Address = serde_json::from_str(&data)?;

                address.try_into()?
            }
        };

        let id = self.service.add(&address)?;

        println!("Address added with id: {}", id);

        Ok(())
    }

    /// Update an address
    pub fn update(&self, address_format: AddressFormat, id: Uuid, data: String) -> Result<()> {
        let address = match address_format {
            AddressFormat::French => {
                let address: FrenchAddress = serde_json::from_str(&data)?;

                address.try_into()?
            }
            AddressFormat::Iso20022 => {
                let address: ISO20022Address = serde_json::from_str(&data)?;

                address.try_into()?
            }
        };

        self.service
            .update(id, &address)
            .with_context(|| "Failed to update address")?;

        println!("Address updated with id: {}", id);

        Ok(())
    }

    /// Delete an address
    pub fn delete(&self, id: Uuid) -> Result<()> {
        self.service
            .delete(id)
            .with_context(|| "Failed to delete address")?;

        println!("Address deleted with id: {}", id);

        Ok(())
    }
}
