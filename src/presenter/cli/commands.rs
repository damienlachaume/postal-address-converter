use std::{path::PathBuf, str::FromStr, sync::Arc};

use clap::{Parser, Subcommand, ValueEnum};
use uuid::Uuid;

use crate::{AddressService, AnyhowResult, infrastructure::FileAddressRepository};

use super::AddressHandler;

#[derive(Parser)]
#[clap(
    name = "postal-address-converter",
    version,
    about = "Convert and manage postal addresses"
)]
struct Cli {
    /// Path to the address database file
    #[clap(short, long, default_value = "addresses.json")]
    database: PathBuf,

    #[clap(subcommand)]
    command: Command,
}

/// Address format
#[derive(ValueEnum, Clone, Debug, PartialEq, Eq)]
pub enum AddressFormat {
    /// French address format
    French,

    /// ISO 20022 address format
    Iso20022,
}

#[derive(Subcommand)]
enum Command {
    /// Add a new address
    Add {
        /// Address format (french or iso20022)
        #[clap(short, long, value_enum)]
        format: AddressFormat,

        /// Address data as JSON string
        #[clap(short, long)]
        data: String,
    },

    /// Get an address by ID
    Get {
        /// Address identifier
        #[clap(short, long)]
        id: String,

        /// Output format (french or iso20022)
        #[clap(short, long, value_enum)]
        format: Option<AddressFormat>,
    },

    /// List all addresses
    List {
        /// Output format (french or iso20022)
        #[clap(short, long, value_enum)]
        format: Option<AddressFormat>,
    },

    /// Update an existing address
    Update {
        /// Address identifier
        #[clap(short, long)]
        id: String,

        /// Address format (french or iso20022)
        #[clap(short, long, value_enum)]
        format: AddressFormat,

        /// Address data as JSON string
        #[clap(short, long)]
        data: String,
    },

    /// Delete an address
    Delete {
        /// Address identifier
        #[clap(short, long)]
        id: String,
    },

    /// Convert an address between formats (not implemented yet)
    Convert {
        /// Source
        #[clap(short, long)]
        data: String,

        /// Source format
        #[clap(short, long, value_enum)]
        from: AddressFormat,

        /// Target format
        #[clap(short, long, value_enum)]
        to: AddressFormat,
    },
}

/// Run the CLI
pub fn run() -> AnyhowResult<()> {
    let cli = Cli::parse();
    let repository = Arc::new(FileAddressRepository::new(&cli.database)?);
    let service = AddressService::new(repository);
    let handler = AddressHandler::new(service);

    match cli.command {
        Command::Get { id, format } => handler.get(Uuid::from_str(&id)?, format),
        Command::List { format } => handler.list(format),
        Command::Add { format, data } => handler.add(format, data),
        Command::Update { id, format, data } => handler.update(format, Uuid::from_str(&id)?, data),
        Command::Delete { id } => handler.delete(Uuid::from_str(&id)?),
        Command::Convert { data, from, to } => handler.convert(data, from, to),
    }
}
