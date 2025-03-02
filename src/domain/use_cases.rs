use crate::{
    AnyhowResult,
    domain::{Address, FrenchAddress, ISO20022Address},
};

pub struct AddressConverter;

impl AddressConverter {
    pub fn french_to_iso(french: FrenchAddress) -> AnyhowResult<ISO20022Address> {
        let internal: Address = french.try_into()?;

        Ok(internal.into())
    }
}
