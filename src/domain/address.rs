use serde::{Deserialize, Serialize};

/// Represents a postal address in the internal model
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Address {
    pub floor: Option<String>,
    pub post_box: Option<String>,
    pub room: Option<String>,
    pub street_name: String,
    pub town_name: String,
    pub town_location_name: Option<String>,
    pub post_code: String,
    pub country: String,
}

impl Address {
    /// Constructor for an [Address]
    pub fn new(
        floor: Option<String>,
        post_box: Option<String>,
        room: Option<String>,
        street_name: String,
        town_name: String,
        town_location_name: Option<String>,
        post_code: String,
        country: String,
    ) -> Self {
        Self {
            floor,
            post_box,
            room,
            street_name,
            town_name,
            town_location_name,
            post_code,
            country,
        }
    }

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
