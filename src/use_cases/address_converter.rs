use crate::{
    AnyhowResult,
    domain::{
        Address, {FrenchAddress, ISO20022Address},
    },
};

/// Convert addresses between different formats
pub struct AddressConverter;

impl AddressConverter {
    /// Convert a [FrenchAddress] to an [ISO20022Address]
    pub fn french_to_iso(address: FrenchAddress) -> AnyhowResult<ISO20022Address> {
        let internal_address: Address = address.try_into()?;

        Ok(internal_address.into())
    }

    /// Convert a [ISO20022Address] to a [FrenchAddress]
    pub fn iso_to_french(address: ISO20022Address) -> AnyhowResult<FrenchAddress> {
        let internal_address: Address = address.try_into()?;

        internal_address.try_into()
    }

    /// Convert a [ISO20022Address] to an [Address]
    pub fn iso_to_address(address: ISO20022Address) -> AnyhowResult<Address> {
        let internal_address: Address = address.try_into()?;

        Ok(internal_address)
    }

    /// Convert an [Address] to an [ISO20022Address]
    pub fn internal_address_to_iso(address: Address) -> ISO20022Address {
        let internal_address: ISO20022Address = address.into();

        internal_address
    }

    /// Convert an [Address] to a [FrenchAddress]
    pub fn internal_address_to_french(address: Address) -> AnyhowResult<FrenchAddress> {
        let internal_address: FrenchAddress = address.try_into()?;

        Ok(internal_address)
    }
}
