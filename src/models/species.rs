use crate::INaturalistClient;
use crate::error::{ClientError, Result};
use crate::models::model::process_stats_parallel;
use crate::models::save_stats_to_csv;
use crate::utils::json::{extract_u32_field, json_value_to_u32};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ObservationSpeciesStats {
    pub count: u32,
    pub id: u32,
    pub name: String,
    pub rank: String,
    pub ancestor_ids: Vec<u32>,
    pub location: Option<u32>,
}

impl ObservationSpeciesStats {
    pub fn from_response(response: &Value, location: Option<u32>) -> Result<Vec<Self>> {
        let results_array = response["results"].as_array().ok_or_else(|| {
            ClientError::InvalidResponse("Invalid species response format".to_string())
        })?;

        let mut stats = Vec::new();
        for item in results_array {
            let count = extract_u32_field(item, "count")?;

            let taxon = &item["taxon"];
            let id = extract_u32_field(taxon, "id")?;
            let name = taxon["name"]
                .as_str()
                .ok_or_else(|| ClientError::MissingField("taxon.name".to_string()))?
                .to_string();
            let rank = taxon["rank"]
                .as_str()
                .ok_or_else(|| ClientError::MissingField("taxon.rank".to_string()))?
                .to_string();

            let ancestor_ids = taxon["ancestor_ids"]
                .as_array()
                .map(|arr| {
                    arr.iter()
                        .map(|v| json_value_to_u32(v))
                        .collect::<Result<Vec<u32>>>()
                })
                .unwrap_or_else(|| Ok(Vec::new()))?;

            stats.push(ObservationSpeciesStats {
                count,
                id,
                name,
                rank,
                ancestor_ids,
                location,
            });
        }

        Ok(stats)
    }
}

/// Handle species parallel processing
pub async fn handle_species_processing(
    client: &INaturalistClient,
    locations: Vec<u32>,
    extra_params: Vec<(String, String)>,
    max_workers: usize,
) -> Vec<ObservationSpeciesStats> {
    process_stats_parallel(
        client,
        locations,
        extra_params,
        max_workers,
        |client, location, params| async move {
            client.get_species_stats(Some(location), params).await
        },
    )
    .await
    .into_iter()
    .flatten()
    .collect::<Vec<_>>()
}

/// Execute species statistics command
pub async fn execute_species_stats(
    client: &INaturalistClient,
    locations: Vec<u32>,
    max_workers: usize,
    params: Vec<(String, String)>,
    output: PathBuf,
) -> Result<()> {
    let res = handle_species_processing(client, locations, params, max_workers).await;
    save_stats_to_csv(&res, output)?;
    Ok(())
}
