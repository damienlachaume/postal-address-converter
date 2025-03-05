use std::fs;

use postal_address_converter::{Address, AddressService, FileAddressRepository};

#[test]
fn service_add_update_delete() {
    let dir = std::env::temp_dir().join("service_integration_test");
    if dir.exists() {
        fs::remove_dir_all(&dir).unwrap();
    }
    fs::create_dir_all(&dir).unwrap();
    let db_path = dir.join("addresses-database.json");

    let repository = FileAddressRepository::new(&db_path).unwrap();
    let service = AddressService::new(std::sync::Arc::new(repository));

    let address = Address {
        name: Some("Alice".to_string()),
        street_name: "123 Integration St".to_string(),
        town_name: "Paris".to_string(),
        post_code: "75000".to_string(),
        country: "FR".to_string(),
        floor: None,
        post_box: None,
        room: None,
        town_location_name: None,
    };

    let id = service.add(&address).unwrap();
    let retrieved = service.get(id).unwrap().expect("Address must be found");
    assert_eq!(retrieved, address);

    let updated_address = Address {
        town_name: "Bordeaux".to_string(),
        ..address.clone()
    };
    service.update(id, &updated_address).unwrap();
    let updated = service.get(id).unwrap().expect("Address must be found");
    assert_eq!(updated, updated_address);

    service.delete(id).unwrap();
    assert!(service.get(id).unwrap().is_none());

    let file_content = fs::read_to_string(&db_path).unwrap();
    let data: serde_json::Value = serde_json::from_str(&file_content).unwrap();
    assert!(
        !data["addresses"]
            .as_object()
            .unwrap()
            .contains_key(&id.to_string())
    );
}
