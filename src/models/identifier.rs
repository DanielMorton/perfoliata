use crate::models::model::process_stats_parallel;
use crate::{ClientError, INaturalistClient};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IdentifierStats {
    pub identification_count: i64,
    pub species_count: i64,
    pub name: String,
    pub login: String,
    pub location: String,
}

impl IdentifierStats {
    pub fn from_response(response: &Value, location: &str) -> crate::Result<Vec<Self>> {
        let results_array = response["results"].as_array().ok_or_else(|| {
            ClientError::InvalidResponse("Invalid identifiers response format".to_string())
        })?;

        let mut stats = Vec::new();
        for item in results_array {
            let identification_count = item["identification_count"]
                .as_i64()
                .ok_or_else(|| ClientError::MissingField("identification_count".to_string()))?;
            let species_count = item["species_count"]
                .as_i64()
                .ok_or_else(|| ClientError::MissingField("species_count".to_string()))?;

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

            stats.push(IdentifierStats {
                identification_count,
                species_count,
                name,
                login,
                location: location.to_string(),
            });
        }

        Ok(stats)
    }
}

/// Handle identifier parallel processing
pub async fn handle_identifier_processing(
    client: &INaturalistClient,
    locations: Vec<String>,
    extra_params: Vec<(String, String)>,
    max_workers: usize,
) -> Vec<Vec<IdentifierStats>> {
    process_stats_parallel(
        client,
        locations,
        extra_params,
        max_workers,
        |client, location, params| async move {
            client.get_identifier_stats(&location, params).await
        },
    ).await
}
