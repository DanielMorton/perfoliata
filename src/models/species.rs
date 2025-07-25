use std::path::PathBuf;
use crate::INaturalistClient;
use crate::error::{ClientError, Result};
use crate::models::model::process_stats_parallel;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::models::save_stats_to_csv;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SpeciesStats {
    pub count: u64,
    pub id: u64,
    pub name: String,
    pub rank: String,
    pub ancestor_ids: Vec<u64>,
    pub location: Option<u32>,
}

impl SpeciesStats {
    pub fn from_response(response: &Value, location: Option<u32>) -> Result<Vec<Self>> {
        let results_array = response["results"].as_array().ok_or_else(|| {
            ClientError::InvalidResponse("Invalid species response format".to_string())
        })?;

        let mut stats = Vec::new();
        for item in results_array {
            let count = item["count"]
                .as_u64()
                .ok_or_else(|| ClientError::MissingField("count".to_string()))?;

            let taxon = &item["taxon"];
            let id = taxon["id"]
                .as_u64()
                .ok_or_else(|| ClientError::MissingField("taxon.id".to_string()))?;
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
                .map(|arr| arr.iter().filter_map(|v| v.as_u64()).collect())
                .unwrap_or_default();

            stats.push(SpeciesStats {
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
) -> Vec<SpeciesStats> {
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