use serde::{Deserialize, Serialize};

/// Represents a postal address in the internal model
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Address {
    /// Floor
    pub floor: Option<String>,

    /// Numbered box in a post office
    pub post_box: Option<String>,

    /// Room within a building
    pub room: Option<String>,

    /// Name of a street
    pub street_name: String,

    /// Name of the town
    pub town_name: String,

    /// Specific location name within the town
    pub town_location_name: Option<String>,

    /// Postal code
    pub post_code: String,

    /// Country (ISO country code)
    pub country: String,
}

impl Address {
    #[cfg(test)]
    /// Create a dummy [Address]
    pub fn dummy() -> Self {
        Self {
            floor: None,
            post_box: None,
            room: None,
            street_name: "123 Main St".to_string(),
            town_name: "Anytown".to_string(),
            town_location_name: None,
            post_code: "12345".to_string(),
            country: "FR".to_string(),
        }
    }
}
