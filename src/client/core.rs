use futures::future::join_all;
use log::error;
use reqwest::Client;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use crate::api::requests::RequestHandler;
use crate::api::endpoints::Endpoints;
use crate::client::rate_limiter::RateLimiter;
use crate::error::{ClientError, Result};
use crate::models::{IdentifierStats, LocationStats, ObserverStats, SpeciesStats};

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

    pub async fn get_location_stats(
        &self,
        location: u32,
        extra_params: HashMap<String, String>,
    ) -> Result<Vec<LocationStats>> {
        let results = self
            .request_handler
            .get_stats(
                Endpoints::OBSERVATIONS_HISTOGRAM,
                Some(location),
                extra_params,
                &self.rate_limiter,
            )
            .await?;

        LocationStats::from_histogram_response(&results, location)
    }

    pub async fn get_observer_stats(
        &self,
        location: u32,
        extra_params: HashMap<String, String>,
    ) -> Result<Vec<ObserverStats>> {
        let results = self
            .request_handler
            .get_stats(
                Endpoints::OBSERVATIONS_OBSERVERS,
                Some(location),
                extra_params,
                &self.rate_limiter,
            )
            .await?;

        ObserverStats::from_response(&results, location)
    }

    pub async fn get_identifier_stats(
        &self,
        location: u32,
        extra_params: HashMap<String, String>,
    ) -> Result<Vec<IdentifierStats>> {
        let results = self
            .request_handler
            .get_stats(
                Endpoints::OBSERVATIONS_IDENTIFIERS,
                Some(location),
                extra_params,
                &self.rate_limiter,
            )
            .await?;

        IdentifierStats::from_response(&results, location)
    }

    pub async fn get_species_stats(
        &self,
        location: Option<u32>,
        extra_params: HashMap<String, String>,
    ) -> Result<Vec<SpeciesStats>> {
        let results = self
            .request_handler
            .get_stats(
                Endpoints::OBSERVATIONS_SPECIES_COUNTS,
                location,
                extra_params,
                &self.rate_limiter,
            )
            .await?;

        SpeciesStats::from_response(&results, location)
    }

    pub async fn process_locations_parallel<T, F, Fut>(
        &self,
        locations: Vec<u32>,
        process_fn: F,
        max_workers: Option<usize>,
    ) -> Vec<T>
    where
        F: Fn(u32) -> Fut + Send + Sync + Clone + 'static,
        Fut: Future<Output = Result<T>> + Send,
        T: Send + 'static,
    {
        let max_workers = max_workers.unwrap_or_else(|| std::cmp::min(4, locations.len()));
        let semaphore = Arc::new(tokio::sync::Semaphore::new(max_workers));

        let tasks: Vec<_> = locations
            .into_iter()
            .map(|location| {
                let semaphore = semaphore.clone();
                let process_fn = process_fn.clone();

                tokio::spawn(async move {
                    let _permit = semaphore.acquire().await.unwrap();
                    process_fn(location).await
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