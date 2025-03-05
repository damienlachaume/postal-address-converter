[![WIP](https://img.shields.io/badge/status-WIP-yellow.svg)](https://github.com/damienlachaume/postal-address-converter)

# Postal Address Converter

Postal Address Converter is a CLI application that converts postal addresses between the French format (NF Z10-011) and the ISO 20022 format while persisting them in a JSON file.

## Features

- **Address Conversion**: Convert addresses between the French format and ISO 20022 format.
- **Persistence**: Save, update, and delete addresses in a JSON file.
- **Command-Line Interface (CLI)**: Manage addresses through commands.
- **Extensibility**: Modular architecture that makes it easy to add new presenters (e.g., an API) or repository implementations (e.g., a database).

## Architecture

The project is organized into several modules:

- **Domain**: Contains the `Address` internal struct and conversion models (`FrenchAddress`, `ISO20022Address`), which centralize the core business logic.
- **Repositories**: Implements the `AddressRepository` trait with two concrete versions: an in-memory repository and a file-based repository.
- **Service**: Encapsulated in `AddressService`, which handles business logic and uses dependency injection to interact with repositories.
- **Presenter (CLI)**: Built with Clap, this module defines commands (add, get, list, update, delete, convert) and formats the output accordingly.

## Installation

Clone the repository:

```bash
git clone https://github.com/damienlachaume/postal-address-converter.git
cd postal-address-converter
```

Then build the project and add:

```bash
cargo build --release
export PATH="$PATH:$(pwd)/target/release"
```

or using the [Just](https://github.com/casey/just) command:

```bash
just install
```

## Usage

For simplicity, you can use an address JSON file located in the `/assets/examples` directory to insert addresses.
You can also run the command with the JSON inlined: `{"name": "Monsieur Jean DELHOURME", "floor": "Entrée A Bâtiment Jonquille", "post_box": null, "room": "Chez Mireille COPEAU Appartement 2", "street_name": "25 RUE DE L’EGLISE", "town_name": "MIOS", "town_location_name": "CAUDOS", "post_code": "33380", "country": "FR"}`

The database argument is optional and defaults to `addresses.json` in your current directory.

### Adding an address

```bash
postal-address-converter --database db.json add --data "$(cat ./assets/examples/internal_address_01.json)"
```

or

```bash
postal-address-converter --database db.json add --data '{"name": "Monsieur Jean DELHOURME", "floor": "Entrée A Bâtiment Jonquille", "post_box": null, "room": "Chez Mireille COPEAU Appartement 2", "street_name": "25 RUE DE L’EGLISE", "town_name": "MIOS", "town_location_name": "CAUDOS", "post_code": "33380", "country": "FR"}'
```

### Retrieving an addresses

```bash
postal-address-converter --database db.json get --id <ID> --format iso20022
```

```bash
postal-address-converter --database db.json get --id <ID> --format french
```

### Listing all addresses

```bash
postal-address-converter --database db.json list --format iso20022
```

```bash
postal-address-converter --database db.json list --format french
```

### Updating an address

```bash
postal-address-converter --database db.json update --id <ID> --data "$(cat ./assets/examples/updated_address.json)"
```

### Deleting an address

```bash
postal-address-converter --database db.json delete --id <ID>
```

### Converting an address

🚧 Note: The conversion command is not yet implemented.
For now, you can test the address conversion mechanism using the `--format` argument on the `get` and `list` commands.

## Test

To run both unit tests and integration tests, execute:

```bash
cargo test

```

For even faster testing using [`cargo-nextest`](https://crates.io/crates/cargo-nextest) with:

```bash
cargo nextest run

```

## Generate documentation

Generate HTML documentation with:

```bash
cargo doc --open
```
