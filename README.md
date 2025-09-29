# Perfoliata

A fast, parallel Rust CLI tool for fetching and analyzing biodiversity statistics from the iNaturalist API.

## Features

- **Multiple Statistics Types**: Fetch observation histograms, observer stats, identifier stats, species counts, and taxonomic data
- **Parallel Processing**: Efficiently process multiple locations simultaneously with configurable worker limits
- **Rate Limiting**: Built-in rate limiting to respect API limits and avoid being blocked
- **Flexible Input**: Accept location IDs directly or from CSV files
- **CSV Export**: Save results to CSV format for further analysis
- **Robust Error Handling**: Comprehensive error handling with detailed logging

## Installation

```bash
# Clone the repository
git clone https://github.com/your-username/perfoliata.git
cd perfoliata

# Build the project
cargo build --release

# The binary will be available at target/release/perfoliata
```

## Usage

### Basic Command Structure

```bash
inaturalist-cli [OPTIONS] <COMMAND>
```

### Global Options

- `-r, --rate-limit <RATE>`: Set requests per second for API calls (default: 1.0)
- `-v, --verbose`: Enable verbose logging for debugging

### Commands

The CLI provides four main commands for different types of iNaturalist statistics:

#### Location Statistics (Observation Histograms)

Get monthly observation counts for specific locations:

```bash
inaturalist-cli location-stats --locations 1,2,3 --output results.csv
```

#### Observer Statistics

Get statistics about the most active observers in specified locations:

```bash
inaturalist-cli observer-stats --locations 1,2,3 --output observers.csv
```

#### Identifier Statistics

Get statistics about the most active identifiers in specified locations:

```bash
inaturalist-cli identifier-stats --locations 1,2,3 --output identifiers.csv
```

#### Species Statistics

Get species counts and related statistics for specified locations:

```bash
inaturalist-cli species-stats --locations 1,2,3 --output species.csv
```

### Advanced Usage

#### Using CSV Input

Instead of specifying location IDs directly, you can provide them via CSV:

```bash
inaturalist-cli observer-stats --csv-file locations.csv --csv-column place_id --output results.csv
```

#### Adding API Parameters

Pass additional parameters to the iNaturalist API using key=value format:

```bash
inaturalist-cli species-stats --locations 1,2,3 --params quality_grade=research --params per_page=200 --output species.csv
```

#### Controlling Parallelism

Adjust the number of concurrent workers for processing multiple locations:

```bash
inaturalist-cli observer-stats --locations 1,2,3 --max-workers 8 --output observers.csv
```

#### Rate Limiting

Set a custom rate limit (requests per second):

```bash
inaturalist-cli --rate-limit 0.5 observer-stats --locations 1,2,3 --output observers.csv
```

## Output Format

All commands output CSV files with relevant statistics. Each row typically includes:

- **Location-specific data**: The location ID (if applicable)
- **Statistical data**: Counts, names, and other relevant metrics
- **Metadata**: Additional context depending on the command

### Example Output Formats

**Observer Stats (`observer-stats`)**:
```csv
observation_count,species_count,name,login,location
1250,834,John Doe,johndoe,12345
892,445,Jane Smith,jsmith,12345
```

**Location Stats (`location-stats`)**:
```csv
month,observation_count,location
1,450,12345
2,523,12345
3,672,12345
```

## Error Handling

The tool includes comprehensive error handling for:

- Network connectivity issues
- API rate limiting and HTTP errors
- Invalid JSON responses
- Missing or malformed data fields
- File I/O errors

Errors are logged with appropriate detail levels. Use `--verbose` for debug information.

## Rate Limiting

Perfoliata includes built-in rate limiting to respect the iNaturalist API's usage policies:

- Default: 1 request per second
- Configurable via `--rate-limit` option
- Automatic retry logic for rate-limited requests
- Parallel processing respects the global rate limit

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Dependencies

- `tokio`: Async runtime
- `reqwest`: HTTP client
- `serde`: Serialization/deserialization
- `clap`: Command line argument parsing
- `csv`: CSV reading/writing
- `log` & `env_logger`: Logging functionality

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [iNaturalist](https://www.inaturalist.org/) for providing the API and biodiversity data
- The Rust community for excellent crates and documentation

## API Reference

This tool uses the iNaturalist API. For more information about available parameters and data formats, see:
- [iNaturalist API Documentation](https://www.inaturalist.org/pages/api+reference)

## Troubleshooting

### Common Issues

**Rate Limiting Errors**
- Reduce the `--rate-limit` value
- The tool should automatically handle rate limiting, but manual adjustment may help

**Memory Usage**
- For large datasets, consider processing locations in smaller batches
- Reduce `--max-workers` if experiencing memory issues

**Network Timeouts**
- The tool has a 10-second timeout per request
- Check your internet connection
- Some locations may have large amounts of data that take time to process

### Getting Help

- Check the logs with `--verbose` for detailed error information
- Ensure location IDs are valid iNaturalist place IDs
- Verify CSV files are properly formatted with headers