use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::error::{ClientError, Result};

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
            let name = match user["name"].as_str() {
                Some(n) if !n.is_empty() => n.to_string(),
                _ => {
                    // Fall back to login if name is null, empty, or missing
                    user["login"].as_str()
                        .unwrap_or("")
                        .to_string()
                }
            };
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