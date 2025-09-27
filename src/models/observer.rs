use crate::INaturalistClient;
use crate::error::{ClientError, Result};
use crate::models::model::process_stats_parallel;
use crate::models::save_stats_to_csv;
use crate::utils::json::extract_u32_field;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ObservationObserverStats {
    pub observation_count: u32,
    pub species_count: u32,
    pub name: String,
    pub login: String,
    pub location: Option<u32>,
}

impl ObservationObserverStats {
    pub fn from_response(response: &Value, location: Option<u32>) -> Result<Vec<Self>> {
        let results_array = response["results"].as_array().ok_or_else(|| {
            ClientError::InvalidResponse("Invalid observers response format".to_string())
        })?;

        let mut stats = Vec::new();
        for item in results_array {
            let observation_count: u32 = extract_u32_field(item, "observation_count")?;
            let species_count = extract_u32_field(item, "species_count")?;

            let user = &item["user"];
            let name = match user["name"].as_str() {
                Some(n) if !n.is_empty() => n.to_string(),
                _ => {
                    // Fall back to login if name is null, empty, or missing
                    user["login"].as_str().unwrap_or("").to_string()
                }
            };
            let login = user["login"]
                .as_str()
                .ok_or_else(|| ClientError::MissingField("user.login".to_string()))?
                .to_string();

            stats.push(ObservationObserverStats {
                observation_count,
                species_count,
                name,
                login,
                location,
            });
        }

        Ok(stats)
    }
}

/// Handle observer parallel processing
pub async fn handle_observer_processing(
    client: &INaturalistClient,
    locations: &[u32],
    extra_params: Vec<(String, String)>,
    max_workers: usize,
) -> Vec<ObservationObserverStats> {
    process_stats_parallel(
        client,
        locations,
        extra_params,
        max_workers,
        |client, location, params| async move { client.get_observer_stats(location, &params).await },
    )
    .await
    .into_iter()
    .flatten()
    .collect::<Vec<_>>()
}

/// Execute observer statistics command
pub async fn execute_observer_stats(
    client: &INaturalistClient,
    locations: &[u32],
    max_workers: usize,
    params: Vec<(String, String)>,
    output: PathBuf,
) -> Result<()> {
    let res = handle_observer_processing(client, locations, params, max_workers).await;
    save_stats_to_csv(&res, output)?;
    Ok(())
}
