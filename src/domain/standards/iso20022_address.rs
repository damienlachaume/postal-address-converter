use std::fmt::{self, Display, Formatter};

use anyhow::anyhow;
use celes::Country;
use serde::{Deserialize, Serialize};

use crate::{AnyhowError, AnyhowResult, domain::Address};

/// Represents an ISO 20022 postal address
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ISO20022Address {
    /// Identification of a division of a large organization or building
    #[serde(rename = "Dept")]
    pub department: Option<String>,

    /// Identification of a sub-division of a large organization or building
    #[serde(rename = "SubDept")]
    pub sub_department: Option<String>,

    /// Name of a street or thoroughfare
    #[serde(rename = "StrtNm")]
    pub street_name: String,

    /// Number that identifies the position of a building on a street
    #[serde(rename = "BldgNb")]
    pub building_number: Option<String>,

    /// Name of the building or house.
    #[serde(rename = "BldgNm")]
    pub building_name: Option<String>,

    /// Floor or storey within a building
    #[serde(rename = "Flr")]
    pub floor: Option<String>,

    /// Numbered box in a post office, assigned to a person or organization
    #[serde(rename = "PstBx")]
    pub post_box: Option<String>,

    /// Room within a building
    #[serde(rename = "Rm")]
    pub room: Option<String>,

    /// Identifier consisting of a group of letters and/or numbers that is added to a postal address
    #[serde(rename = "PstCd")]
    pub post_code: String,

    /// Name of a built-up area, with defined boundaries, and a local government
    #[serde(rename = "TwnNm")]
    pub town_name: String,

    /// Specific location name within the town
    #[serde(rename = "TwnLctnNm")]
    pub town_location_name: Option<String>,

    /// Identifies a subdivision within a country sub-division
    #[serde(rename = "DstrctNm")]
    pub district_name: Option<String>,

    /// Identifies a subdivision of a country such as state, region, county
    #[serde(rename = "CtrySubDvsn")]
    pub country_sub_division: Option<String>,

    /// Nation with its own government (ISO country code)
    #[serde(rename = "Ctry")]
    pub country: String,
}

impl Display for ISO20022Address {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "<PstlAdr>\n  <StrtNm>{}</StrtNm>\n  <PstCd>{}</PstCd>\n  <TwnNm>{}</TwnNm>\n  <Ctry>{}</Ctry>\n</PstlAdr>",
            self.street_name, self.post_code, self.town_name, self.country
        )
    }
}

impl From<Address> for ISO20022Address {
    fn from(address: Address) -> ISO20022Address {
        ISO20022Address {
            department: None,
            sub_department: None,
            street_name: address.street_name,
            building_number: None,
            building_name: None,
            floor: address.floor,
            post_box: address.post_box,
            room: address.room,
            post_code: address.post_code,
            town_name: address.town_name,
            town_location_name: address.town_location_name,
            district_name: None,
            country_sub_division: None,
            country: address.country,
        }
    }
}

impl TryFrom<ISO20022Address> for Address {
    type Error = AnyhowError;

    fn try_from(iso_address: ISO20022Address) -> AnyhowResult<Address> {
        verify_country_code(&iso_address.country)?;

        Ok(Address {
            name: None,
            floor: iso_address.floor,
            post_box: iso_address.post_box,
            room: iso_address.room,
            street_name: iso_address.street_name,
            town_name: iso_address.town_name,
            town_location_name: iso_address.town_location_name,
            post_code: iso_address.post_code,
            country: iso_address.country,
        })
    }
}

fn verify_country_code(country: &str) -> AnyhowResult<()> {
    Country::from_alpha2(country).map_err(|e| {
        anyhow!(e).context(format!(
            "Failed to retrieve country from ISO code: '{}'",
            country
        ))
    })?;

    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_from_internal_to_iso20022() {
        let address = Address {
            name: None,
            floor: Some("Zone industrielle de la Ballastrierre Ouest".to_string()),
            post_box: Some("BP 40122".to_string()),
            room: None,
            street_name: "22BIS RUE DES FLEURS".to_string(),
            town_name: "LIBOURNE CEDEX".to_string(),
            town_location_name: None,
            post_code: "33506".to_string(),
            country: "FR".to_string(),
        };

        let iso_address: ISO20022Address = address.into();

        assert_eq!(
            iso_address,
            ISO20022Address {
                department: None,
                sub_department: None,
                street_name: "22BIS RUE DES FLEURS".to_string(),
                building_number: None,
                building_name: None,
                floor: Some("Zone industrielle de la Ballastrierre Ouest".to_string()),
                post_box: Some("BP 40122".to_string()),
                room: None,
                post_code: "33506".to_string(),
                town_name: "LIBOURNE CEDEX".to_string(),
                town_location_name: None,
                district_name: None,
                country_sub_division: None,
                country: "FR".to_string(),
            }
        );
    }

    #[test]
    fn convert_iso20022_internal_simple() {
        let initial_iso_address = ISO20022Address {
            department: None,
            sub_department: None,
            street_name: "25D RUE DES FLEURS".to_string(),
            building_number: None,
            building_name: None,
            floor: None,
            post_box: None,
            room: None,
            post_code: "33500".to_string(),
            town_name: "LIBOURNE".to_string(),
            town_location_name: None,
            district_name: None,
            country_sub_division: None,
            country: "FR".to_string(),
        };

        let internal: Address = initial_iso_address.clone().try_into().unwrap();

        assert_eq!(
            internal,
            Address {
                name: None,
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

        let iso_address_from_internal: ISO20022Address = internal.into();

        assert_eq!(iso_address_from_internal, initial_iso_address);
    }

    #[test]
    fn convert_iso20022_internal_extended() {
        let initial_iso_address = ISO20022Address {
            department: None,
            sub_department: None,
            street_name: "25 RUE DES FLEURS".to_string(),
            building_number: None,
            building_name: None,
            floor: Some("Entrée A Bâtiment Jonquille".to_string()),
            post_box: None,
            room: Some("Chez Mireille COPEAU Appartement 2".to_string()),
            post_code: "33380".to_string(),
            town_name: "MIOS".to_string(),
            town_location_name: Some("CAUDOS".to_string()),
            district_name: None,
            country_sub_division: None,
            country: "FR".to_string(),
        };

        let internal: Address = initial_iso_address.clone().try_into().unwrap();

        assert_eq!(
            internal,
            Address {
                name: None,
                floor: Some("Entrée A Bâtiment Jonquille".to_string()),
                post_box: None,
                room: Some("Chez Mireille COPEAU Appartement 2".to_string()),
                street_name: "25 RUE DES FLEURS".to_string(),
                town_name: "MIOS".to_string(),
                town_location_name: Some("CAUDOS".to_string()),
                post_code: "33380".to_string(),
                country: "FR".to_string(),
            }
        );

        let iso_address_from_internal: ISO20022Address = internal.into();

        assert_eq!(iso_address_from_internal, initial_iso_address);
    }

    #[test]
    fn verify_country_code_returns_ok_when_country_code_is_valid() {
        verify_country_code("FR").expect("Country code should not fail with a valid ISO code");
    }

    #[test]
    fn verify_country_code_returns_error_when_country_code_is_invalid() {
        verify_country_code("France")
            .expect_err("Country code should fail with an invalid ISO code");
    }
}
