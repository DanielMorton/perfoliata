use crate::{ClientError, INaturalistClient};
use indicatif::{ProgressBar, ProgressStyle};
use std::collections::HashMap;
use std::future::Future;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;

fn params_to_hashmap(params: Vec<(String, String)>) -> HashMap<String, String> {
    params.into_iter().collect()
}

/// Generic parallel processing function with progress bar
pub async fn process_stats_parallel<T, F, Fut>(
    client: &INaturalistClient,
    locations: &[u32],
    extra_params: Vec<(String, String)>,
    max_workers: usize,
    stats_fn: F,
) -> Vec<Vec<T>>
where
    F: Fn(INaturalistClient, Option<u32>, HashMap<String, String>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<Vec<T>, ClientError>> + Send,
    T: Send + 'static,
{
    let extra_params = Arc::new(params_to_hashmap(extra_params));
    let stats_fn = Arc::new(stats_fn);

    // Handle empty locations (global query) without progress bar
    if locations.is_empty() {
        let client_clone = client.clone();
        let extra_params = (*extra_params).clone();

        println!("Processing global query...");
        let start_time = Instant::now();

        let result = client_clone
            .process_parallel(
                locations,
                {
                    let stats_fn = Arc::clone(&stats_fn);
                    let client_for_closure = client.clone();
                    move |location| {
                        let stats_fn = Arc::clone(&stats_fn);
                        let extra_params = extra_params.clone();
                        let client = client_for_closure.clone();

                        async move { stats_fn(client, location, extra_params).await }
                    }
                },
                Some(max_workers),
            )
            .await;

        println!(
            "Global query completed in {}",
            format_duration(start_time.elapsed())
        );

        return result;
    }

    // Process multiple locations with progress bar
    let total_locations = locations.len();

    // Create progress bar
    let progress_bar = ProgressBar::new(total_locations as u64);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({per_sec}) {eta_precise}")
            .unwrap()
            .progress_chars("#>-"),
    );
    progress_bar.set_message("Processing locations");

    // Track timing for ETA calculation
    let start_time = Instant::now();
    let completed_count = Arc::new(Mutex::new(0usize));

    let result = client
        .process_parallel(
            locations,
            {
                let stats_fn = Arc::clone(&stats_fn);
                let extra_params = Arc::clone(&extra_params);
                let client = client.clone();
                let progress_bar = progress_bar.clone();
                let completed_count = Arc::clone(&completed_count);

                move |location| {
                    let stats_fn = Arc::clone(&stats_fn);
                    let extra_params = (*extra_params).clone();
                    let client = client.clone();
                    let progress_bar = progress_bar.clone();
                    let completed_count = Arc::clone(&completed_count);

                    async move {
                        let result = stats_fn(client, location, extra_params).await;

                        // Update progress
                        let mut count = completed_count.lock().await;
                        *count += 1;
                        let completed = *count;
                        drop(count);

                        progress_bar.inc(1);

                        // Update ETA calculation
                        if completed > 0 {
                            let elapsed = start_time.elapsed();
                            let rate = completed as f64 / elapsed.as_secs_f64();
                            let remaining = total_locations - completed;

                            if rate > 0.0 {
                                let eta = Duration::from_secs_f64(remaining as f64 / rate);
                                progress_bar.set_message(format!(
                                    "Processing locations (ETA: {})",
                                    format_duration(eta)
                                ));
                            }
                        }

                        result
                    }
                }
            },
            Some(max_workers),
        )
        .await;

    // Finish progress bar
    progress_bar.finish_with_message(format!(
        "Completed {} locations in {}",
        total_locations,
        format_duration(start_time.elapsed())
    ));

    result
}

/// Format duration in a human-readable way
fn format_duration(duration: Duration) -> String {
    let total_seconds = duration.as_secs();
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;

    if hours > 0 {
        format!("{}h {}m {}s", hours, minutes, seconds)
    } else if minutes > 0 {
        format!("{}m {}s", minutes, seconds)
    } else {
        format!("{}s", seconds)
    }
}
