/// Represents a postal address in the internal model
#[derive(Debug, PartialEq)]
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
}
