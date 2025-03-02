use postal_address_converter::Address;

fn main() {
    let address = Address::new(
        "25D RUE DES FLEURS".to_string(),
        "LIBOURNE".to_string(),
        "33500".to_string(),
        "FR".to_string(),
    );

    println!("Address: {:?}", address);
}
