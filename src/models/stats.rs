use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::error::{ClientError, Result};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LocationStats {
    pub month: i32,
    pub observation_count: i64,
    pub location: String,
}

impl LocationStats {
    pub fn from_histogram_response(response: &Value, location: &str) -> Result<Vec<Self>> {
        let results_obj = response["results"].as_object()
            .ok_or_else(|| ClientError::InvalidResponse("Invalid histogram response format".to_string()))?;

        let mut stats = Vec::new();

        // Handle month_of_year structure
        if let Some(month_data) = results_obj.get("month_of_year").and_then(|v| v.as_object()) {
            for (month_str, count) in month_data {
                let month = month_str.parse::<i32>()
                    .map_err(|e| ClientError::Parse(format!("Failed to parse month from '{}': {}", month_str, e)))?;

                let observation_count = count.as_i64()
                    .ok_or_else(|| ClientError::InvalidResponse(format!("Invalid count format for month '{}': expected number, got {:?}", month_str, count)))?;

                stats.push(LocationStats {
                    month,
                    observation_count,
                    location: location.to_string(),
                });
            }
        }

        // Sort by month for consistent ordering
        stats.sort_by_key(|s| s.month);

        Ok(stats)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ObserverStats {
    pub observation_count: i64,
    pub species_count: i64,
    pub name: String,
    pub login: String,
    pub location: String,
}

impl ObserverStats {
    pub fn from_response(response: &Value, location: &str) -> Result<Vec<Self>> {
        let results_array = response["results"].as_array()
            .ok_or_else(|| ClientError::InvalidResponse("Invalid observers response format".to_string()))?;

        let mut stats = Vec::new();
        for item in results_array {
            let observation_count = item["observation_count"].as_i64()
                .ok_or_else(|| ClientError::MissingField("observation_count".to_string()))?;
            let species_count = item["species_count"].as_i64()
                .ok_or_else(|| ClientError::MissingField("species_count".to_string()))?;

            let user = &item["user"];
            let name = user["name"].as_str()
                .ok_or_else(|| ClientError::MissingField("user.name".to_string()))?
                .to_string();
            let login = user["login"].as_str()
                .ok_or_else(|| ClientError::MissingField("user.login".to_string()))?
                .to_string();

            stats.push(ObserverStats {
                observation_count,
                species_count,
                name,
                login,
                location: location.to_string(),
            });
        }

        Ok(stats)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SpeciesStats {
    pub count: i64,
    pub id: i64,
    pub name: String,
    pub rank: String,
    pub ancestor_ids: Vec<i64>,
    pub location: String,
}

impl SpeciesStats {
    pub fn from_response(response: &Value, location: Option<&str>) -> Result<Vec<Self>> {
        let results_array = response["results"].as_array()
            .ok_or_else(|| ClientError::InvalidResponse("Invalid species response format".to_string()))?;

        let mut stats = Vec::new();
        for item in results_array {
            let count = item["count"].as_i64()
                .ok_or_else(|| ClientError::MissingField("count".to_string()))?;

            let taxon = &item["taxon"];
            let id = taxon["id"].as_i64()
                .ok_or_else(|| ClientError::MissingField("taxon.id".to_string()))?;
            let name = taxon["name"].as_str()
                .ok_or_else(|| ClientError::MissingField("taxon.name".to_string()))?
                .to_string();
            let rank = taxon["rank"].as_str()
                .ok_or_else(|| ClientError::MissingField("taxon.rank".to_string()))?
                .to_string();

            let ancestor_ids = taxon["ancestor_ids"].as_array()
                .map(|arr| arr.iter().filter_map(|v| v.as_i64()).collect())
                .unwrap_or_default();

            stats.push(SpeciesStats {
                count,
                id,
                name,
                rank,
                ancestor_ids,
                location: location.unwrap_or("").to_string(),
            });
        }

        Ok(stats)
    }
}