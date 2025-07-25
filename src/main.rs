use clap::Parser;
use log::info;
use perfoliata::cli::{Cli, execute_command};
use perfoliata::{ClientError, INaturalistClient};

#[tokio::main]
async fn main() -> Result<(), ClientError> {
    let cli = Cli::parse();

    // Initialize logging
    if cli.verbose {
        env_logger::Builder::from_default_env()
            .filter_level(log::LevelFilter::Debug)
            .init();
    } else {
        env_logger::Builder::from_default_env()
            .filter_level(log::LevelFilter::Info)
            .init();
    }

    // Create client with specified rate limit
    let client = INaturalistClient::new(cli.rate_limit)?;
    info!(
        "Created iNaturalist client with rate limit: {} req/s",
        cli.rate_limit
    );

    let _ = execute_command(&client, cli.command).await;
    Ok(())
}
