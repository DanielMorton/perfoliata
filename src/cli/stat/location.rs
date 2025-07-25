use crate::cli::stat::stats::process_stats_parallel;
use crate::{INaturalistClient, LocationStats};
use std::collections::HashMap;


/// Handle location parallel processing
pub async fn handle_location_processing(
    client: &INaturalistClient,
    locations: Vec<String>,
    extra_params: HashMap<String, String>,
    max_workers: usize,
) -> Vec<Vec<LocationStats>> {
    process_stats_parallel(
        client,
        locations,
        extra_params,
        max_workers,
        |client, location, params| async move {
            client.get_location_stats(&location, params).await
        },
    ).await
}
