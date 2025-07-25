use crate::cli::stat::params_to_hashmap;
use crate::cli::stat::stats::process_stats_parallel;
use crate::{ClientError, INaturalistClient, ObserverStats};
use log::{error, info};
use std::collections::HashMap;

/// Handle observer statistics command
pub async fn handle_observer_stats(
    client: &INaturalistClient,
    location: String,
    params: Vec<(String, String)>,
) -> Result<(), ClientError> {
    info!("Fetching observer stats for location: {}", location);
    let extra_params = params_to_hashmap(params);

    match client.get_observer_stats(&location, extra_params).await {
        Ok(stats) => {
            println!("Observer Statistics for {}:", location);
            println!(
                "{:<20} {:<15} {:<12} {:<15}",
                "Name", "Login", "Observations", "Species"
            );
            println!("{:-<62}", "");
            for stat in stats.iter().take(20) {
                // Show top 20
                println!(
                    "{:<20} {:<15} {:<12} {:<15}",
                    stat.name, stat.login, stat.observation_count, stat.species_count
                );
            }
            if stats.len() > 20 {
                println!("... and {} more observers", stats.len() - 20);
            }
            Ok(())
        }
        Err(e) => {
            error!("Failed to fetch observer stats: {}", e);
            Err(e)
        }
    }
}

/// Handle observer parallel processing
pub async fn handle_observer_processing(
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
            client.get_observer_stats(&location, params).await
        },
    ).await
}
