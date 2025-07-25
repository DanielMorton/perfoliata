use log::{error, info};
use reqwest::Client;
use serde_json::Value;
use std::collections::HashMap;
use tokio::time::sleep;

use crate::api::endpoints::Endpoints;
use crate::client::rate_limiter::RateLimiter;
use crate::error::{ClientError, Result};
use crate::utils::backoff::exponential_backoff;

#[derive(Debug, Clone)]
pub struct RequestHandler {
    client: Client,
}

impl RequestHandler {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    async fn get_data(&self, endpoint: &str, params: &HashMap<String, String>) -> Result<Value> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Accept", "application/json".parse().unwrap());

        let response = self
            .client
            .get(endpoint)
            .query(params)
            .headers(headers)
            .send()
            .await?;

        if response.status().is_success() {
            let json: Value = response.json().await?;
            Ok(json)
        } else if response.status() == 429 {
            Err(ClientError::RateLimit)
        } else {
            Err(ClientError::InvalidResponse(format!(
                "HTTP error: {}",
                response.status()
            )))
        }
    }

    async fn get_data_with_retry(
        &self,
        endpoint: &str,
        params: &HashMap<String, String>,
        rate_limiter: &RateLimiter,
        max_retries: u32,
    ) -> Result<Value> {
        let mut retry_count = 0;

        loop {
            rate_limiter.wait().await;

            match self.get_data(endpoint, params).await {
                Ok(data) => return Ok(data),
                Err(ClientError::RateLimit) => {
                    retry_count += 1;
                    if retry_count > max_retries {
                        error!("Max retries exceeded for endpoint: {endpoint}");
                        return Err(ClientError::RateLimit);
                    }

                    let delay = exponential_backoff(retry_count, 1.0, 60.0);
                    info!("Rate limit hit for endpoint {endpoint}. Retrying in {delay:.2?}...");
                    sleep(delay).await;
                }
                Err(e) => {
                    error!("Error for endpoint {endpoint}: {e}");
                    return Err(e);
                }
            }
        }
    }

    pub async fn get_stats(
        &self,
        endpoint_tag: &str,
        location: Option<&str>,
        extra_params: HashMap<String, String>,
        rate_limiter: &RateLimiter,
    ) -> Result<Value> {
        let endpoint = Endpoints::full_url(endpoint_tag);
        let mut params = HashMap::new();
        params.insert("verifiable".to_string(), "true".to_string());

        if let Some(loc) = location {
            params.insert("place_id".to_string(), loc.to_string());
        }

        for (key, value) in extra_params {
            params.insert(key, value);
        }

        self.get_data_with_retry(&endpoint, &params, rate_limiter, 5)
            .await
    }

    pub async fn get_taxa(
        &self,
        taxon_id: i64,
        extra_params: HashMap<String, String>,
        rate_limiter: &RateLimiter,
    ) -> Result<Value> {
        let endpoint = Endpoints::full_url(Endpoints::taxa());
        let mut params = HashMap::new();
        params.insert("taxon_id".to_string(), taxon_id.to_string());

        for (key, value) in extra_params {
            params.insert(key, value);
        }

        self.get_data_with_retry(&endpoint, &params, rate_limiter, 5)
            .await
    }
}
