#[derive(Debug)]
pub struct Address {
    street_name: String,
    town_name: String,
    post_code: String,
    country: String,
}

impl Address {
    pub fn new(street_name: String, town_name: String, post_code: String, country: String) -> Self {
        Self {
            street_name,
            town_name,
            post_code,
            country,
        }
    }
}
