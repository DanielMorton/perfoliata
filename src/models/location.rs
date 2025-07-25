use std::path::PathBuf;
use crate::INaturalistClient;
use crate::error::{ClientError, Result};
use crate::models::model::process_stats_parallel;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::models::save_stats_to_csv;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LocationStats {
    pub month: i32,
    pub observation_count: i64,
    pub location: u32,
}

impl LocationStats {
    pub fn from_histogram_response(response: &Value, location: u32) -> Result<Vec<Self>> {
        let results_obj = response["results"].as_object().ok_or_else(|| {
            ClientError::InvalidResponse("Invalid histogram response format".to_string())
        })?;

        let mut stats = Vec::new();

        // Handle month_of_year structure
        if let Some(month_data) = results_obj.get("month_of_year").and_then(|v| v.as_object()) {
            for (month_str, count) in month_data {
                let month = month_str.parse::<i32>().map_err(|e| {
                    ClientError::Parse(format!("Failed to parse month from '{month_str}': {e}"))
                })?;

                let observation_count = count.as_i64().ok_or_else(|| {
                    ClientError::InvalidResponse(format!(
                        "Invalid count format for month '{month_str}': expected number, got {count:?}"
                    ))
                })?;

                stats.push(LocationStats {
                    month,
                    observation_count,
                    location,
                });
            }
        }

        // Sort by month for consistent ordering
        stats.sort_by_key(|s| s.month);

        Ok(stats)
    }
}

/// Handle location parallel processing
pub async fn handle_location_processing(
    client: &INaturalistClient,
    locations: Vec<u32>,
    extra_params: Vec<(String, String)>,
    max_workers: usize,
) -> Vec<LocationStats> {
    process_stats_parallel(
        client,
        locations,
        extra_params,
        max_workers,
        |client, location, params| async move {
            client.get_location_stats(location, params).await
        },
    ).await.into_iter().flatten().collect::<Vec<_>>()
}

/// Execute location statistics command
pub async fn execute_location_stats(
    client: &INaturalistClient,
    locations: Vec<u32>,
    max_workers: usize,
    params: Vec<(String, String)>,
    output: PathBuf,
) -> Result<()> {
    let res = handle_location_processing(client, locations, params, max_workers).await;
    save_stats_to_csv(&res, output)?;
    Ok(())
}
