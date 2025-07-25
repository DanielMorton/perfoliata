use crate::error::{ClientError, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;

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
        let results_array = response["results"].as_array().ok_or_else(|| {
            ClientError::InvalidResponse("Invalid species response format".to_string())
        })?;

        let mut stats = Vec::new();
        for item in results_array {
            let count = item["count"]
                .as_i64()
                .ok_or_else(|| ClientError::MissingField("count".to_string()))?;

            let taxon = &item["taxon"];
            let id = taxon["id"]
                .as_i64()
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
