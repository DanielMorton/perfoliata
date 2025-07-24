
use crate::cli::stats::{handle_identifier_stats, handle_location_stats, handle_observer_stats, handle_process_locations, handle_species_stats};
use crate::client::INaturalistClient;
use crate::error::ClientError;
use super::args::Commands;

/// Main command execution dispatcher
pub async fn execute_command(client: &INaturalistClient, command: Commands) -> Result<(), ClientError> {
    match command {
        Commands::LocationStats { location, params } => {
            handle_location_stats(client, location, params).await
        }

        Commands::ObserverStats { location, params } => {
            handle_observer_stats(client, location, params).await
        }

        Commands::IdentifierStats { location, params } => {
            handle_identifier_stats(client, location, params).await
        }

        Commands::SpeciesStats { location, params } => {
            handle_species_stats(client, location, params).await
        }

        Commands::ProcessLocations { locations, stat_type, max_workers, params } => {
            handle_process_locations(client, locations, stat_type, max_workers, params).await
        }
    }
}