use crate::cli::handle_location_processing;
use super::args::Commands;
use crate::cli::stat::{handle_identifier_processing, handle_observer_processing, handle_species_processing};
use crate::client::INaturalistClient;
use crate::error::ClientError;

/// Main command execution dispatcher
pub async fn execute_command(
    client: &INaturalistClient,
    command: Commands,
) -> Result<(), ClientError> {
    match command {
        Commands::LocationStats { locations, max_workers, params } => {
            let _ = handle_location_processing(client, locations, params, max_workers).await;
        }

        Commands::ObserverStats { locations, max_workers, params } => {
            let _ = handle_observer_processing(client, locations, params, max_workers).await;
        }

        Commands::IdentifierStats { locations, max_workers, params } => {
            let _= handle_identifier_processing(client, locations, params, max_workers).await;
        }

        Commands::SpeciesStats { locations, max_workers, params } => {
            let _= handle_species_processing(client, locations, params, max_workers).await;
        }
    }
    Ok(())
}