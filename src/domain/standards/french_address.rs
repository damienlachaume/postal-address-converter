use anyhow::{Context, anyhow};
use celes::Country;
use serde::{Deserialize, Serialize};

use crate::{AnyhowError, AnyhowResult, domain::Address};

type TownName = String;
type PostCode = String;

/// Represents a French postal address (NF Z10-011)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FrenchAddress {
    /// Line 1: Recipient identity
    name: String,

    /// Line 2: Additional identification of recipient or delivery point
    recipient_info: Option<String>,

    /// Line 3: Additional geographic point identification (entrance, building, residence)
    geographic_info: Option<String>,

    /// Line 4: Street number and name
    street: String,

    /// Line 5: Locality or special distribution service
    special_mentions: Option<String>,

    /// Line 6: Postal code and destination locality
    postal_info: String,

    /// Line 7: Country name
    country: String,
}

impl FrenchAddress {
    /// Constructor for a [FrenchAddress]
    pub fn new(
        name: String,
        recipient_info: Option<String>,
        geographic_info: Option<String>,
        street: String,
        special_mentions: Option<String>,
        postal_info: String,
        country: String,
    ) -> Self {
        Self {
            name,
            recipient_info,
            geographic_info,
            street,
            special_mentions,
            postal_info,
            country,
        }
    }
}

impl TryFrom<FrenchAddress> for Address {
    type Error = AnyhowError;

    fn try_from(french_address: FrenchAddress) -> AnyhowResult<Address> {
        let (post_code, town_name) = split_postal_info(french_address.postal_info)
            .with_context(|| "Failed to split postal info")?;

        let country_code = Country::from_name(french_address.country)
            .map_err(|e| anyhow!(e).context("Failed to convert country name to ISO code"))?
            .alpha2
            .to_string();

        let address = Address {
            floor: french_address.geographic_info,
            post_box: None,
            room: french_address.recipient_info,
            street_name: french_address.street,
            town_name,
            town_location_name: None,
            post_code,
            country: country_code,
        };

        Ok(address)
    }
}

impl TryFrom<Address> for FrenchAddress {
    type Error = AnyhowError;

    // TODO: test this conversion
    fn try_from(address: Address) -> AnyhowResult<Self> {
        let country = Country::from_alpha2(&address.country)
            .map_err(|e| anyhow!(e).context("Failed to convert country name to ISO code"))?;

        let postal_info = format!("{} {}", address.post_code, address.town_name);

        Ok(FrenchAddress::new(
            "".to_string(),
            address.room,
            address.floor,
            address.street_name,
            address.town_location_name,
            postal_info,
            country.long_name.to_string(),
        ))
    }
}

fn split_postal_info(postal_info: String) -> AnyhowResult<(PostCode, TownName)> {
    let parts: Vec<&str> = postal_info.split_whitespace().collect();
    let post_code = parts
        .first()
        .ok_or(anyhow!("Postal info is empty"))?
        .to_string();
    let town_name = parts[1..].join(" ");

    Ok((post_code, town_name))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_postal_info_into_town_name_and_post_code() {
        let postal_info = "33500 LIBOURNE".to_string();
        let (post_code, town_name) = split_postal_info(postal_info).unwrap();

        assert_eq!(post_code, "33500".to_string());
        assert_eq!(town_name, "LIBOURNE".to_string());
    }

    #[test]
    fn split_postal_info_returns_all_information_except_post_code_in_town_name() {
        let postal_info = "33500 LIBOURNE AND ANOTHER INFORMATION".to_string();
        let (post_code, town_name) = split_postal_info(postal_info).unwrap();

        assert_eq!(post_code, "33500".to_string());
        assert_eq!(town_name, "LIBOURNE AND ANOTHER INFORMATION".to_string());
    }

    #[test]
    fn split_postal_info_returns_error_when_postal_info_is_empty() {
        split_postal_info("".to_string())
            .expect_err("Should return error when postal info is empty");
    }

    #[test]
    fn convert_french_address_to_internal_with_empty_lines() {
        let french_address = FrenchAddress::new(
            "Monsieur Jean DURAND".to_string(),
            None,
            None,
            "25D RUE DES FLEURS".to_string(),
            None,
            "33500 LIBOURNE".to_string(),
            "France".to_string(),
        );

        let address: Address = french_address.try_into().unwrap();

        assert_eq!(
            address,
            Address {
                floor: None,
                post_box: None,
                room: None,
                street_name: "25D RUE DES FLEURS".to_string(),
                town_name: "LIBOURNE".to_string(),
                town_location_name: None,
                post_code: "33500".to_string(),
                country: "FR".to_string(),
            }
        );
    }

    #[test]
    fn convert_french_address_to_internal_without_empty_lines() {
        let french_address = FrenchAddress::new(
            "Monsieur Jean DELHOURME".to_string(),
            Some("Chez Mireille COPEAU Appartement 2".to_string()),
            Some("Entrée A Bâtiment Jonquille".to_string()),
            "25 RUE DE L’EGLISE".to_string(),
            Some("CAUDOS".to_string()),
            "33380 MIOS".to_string(),
            "France".to_string(),
        );

        let address: Address = french_address.try_into().unwrap();

        assert_eq!(
            address,
            Address {
                floor: Some("Entrée A Bâtiment Jonquille".to_string()),
                post_box: None,
                room: Some("Chez Mireille COPEAU Appartement 2".to_string()),
                street_name: "25 RUE DE L’EGLISE".to_string(),
                town_name: "MIOS".to_string(),
                town_location_name: None,
                post_code: "33380".to_string(),
                country: "FR".to_string(),
            },
        );
    }
}
