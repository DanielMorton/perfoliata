use crate::cli::stat::{
    handle_identifier_processing, handle_observer_processing, handle_species_processing,
    params_to_hashmap,
};
use crate::cli::{StatType, handle_location_processing};
use crate::{ClientError, INaturalistClient, LocationStats, ObserverStats, SpeciesStats};
use log::{error, info};
use std::collections::HashMap;

/// Handle parallel processing of multiple locations
pub async fn handle_process_locations(
    client: &INaturalistClient,
    locations: Vec<String>,
    stat_type: StatType,
    max_workers: usize,
    params: Vec<(String, String)>,
) -> Result<(), ClientError> {
    info!(
        "Processing {} locations in parallel with {} workers",
        locations.len(),
        max_workers
    );
    let extra_params = params_to_hashmap(params);

    match stat_type {
        StatType::Location => {
            let results =
                handle_location_processing(client, locations.clone(), extra_params, max_workers)
                    .await;
            println!("Processed {} locations successfully", results.len());
            for (i, location_stats) in results.iter().enumerate() {
                if i < locations.len() {
                    println!(
                        "Location {}: {} monthly records",
                        locations[i],
                        location_stats.len()
                    );
                }
            }
        }

        StatType::Observer => {
            let results =
                handle_observer_processing(client, locations.clone(), extra_params, max_workers)
                    .await;
            println!("Processed {} locations successfully", results.len());
            for (i, observer_stats) in results.iter().enumerate() {
                if i < locations.len() {
                    println!(
                        "Location {}: {} observers",
                        locations[i],
                        observer_stats.len()
                    );
                }
            }
        }

        StatType::Identifier => {
            let results =
                handle_identifier_processing(client, locations.clone(), extra_params, max_workers)
                    .await;
            println!("Processed {} locations successfully", results.len());
            for (i, identifier_stats) in results.iter().enumerate() {
                if i < locations.len() {
                    println!(
                        "Location {}: {} identifiers",
                        locations[i],
                        identifier_stats.len()
                    );
                }
            }
        }

        StatType::Species => {
            let results =
                handle_species_processing(client, locations.clone(), extra_params, max_workers)
                    .await;
            println!("Processed {} locations successfully", results.len());
            for (i, species_stats) in results.iter().enumerate() {
                if i < locations.len() {
                    println!("Location {}: {} species", locations[i], species_stats.len());
                }
            }
        }
    }

    Ok(())
}
