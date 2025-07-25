use crate::cli::stat::stats::process_stats_parallel;
use crate::{INaturalistClient, ObserverStats};
use std::collections::HashMap;

/// Handle identifier parallel processing
pub async fn handle_identifier_processing(
    client: &INaturalistClient,
    locations: Vec<String>,
    extra_params: HashMap<String, String>,
    max_workers: usize,
) -> Vec<Vec<ObserverStats>> {
    process_stats_parallel(
        client,
        locations,
        extra_params,
        max_workers,
        |client, location, params| async move {
            client.get_identifier_stats(&location, params).await
        },
    ).await
}
