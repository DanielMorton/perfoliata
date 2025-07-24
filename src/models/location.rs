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