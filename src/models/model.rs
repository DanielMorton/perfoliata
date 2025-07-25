use crate::{ClientError, INaturalistClient};
use std::collections::HashMap;
use std::sync::Arc;

fn params_to_hashmap(params: Vec<(String, String)>) -> HashMap<String, String> {
    params.into_iter().collect()
}

/// Generic parallel processing function
pub async fn process_stats_parallel<T, F, Fut>(
    client: &INaturalistClient,
    locations: Vec<u32>,
    extra_params: Vec<(String, String)>,
    max_workers: usize,
    stats_fn: F,
) -> Vec<Vec<T>>
where
    F: Fn(INaturalistClient, u32, HashMap<String, String>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<Vec<T>, ClientError>> + Send,
    T: Send + 'static,
{
    let stats_fn = Arc::new(stats_fn);
    let extra_params = Arc::new(params_to_hashmap(extra_params));

    client
        .process_locations_parallel(
            locations,
            {
                let stats_fn = Arc::clone(&stats_fn);
                let extra_params = Arc::clone(&extra_params);
                let client = client.clone();
                move |location| {
                    let stats_fn = Arc::clone(&stats_fn);
                    let extra_params = (*extra_params).clone();
                    let client = client.clone();
                    async move { stats_fn(client, location, extra_params).await }
                }
            },
            Some(max_workers),
        )
        .await
}

