use super::args::Commands;
use crate::client::INaturalistClient;
use crate::error::ClientError;
use crate::models::{execute_identifier_stats, execute_location_stats, execute_observer_stats, execute_species_stats};

/// Main command execution dispatcher
pub async fn execute_command(
    client: &INaturalistClient,
    command: Commands,
) -> Result<(), ClientError> {
    match command {
        Commands::LocationStats { locations, max_workers, params, output } => {
            execute_location_stats(client, locations, max_workers, params, output).await
        }
        Commands::ObserverStats { locations, max_workers, params, output } => {
            execute_observer_stats(client, locations, max_workers, params, output).await
        }
        Commands::IdentifierStats { locations, max_workers, params, output } => {
            execute_identifier_stats(client, locations, max_workers, params, output).await
        }
        Commands::SpeciesStats { locations, max_workers, params, output } => {
            execute_species_stats(client, locations, max_workers, params, output).await
        }
    }
}

