use std::collections::HashMap;
use log::{error, info};
use crate::cli::StatType;
use crate::{ClientError, INaturalistClient, LocationStats, ObserverStats, SpeciesStats};

/// Convert Vec<(String, String)> to HashMap<String, String>
fn params_to_hashmap(params: Vec<(String, String)>) -> HashMap<String, String> {
    params.into_iter().collect()
}

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
            println!("{:<20} {:<15} {:<12} {:<15}", "Name", "Login", "Observations", "Species");
            println!("{:-<62}", "");
            for stat in stats.iter().take(20) { // Show top 20
                println!("{:<20} {:<15} {:<12} {:<15}",
                         stat.name, stat.login, stat.observation_count, stat.species_count);
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

/// Handle identifier statistics command
pub async fn handle_identifier_stats(
    client: &INaturalistClient,
    location: String,
    params: Vec<(String, String)>,
) -> Result<(), ClientError> {
    info!("Fetching identifier stats for location: {}", location);
    let extra_params = params_to_hashmap(params);

    match client.get_identifier_stats(&location, extra_params).await {
        Ok(stats) => {
            println!("Identifier Statistics for {}:", location);
            println!("{:<20} {:<15} {:<12} {:<15}", "Name", "Login", "Identifications", "Species");
            println!("{:-<62}", "");
            for stat in stats.iter().take(20) { // Show top 20
                println!("{:<20} {:<15} {:<12} {:<15}",
                         stat.name, stat.login, stat.observation_count, stat.species_count);
            }
            if stats.len() > 20 {
                println!("... and {} more identifiers", stats.len() - 20);
            }
            Ok(())
        }
        Err(e) => {
            error!("Failed to fetch identifier stats: {}", e);
            Err(e)
        }
    }
}

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
            for stat in stats.iter().take(20) { // Show top 20
                println!("{:<8} {:<30} {:<15} {:<12}",
                         stat.count,
                         stat.name.chars().take(28).collect::<String>(),
                         stat.rank,
                         stat.id);
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

/// Handle location parallel processing
pub async fn handle_location_processing(
    client: &INaturalistClient,
    locations: Vec<String>,
    extra_params: HashMap<String, String>,
    max_workers: usize,
) -> Vec<Vec<LocationStats>> {
    client.process_locations_parallel(
        locations.clone(),
        {
            let client = client.clone();
            let params = extra_params.clone();
            move |location| {
                let client = client.clone();
                let params = params.clone();
                async move {
                    client.get_location_stats(&location, params).await
                }
            }
        },
        Some(max_workers),
    ).await
}

/// Handle observer parallel processing
pub async fn handle_observer_processing(
    client: &INaturalistClient,
    locations: Vec<String>,
    extra_params: HashMap<String, String>,
    max_workers: usize,
) -> Vec<Vec<ObserverStats>> {
    client.process_locations_parallel(
        locations.clone(),
        {
            let client = client.clone();
            let params = extra_params.clone();
            move |location| {
                let client = client.clone();
                let params = params.clone();
                async move {
                    client.get_observer_stats(&location, params).await
                }
            }
        },
        Some(max_workers),
    ).await
}

/// Handle identifier parallel processing
pub async fn handle_identifier_processing(
    client: &INaturalistClient,
    locations: Vec<String>,
    extra_params: HashMap<String, String>,
    max_workers: usize,
) -> Vec<Vec<ObserverStats>> {
    client.process_locations_parallel(
        locations.clone(),
        {
            let client = client.clone();
            let params = extra_params.clone();
            move |location| {
                let client = client.clone();
                let params = params.clone();
                async move {
                    client.get_identifier_stats(&location, params).await
                }
            }
        },
        Some(max_workers),
    ).await
}

/// Handle species parallel processing
pub async fn handle_species_processing(
    client: &INaturalistClient,
    locations: Vec<String>,
    extra_params: HashMap<String, String>,
    max_workers: usize,
) -> Vec<Vec<SpeciesStats>> {
    client.process_locations_parallel(
        locations.clone(),
        {
            let client = client.clone();
            let params = extra_params.clone();
            move |location| {
                let client = client.clone();
                let params = params.clone();
                async move {
                    client.get_species_stats(Some(&location), params).await
                }
            }
        },
        Some(max_workers),
    ).await
}

/// Handle parallel processing of multiple locations
pub async fn handle_process_locations(
    client: &INaturalistClient,
    locations: Vec<String>,
    stat_type: StatType,
    max_workers: usize,
    params: Vec<(String, String)>,
) -> Result<(), ClientError> {
    info!("Processing {} locations in parallel with {} workers", locations.len(), max_workers);
    let extra_params = params_to_hashmap(params);

    match stat_type {
        StatType::Location => {
            let results = handle_location_processing(client, locations.clone(), extra_params, max_workers).await;
            println!("Processed {} locations successfully", results.len());
            for (i, location_stats) in results.iter().enumerate() {
                if i < locations.len() {
                    println!("Location {}: {} monthly records", locations[i], location_stats.len());
                }
            }
        }

        StatType::Observer => {
            let results = handle_observer_processing(client, locations.clone(), extra_params, max_workers).await;
            println!("Processed {} locations successfully", results.len());
            for (i, observer_stats) in results.iter().enumerate() {
                if i < locations.len() {
                    println!("Location {}: {} observers", locations[i], observer_stats.len());
                }
            }
        }

        StatType::Identifier => {
            let results = handle_identifier_processing(client, locations.clone(), extra_params, max_workers).await;
            println!("Processed {} locations successfully", results.len());
            for (i, identifier_stats) in results.iter().enumerate() {
                if i < locations.len() {
                    println!("Location {}: {} identifiers", locations[i], identifier_stats.len());
                }
            }
        }

        StatType::Species => {
            let results = handle_species_processing(client, locations.clone(), extra_params, max_workers).await;
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