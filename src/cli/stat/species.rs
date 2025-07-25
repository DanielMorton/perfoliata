use crate::cli::stat::params_to_hashmap;
use crate::cli::stat::stats::process_stats_parallel;
use crate::{ClientError, INaturalistClient, SpeciesStats};
use log::{error, info};
use std::collections::HashMap;

/// Handle species statistics command
pub async fn handle_species_stats(
    client: &INaturalistClient,
    location: Option<String>,
    params: Vec<(String, String)>,
) -> Result<(), ClientError> {
    let location_str = location.as_deref();
    info!("Fetching species stats for location: {:?}", location_str);
    let extra_params = params_to_hashmap(params);

    match client.get_species_stats(location_str, extra_params).await {
        Ok(stats) => {
            let loc_display = location_str.unwrap_or("Global");
            println!("Species Statistics for {}:", loc_display);
            println!("{:<8} {:<30} {:<15} {:<12}", "Count", "Name", "Rank", "ID");
            println!("{:-<65}", "");
            for stat in stats.iter().take(20) {
                // Show top 20
                println!(
                    "{:<8} {:<30} {:<15} {:<12}",
                    stat.count,
                    stat.name.chars().take(28).collect::<String>(),
                    stat.rank,
                    stat.id
                );
            }
            if stats.len() > 20 {
                println!("... and {} more species", stats.len() - 20);
            }
            Ok(())
        }
        Err(e) => {
            error!("Failed to fetch species stats: {}", e);
            Err(e)
        }
    }
}

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
