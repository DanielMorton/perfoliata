use crate::cli::Commands;
use crate::cli::args::get_locations;
use crate::client::INaturalistClient;
use crate::error::Result;
use crate::models::{
    execute_identifier_stats, execute_location_stats, execute_observer_stats,
    execute_species_stats, execute_taxon_stats,
};

pub async fn execute_command(client: &INaturalistClient, command: Commands) -> Result<()> {
    match command {
        Commands::LocationStats(cmd) => {
            let resolved_locations = get_locations(&cmd.locations, &cmd.csv_file, &cmd.csv_column)?;
            execute_location_stats(
                client,
                &resolved_locations,
                cmd.max_workers,
                cmd.params,
                cmd.output,
            )
            .await
        }
        Commands::ObserverStats(cmd) => {
            let resolved_locations = get_locations(&cmd.locations, &cmd.csv_file, &cmd.csv_column)?;
            execute_observer_stats(
                client,
                &resolved_locations,
                cmd.max_workers,
                cmd.params,
                cmd.output,
            )
            .await
        }
        Commands::IdentifierStats(cmd) => {
            let resolved_locations = get_locations(&cmd.locations, &cmd.csv_file, &cmd.csv_column)?;
            execute_identifier_stats(
                client,
                &resolved_locations,
                cmd.max_workers,
                cmd.params,
                cmd.output,
            )
            .await
        }
        Commands::SpeciesStats(cmd) => {
            let resolved_locations = get_locations(&cmd.locations, &cmd.csv_file, &cmd.csv_column)?;
            execute_species_stats(
                client,
                &resolved_locations,
                cmd.max_workers,
                cmd.params,
                cmd.output,
            )
            .await
        }
        Commands::Taxa(cmd) => {
            execute_taxon_stats(
                client,
                cmd.id_above,
                cmd.id_below,
                cmd.per_page,
                cmd.max_workers, // Pass the entire command struct
                cmd.params,
                cmd.output,
            )
            .await
        }
    }
}
