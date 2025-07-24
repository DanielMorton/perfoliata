use log::info;
use perfoliata::{ClientError, INaturalistClient};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), ClientError> {
    env_logger::init();

    let client = INaturalistClient::new(1.0)?; // 1 request per second

    info!("Starting iNaturalist client example");

    // Example: Get location stats for a specific place
    let location_stats = client.get_location_stats("6803", HashMap::new()).await?;
    println!("Location stats: {:?}", location_stats);

    // Example: Get observer stats
    let observer_stats = client.get_observer_stats("6803", HashMap::new()).await?;
    println!(
        "Observer stats (first 3): {:?}",
        &observer_stats[..3.min(observer_stats.len())]
    );

    // Example: Process multiple locations in parallel
    let locations = vec!["6803".to_string(), "6804".to_string()];
    let client_clone = client.clone(); // Clone the client before moving into closure
    let results = client
        .process_locations_parallel(
            locations,
            move |location| {
                let client = client_clone.clone();
                async move { client.get_location_stats(&location, HashMap::new()).await }
            },
            Some(2), // 2 concurrent workers
        )
        .await;

    println!("Processed {} locations", results.len());

    Ok(())
}
