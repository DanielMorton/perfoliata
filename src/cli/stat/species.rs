use crate::cli::stat::stats::process_stats_parallel;
use crate::{INaturalistClient, SpeciesStats};
use std::collections::HashMap;

/// Handle species parallel processing
pub async fn handle_species_processing(
    client: &INaturalistClient,
    locations: Vec<String>,
    extra_params: HashMap<String, String>,
    max_workers: usize,
) -> Vec<Vec<SpeciesStats>> {
    process_stats_parallel(
        client,
        locations,
        extra_params,
        max_workers,
        |client, location, params| async move {
            client.get_species_stats(Some(&location), params).await
        },
    )
    .await
}
