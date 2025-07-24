use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::error::{ClientError, Result};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LocationStats {
    pub year: i32,
    pub observation_count: i64,
    pub location: String,
}

impl LocationStats {
    pub fn from_histogram_response(response: &Value, location: &str) -> Result<Vec<Self>> {
        // Debug: Print the actual response structure to understand the format
        println!("DEBUG - Full response: {}", serde_json::to_string_pretty(response).unwrap_or_else(|_| "Failed to serialize".to_string()));

        // The histogram API returns an object where keys are time periods and values are counts
        if let Some(results_obj) = response["results"].as_object() {
            let mut stats = Vec::new();

            println!("DEBUG - Results object keys: {:?}", results_obj.keys().collect::<Vec<_>>());

            for (key, count) in results_obj {
                // Skip non-numeric keys that might be metadata
                if !key.chars().next().unwrap_or('a').is_ascii_digit() {
                    println!("DEBUG - Skipping non-numeric key: {}", key);
                    continue;
                }

                println!("DEBUG - Processing key: {} with value: {:?}", key, count);

                // Handle different date formats: "2023", "2023-01", "2023-01-01", etc.
                let year = if key.contains('-') {
                    // Extract year from date format like "2023-01" or "2023-01-01"
                    key.split('-').next()
                        .ok_or_else(|| ClientError::Parse("Invalid date format".to_string()))?
                        .parse::<i32>()
                        .map_err(|e| ClientError::Parse(format!("Failed to parse year from '{}': {}", key, e)))?
                } else {
                    // Direct year format like "2023"
                    key.parse::<i32>()
                        .map_err(|e| ClientError::Parse(format!("Failed to parse year from '{}': {}", key, e)))?
                };

                let observation_count = count.as_i64()
                    .ok_or_else(|| ClientError::InvalidResponse(format!("Invalid count format for date '{}': expected number, got {:?}", key, count)))?;

                stats.push(LocationStats {
                    year,
                    observation_count,
                    location: location.to_string(),
                });
            }

            Ok(stats)
        } else if let Some(results_array) = response["results"].as_array() {
            println!("DEBUG - Results is an array with {} elements", results_array.len());
            // Alternative: API might return an array of objects
            let mut stats = Vec::new();

            for (index, count) in results_array.iter().enumerate() {
                let observation_count = count.as_i64()
                    .ok_or_else(|| ClientError::InvalidResponse(format!("Invalid count at index {}: expected number, got {:?}", index, count)))?;

                // If it's an array, we might need additional context for the year
                // For now, use the index as a placeholder - this might need adjustment based on actual API behavior
                stats.push(LocationStats {
                    year: index as i32, // This is likely wrong - may need to be adjusted
                    observation_count,
                    location: location.to_string(),
                });
            }

            Ok(stats)
        } else {
            Err(ClientError::InvalidResponse(format!("Unexpected histogram response format. Results field type: {:?}", response["results"])))
        }
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