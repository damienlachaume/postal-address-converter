use crate::domain::{FrenchAddress, ISO20022Address};

/// Formats a [FrenchAddress] as plain text.
pub fn format_french_address(address: &FrenchAddress) -> String {
    format!(
        "Name: {}\nRecipient Info: {:?}\nGeographic Info: {:?}\nStreet: {}\nSpecial Mentions: {:?}\nPostal Info: {}\nCountry: {}",
        address.name,
        address.recipient_info,
        address.geographic_info,
        address.street,
        address.special_mentions,
        address.postal_info,
        address.country,
    )
}

/// Formats an [ISO20022Address] as XML.
pub fn format_iso20022_address(address: &ISO20022Address) -> String {
    format!(
        "<PstlAdr>\n  <StrtNm>{}</StrtNm>\n  <PstCd>{}</PstCd>\n  <TwnNm>{}</TwnNm>\n  <Ctry>{}</Ctry>\n</PstlAdr>",
        address.street_name, address.post_code, address.town_name, address.country,
    )
}
