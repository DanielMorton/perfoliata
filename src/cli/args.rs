use crate::cli::identifier_stats::IdentifierStatsCommand;
use crate::cli::location_stats::LocationStatsCommand;
use crate::cli::observer_stats::ObserverStatsCommand;
use crate::cli::species_stats::SpeciesStatsCommand;
use crate::cli::taxon_stats::TaxaCommand;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "inaturalist-cli")]
#[command(about = "A CLI tool for interacting with the iNaturalist API")]
#[command(version = "1.0")]
pub struct Cli {
    /// API requests per second (rate limiting)
    #[arg(short, long, default_value = "1.0")]
    pub rate_limit: f64,

    /// Enable verbose logging
    #[arg(short, long)]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Get location histogram statistics (monthly observation counts)
    LocationStats(LocationStatsCommand),
    /// Get observer statistics for a location
    ObserverStats(ObserverStatsCommand),
    /// Get identifier statistics for a location
    IdentifierStats(IdentifierStatsCommand),
    /// Get species statistics for a location
    SpeciesStats(SpeciesStatsCommand),
    /// Get taxon list
    Taxa(TaxaCommand),
}

/// Parse a single key-value pair for parameters
pub fn parse_key_val(s: &str) -> Result<(String, String), String> {
    let pos = s
        .find('=')
        .ok_or_else(|| format!("invalid KEY=value: no `=` found in `{s}`"))?;
    Ok((s[..pos].to_string(), s[pos + 1..].to_string()))
}

/// Helper function to read locations from CSV file
pub fn read_locations_from_csv(
    csv_file: &PathBuf,
    column_name: &str,
) -> crate::error::Result<Vec<u32>> {
    use csv::ReaderBuilder;
    use std::fs::File;

    let file = File::open(csv_file)?;
    let mut reader = ReaderBuilder::new().has_headers(true).from_reader(file);

    let headers = reader.headers()?.clone();
    let column_index = headers
        .iter()
        .position(|h| h == column_name)
        .ok_or_else(|| format!("Column '{}' not found in CSV", column_name))?;

    let mut locations = Vec::new();
    for result in reader.records() {
        let record = result?;
        if let Some(value) = record.get(column_index) {
            match value.trim().parse::<u32>() {
                Ok(location_id) => locations.push(location_id),
                Err(e) => eprintln!("Warning: Could not parse '{}' as u32: {}", value, e),
            }
        }
    }

    if locations.is_empty() {
        return Err(format!("No valid location IDs found in column '{}'", column_name).into());
    }

    Ok(locations)
}

/// Helper function to get locations from either CLI args or CSV file
pub fn get_locations(
    locations: &[u32],
    csv_file: &Option<PathBuf>,
    csv_column: &str,
) -> crate::error::Result<Vec<u32>> {
    if let Some(csv_path) = csv_file {
        read_locations_from_csv(csv_path, csv_column)
    } else if !locations.is_empty() {
        Ok(locations.to_vec())
    } else {
        Err("Either --locations or --csv-file must be provided".into())
    }
}
