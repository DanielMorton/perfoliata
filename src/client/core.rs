use futures::future::join_all;
use log::error;
use reqwest::Client;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use crate::api::endpoints::Endpoints;
use crate::api::requests::RequestHandler;
use crate::client::rate_limiter::RateLimiter;
use crate::error::{ClientError, Result};
use crate::models::{
    ObservationHistogramStats, ObservationIdentifierStats, ObservationObserverStats,
    ObservationSpeciesStats, Taxon,
};

#[derive(Debug, Clone)]
pub struct INaturalistClient {
    request_handler: RequestHandler,
    rate_limiter: RateLimiter,
}

impl INaturalistClient {
    pub fn new(requests_per_second: f64) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .map_err(ClientError::Http)?;

        Ok(Self {
            request_handler: RequestHandler::new(client),
            rate_limiter: RateLimiter::new(requests_per_second),
        })
    }

    pub async fn get_observation_histogram(
        &self,
        location: Option<u32>,
        extra_params: &HashMap<String, String>,
    ) -> Result<Vec<ObservationHistogramStats>> {
        let results = self
            .request_handler
            .get_stats(
                Endpoints::OBSERVATIONS_HISTOGRAM,
                location,
                extra_params,
                &self.rate_limiter,
            )
            .await?;

        ObservationHistogramStats::from_histogram_response(&results, location)
    }

    pub async fn get_observer_stats(
        &self,
        location: Option<u32>,
        extra_params: &HashMap<String, String>,
    ) -> Result<Vec<ObservationObserverStats>> {
        let results = self
            .request_handler
            .get_stats(
                Endpoints::OBSERVATIONS_OBSERVERS,
                location,
                extra_params,
                &self.rate_limiter,
            )
            .await?;

        ObservationObserverStats::from_response(&results, location)
    }

    pub async fn get_identifier_stats(
        &self,
        location: Option<u32>,
        extra_params: &HashMap<String, String>,
    ) -> Result<Vec<ObservationIdentifierStats>> {
        let results = self
            .request_handler
            .get_stats(
                Endpoints::OBSERVATIONS_IDENTIFIERS,
                location,
                extra_params,
                &self.rate_limiter,
            )
            .await?;

        ObservationIdentifierStats::from_response(&results, location)
    }

    pub async fn get_species_stats(
        &self,
        location: Option<u32>,
        extra_params: &HashMap<String, String>,
    ) -> Result<Vec<ObservationSpeciesStats>> {
        let results = self
            .request_handler
            .get_stats(
                Endpoints::OBSERVATIONS_SPECIES_COUNTS,
                location,
                extra_params,
                &self.rate_limiter,
            )
            .await?;

        ObservationSpeciesStats::from_response(&results, location)
    }

    pub async fn get_taxa_stats(
        &self,
        id_start: Option<u32>,
        extra_params: &HashMap<String, String>,
    ) -> Result<Vec<Taxon>> {
        let mut params = extra_params.clone();
        if let Some(start) = id_start {
            let end = start + (*extra_params.get("per_page").unwrap()).parse::<u32>().unwrap() - 1;
            params.insert("id_above".to_string(), start.to_string());
            params.insert("id_below".to_string(), end.to_string());
        }
        let results = self
            .request_handler
            .get_stats(Endpoints::TAXA, None, &params, &self.rate_limiter)
            .await?;

        Taxon::from_response(&results)
    }

    pub async fn process_parallel<T, F, Fut>(
        &self,
        locations: &[u32],
        process_fn: F,
        max_workers: Option<usize>,
    ) -> Vec<T>
    where
        F: Fn(Option<u32>) -> Fut + Send + Sync + Clone + 'static,
        Fut: Future<Output = Result<T>> + Send,
        T: Send + 'static,
    {
        if locations.is_empty() {
            // Empty list - make single global request
            match process_fn(None).await {
                Ok(data) => vec![data],
                Err(e) => {
                    error!("Global query failed: {e}");
                    vec![]
                }
            }
        } else {
            // Process multiple locations in parallel
            let max_workers = max_workers.unwrap_or_else(|| std::cmp::min(4, locations.len()));
            let semaphore = Arc::new(tokio::sync::Semaphore::new(max_workers));

            let tasks: Vec<_> = locations
                .into_iter()
                .map(|&location| {
                    let semaphore = semaphore.clone();
                    let process_fn = process_fn.clone();

                    tokio::spawn(async move {
                        let _permit = semaphore.acquire().await.unwrap();
                        process_fn(Some(location)).await
                    })
                })
                .collect();

            let results = join_all(tasks).await;

            results
                .into_iter()
                .filter_map(|result| match result {
                    Ok(Ok(data)) => Some(data),
                    Ok(Err(e)) => {
                        error!("Task failed: {e}");
                        None
                    }
                    Err(e) => {
                        error!("Task panicked: {e}");
                        None
                    }
                })
                .collect()
        }
    }
}
