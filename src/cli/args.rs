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
    LocationStats {
        /// Location ID(s) (place_id) - can specify multiple with comma separation or multiple flags
        #[arg(short, long, value_delimiter = ',', conflicts_with = "csv_file")]
        locations: Vec<u32>,

        /// CSV file containing locations
        #[arg(long, conflicts_with = "locations")]
        csv_file: Option<PathBuf>,

        /// Column name in CSV file containing location IDs
        #[arg(long, default_value = "id", requires = "csv_file")]
        csv_column: String,

        /// Maximum number of parallel workers when processing multiple locations
        #[arg(short = 'w', long, default_value = "4")]
        max_workers: usize,

        /// Additional query parameters (key=value format)
        #[arg(short = 'p', long = "param", value_parser = parse_key_val)]
        params: Vec<(String, String)>,

        /// Output file path (optional, defaults to stdout)
        #[arg(short, long)]
        output: PathBuf,
    },
    /// Get observer statistics for a location
    ObserverStats {
        /// Location ID(s) (place_id) - can specify multiple with comma separation or multiple flags
        #[arg(short, long, value_delimiter = ',', conflicts_with = "csv_file")]
        locations: Vec<u32>,

        /// CSV file containing locations
        #[arg(long, conflicts_with = "locations")]
        csv_file: Option<PathBuf>,

        /// Column name in CSV file containing location IDs
        #[arg(long, default_value = "id", requires = "csv_file")]
        csv_column: String,

        /// Maximum number of parallel workers when processing multiple locations
        #[arg(short = 'w', long, default_value = "4")]
        max_workers: usize,

        /// Additional query parameters (key=value format)
        #[arg(short = 'p', long = "param", value_parser = parse_key_val)]
        params: Vec<(String, String)>,

        /// Output file path (optional, defaults to stdout)
        #[arg(short, long)]
        output: PathBuf,
    },
    /// Get identifier statistics for a location
    IdentifierStats {
        /// Location ID(s) (place_id) - can specify multiple with comma separation or multiple flags
        #[arg(short, long, value_delimiter = ',', conflicts_with = "csv_file")]
        locations: Vec<u32>,

        /// CSV file containing locations
        #[arg(long, conflicts_with = "locations")]
        csv_file: Option<PathBuf>,

        /// Column name in CSV file containing location IDs
        #[arg(long, default_value = "id", requires = "csv_file")]
        csv_column: String,

        /// Maximum number of parallel workers when processing multiple locations
        #[arg(short = 'w', long, default_value = "4")]
        max_workers: usize,

        /// Additional query parameters (key=value format)
        #[arg(short = 'p', long = "param", value_parser = parse_key_val)]
        params: Vec<(String, String)>,

        /// Output file path (optional, defaults to stdout)
        #[arg(short, long)]
        output: PathBuf,
    },
    /// Get species statistics for a location
    SpeciesStats {
        /// Location ID(s) (place_id) - optional for global stats, can specify multiple with comma separation or multiple flags
        #[arg(short, long, value_delimiter = ',', conflicts_with = "csv_file")]
        locations: Vec<u32>,

        /// CSV file containing locations
        #[arg(long, conflicts_with = "locations")]
        csv_file: Option<PathBuf>,

        /// Column name in CSV file containing location IDs
        #[arg(long, default_value = "id", requires = "csv_file")]
        csv_column: String,

        /// Maximum number of parallel workers when processing multiple locations
        #[arg(short = 'w', long, default_value = "4")]
        max_workers: usize,

        /// Additional query parameters (key=value format)
        #[arg(short = 'p', long = "param", value_parser = parse_key_val)]
        params: Vec<(String, String)>,

        /// Output file path (optional, defaults to stdout)
        #[arg(short, long)]
        output: PathBuf,
    },
}

/// Parse a single key-value pair for parameters
fn parse_key_val(s: &str) -> Result<(String, String), String> {
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
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);

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