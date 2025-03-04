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
}
