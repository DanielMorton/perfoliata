use crate::INaturalistClient;
use crate::error::{ClientError, Result};
use crate::models::model::process_stats_parallel;
use crate::models::save_stats_to_csv;
use crate::utils::json::{extract_f64_field, extract_u32_field};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Taxon {
    pub id: u32,
    pub rank: String,
    pub rank_level: f64,
    pub iconic_taxon_id: Option<u32>,
    //pub ancestor_ids: Vec<u32>,
    pub is_active: bool,
    pub name: String,
    pub parent_id: Option<u32>,
    pub ancestry: Option<String>,
    pub extinct: bool,
    //pub taxon_changes_count: u32,
    //pub taxon_schemes_count: u32,
    pub observations_count: u32,
    //pub current_synonymous_taxon_ids: Option<Value>, // Can be null or array
    pub atlas_id: Option<u32>,
    pub complete_species_count: Option<u32>,
    pub wikipedia_url: Option<String>,
    pub iconic_taxon_name: Option<String>,
}

impl Taxon {
    pub fn from_response(response: &Value) -> Result<Vec<Self>> {
        let results_array = response["results"].as_array().ok_or_else(|| {
            ClientError::InvalidResponse("Invalid taxa response format".to_string())
        })?;

        let mut taxa = Vec::new();
        for item in results_array {
            let taxon = Self::from_json_item(item)?;
            taxa.push(taxon);
        }

        Ok(taxa)
    }

    fn from_json_item(item: &Value) -> Result<Self> {
        let id = extract_u32_field(item, "id")?;
        let rank = item["rank"]
            .as_str()
            .ok_or_else(|| ClientError::MissingField("rank".to_string()))?
            .to_string();
        let rank_level = extract_f64_field(item, "rank_level")?;
        let iconic_taxon_id = item["iconic_taxon_id"].as_u64().map(|v| v as u32);

        /*let ancestor_ids = item["ancestor_ids"]
        .as_array()
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_u64().map(|n| n as u32))
                .collect()
        })
        .unwrap_or_default();*/

        let is_active = item["is_active"]
            .as_bool()
            .ok_or_else(|| ClientError::MissingField("is_active".to_string()))?;

        let name = item["name"]
            .as_str()
            .ok_or_else(|| ClientError::MissingField("name".to_string()))?
            .to_string();

        let parent_id = item["parent_id"].as_u64().map(|v| v as u32);
        let ancestry = item["ancestry"].as_str().map(|s| s.to_string());

        let extinct = item["extinct"].as_bool().unwrap_or(false);

        //let taxon_changes_count = extract_u32_field(item, "taxon_changes_count")?;
        //let taxon_schemes_count = extract_u32_field(item, "taxon_schemes_count")?;
        let observations_count = extract_u32_field(item, "observations_count")?;

        /*let current_synonymous_taxon_ids = if item["current_synonymous_taxon_ids"].is_null() {
            None
        } else {
            Some(item["current_synonymous_taxon_ids"].clone())
        };*/

        let atlas_id = item["atlas_id"].as_u64().map(|v| v as u32);
        let complete_species_count = item["complete_species_count"].as_u64().map(|v| v as u32);
        let wikipedia_url = item["wikipedia_url"].as_str().map(|s| s.to_string());
        let iconic_taxon_name = item["iconic_taxon_name"].as_str().map(|s| s.to_string());

        Ok(Taxon {
            id,
            rank,
            rank_level,
            iconic_taxon_id,
            // ancestor_ids,
            is_active,
            name,
            parent_id,
            ancestry,
            extinct,
            //taxon_changes_count,
            //taxon_schemes_count,
            observations_count,
            //current_synonymous_taxon_ids,
            atlas_id,
            complete_species_count,
            wikipedia_url,
            iconic_taxon_name,
        })
    }
}

/// Handle taxa parallel processing
pub async fn handle_taxon_processing(
    client: &INaturalistClient,
    id_starts: &[u32],
    extra_params: Vec<(String, String)>,
    max_workers: usize,
) -> Vec<Taxon> {
    process_stats_parallel(
        client,
        id_starts,
        extra_params,
        max_workers,
        |client, id_start, params| async move { client.get_taxa_stats(id_start, &params).await },
    )
    .await
    .into_iter()
    .flatten()
    .collect::<Vec<_>>()
}

/// Execute taxon statistics command
pub async fn execute_taxon_stats(
    client: &INaturalistClient,
    id_min: u32,
    id_max: u32,
    per_page: u32,
    max_workers: usize,
    params: Vec<(String, String)>,
    output: PathBuf,
) -> Result<()> {
    let id_starts = (id_min..=id_max).step_by(per_page as usize).collect::<Vec<_>>();
    let res = handle_taxon_processing(client, &id_starts, params, max_workers).await;
    save_stats_to_csv(&res, output)?;
    Ok(())
}
