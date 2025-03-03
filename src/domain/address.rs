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
