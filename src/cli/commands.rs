use super::args::{Commands, get_locations};
use crate::client::INaturalistClient;
use crate::error::ClientError;
use crate::models::{
    execute_identifier_stats, execute_location_stats, execute_observer_stats, execute_species_stats,
};

/// Main command execution dispatcher
pub async fn execute_command(
    client: &INaturalistClient,
    command: Commands,
) -> Result<(), ClientError> {
    match command {
        Commands::LocationStats {
            locations,
            csv_file,
            csv_column,
            max_workers,
            params,
            output,
        } => {
            let resolved_locations = get_locations(&locations, &csv_file, &csv_column)?;
            execute_location_stats(client, resolved_locations, max_workers, params, output).await
        }
        Commands::ObserverStats {
            locations,
            csv_file,
            csv_column,
            max_workers,
            params,
            output,
        } => {
            let resolved_locations = get_locations(&locations, &csv_file, &csv_column)?;
            execute_observer_stats(client, resolved_locations, max_workers, params, output).await
        }
        Commands::IdentifierStats {
            locations,
            csv_file,
            csv_column,
            max_workers,
            params,
            output,
        } => {
            let resolved_locations = get_locations(&locations, &csv_file, &csv_column)?;
            execute_identifier_stats(client, resolved_locations, max_workers, params, output).await
        }
        Commands::SpeciesStats {
            locations,
            csv_file,
            csv_column,
            max_workers,
            params,
            output,
        } => {
            let resolved_locations = get_locations(&locations, &csv_file, &csv_column)?;
            execute_species_stats(client, resolved_locations, max_workers, params, output).await
        }
    }
}
