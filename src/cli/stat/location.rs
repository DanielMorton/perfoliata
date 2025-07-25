use crate::cli::stat::params_to_hashmap;
use crate::cli::stat::stats::process_stats_parallel;
use crate::{ClientError, INaturalistClient, LocationStats};
use log::{error, info};
use std::collections::HashMap;

/// Handle location statistics command
pub async fn handle_location_stats(
    client: &INaturalistClient,
    location: String,
    params: Vec<(String, String)>,
) -> Result<(), ClientError> {
    info!("Fetching location stats for location: {}", location);
    let extra_params = params_to_hashmap(params);

    match client.get_location_stats(&location, extra_params).await {
        Ok(stats) => {
            println!("Location Statistics for {}:", location);
            println!("{:<6} {:<15}", "Month", "Observations");
            println!("{:-<21}", "");
            for stat in stats {
                println!("{:<6} {:<15}", stat.month, stat.observation_count);
            }
            Ok(())
        }
        Err(e) => {
            error!("Failed to fetch location stats: {}", e);
            Err(e)
        }
    }
}

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
