use anyhow::{Context, Result, anyhow};
use uuid::Uuid;

use crate::{AddressConverter, AddressService, AnyhowResult, FrenchAddress, ISO20022Address};

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
    pub fn get(&self, id: Uuid, address_format: Option<AddressFormat>) -> AnyhowResult<()> {
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

        println!("{}", formatted_output);

        Ok(())
    }

    /// List all addresses and format them accordingly.
    pub fn list(&self, address_format: Option<AddressFormat>) -> AnyhowResult<()> {
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

        println!("{}", formatted_output);

        Ok(())
    }

    /// Add a new address
    pub fn add(&self, address_format: AddressFormat, data: String) -> AnyhowResult<()> {
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
    pub fn update(
        &self,
        address_format: AddressFormat,
        id: Uuid,
        data: String,
    ) -> AnyhowResult<()> {
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

    /// Convert an address between formats
    pub fn convert(
        &self,
        data: String,
        from: AddressFormat,
        to: AddressFormat,
    ) -> AnyhowResult<()> {
        if from == to {
            return Err(anyhow!("Source and target formats are the same"));
        }

        match (from, to) {
            (AddressFormat::French, AddressFormat::Iso20022) => {
                let address: FrenchAddress = serde_json::from_str(&data)?;
                let iso: ISO20022Address = AddressConverter::french_to_iso(address)?;

                println!("{}", iso);
            }
            (AddressFormat::Iso20022, AddressFormat::French) => {
                let address: ISO20022Address = serde_json::from_str(&data)?;
                let french: FrenchAddress = AddressConverter::iso_to_french(address)?;

                println!("{}", french);
            }
            _ => {
                return Err(anyhow!("Unsupported format conversion"));
            }
        }

        Ok(())
    }
}
