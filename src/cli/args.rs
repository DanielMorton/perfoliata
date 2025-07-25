use clap::{Parser, Subcommand};

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
        /// Location ID (place_id)
        #[arg(short, long)]
        location: String,

        /// Additional query parameters (key=value format)
        #[arg(short = 'p', long = "param", value_parser = parse_key_val)]
        params: Vec<(String, String)>,
    },
    /// Get observer statistics for a location
    ObserverStats {
        /// Location ID (place_id)
        #[arg(short, long)]
        location: String,

        /// Additional query parameters (key=value format)
        #[arg(short = 'p', long = "param", value_parser = parse_key_val)]
        params: Vec<(String, String)>,
    },
    /// Get identifier statistics for a location
    IdentifierStats {
        /// Location ID (place_id)
        #[arg(short, long)]
        location: String,

        /// Additional query parameters (key=value format)
        #[arg(short = 'p', long = "param", value_parser = parse_key_val)]
        params: Vec<(String, String)>,
    },
    /// Get species statistics for a location
    SpeciesStats {
        /// Location ID (place_id) - optional for global stats
        #[arg(short, long)]
        location: Option<String>,

        /// Additional query parameters (key=value format)
        #[arg(short = 'p', long = "param", value_parser = parse_key_val)]
        params: Vec<(String, String)>,
    },
    /// Process multiple locations in parallel
    ProcessLocations {
        /// Comma-separated list of location IDs
        #[arg(short, long, value_delimiter = ',')]
        locations: Vec<String>,

        /// Type of statistics to gather
        #[arg(short, long, value_enum)]
        stat_type: StatType,

        /// Maximum number of parallel workers
        #[arg(short = 'w', long, default_value = "4")]
        max_workers: usize,

        /// Additional query parameters (key=value format)
        #[arg(short = 'p', long = "param", value_parser = parse_key_val)]
        params: Vec<(String, String)>,
    },
}

#[derive(clap::ValueEnum, Clone)]
pub enum StatType {
    Location,
    Observer,
    Identifier,
    Species,
}

/// Parse a single key-value pair for parameters
fn parse_key_val(s: &str) -> Result<(String, String), String> {
    let pos = s
        .find('=')
        .ok_or_else(|| format!("invalid KEY=value: no `=` found in `{s}`"))?;
    Ok((s[..pos].to_string(), s[pos + 1..].to_string()))
}
