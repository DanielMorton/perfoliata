use super::args::Commands;
use crate::client::INaturalistClient;
use crate::error::ClientError;
use crate::models::identifier::handle_identifier_processing;
use crate::models::location::handle_location_processing;
use crate::models::observer::handle_observer_processing;
use crate::models::save_stats_to_csv;
use crate::models::species::handle_species_processing;

/// Main command execution dispatcher
pub async fn execute_command(
    client: &INaturalistClient,
    command: Commands,
) -> Result<(), ClientError> {
    match command {
        Commands::LocationStats {
            locations,
            max_workers,
            params,
            output,
        } => {
            let res = handle_location_processing(client, locations, params, max_workers).await;
            save_stats_to_csv(&res, output);
        }

        Commands::ObserverStats {
            locations,
            max_workers,
            params,
            output,
        } => {
            let res = handle_observer_processing(client, locations, params, max_workers).await;
            save_stats_to_csv(&res, output);
        }

        Commands::IdentifierStats {
            locations,
            max_workers,
            params,
            output,
        } => {
            let res = handle_identifier_processing(client, locations, params, max_workers).await;
            save_stats_to_csv(&res, output);
        }

        Commands::SpeciesStats {
            locations,
            max_workers,
            params,
            output,
        } => {
            let res = handle_species_processing(client, locations, params, max_workers).await;
            save_stats_to_csv(&res, output);
        }
    }
    Ok(())
}
