use crate::Result;
use csv::Writer;
use serde::Serialize;
use std::fmt::Debug;
use std::path::Path;
use std::io::BufWriter;
use std::fs::File;

pub fn save_stats_to_csv<T, P>(stats: &[T], path: P) -> Result<()>
where
    T: Serialize + Debug,
    P: AsRef<Path>,
{
    if stats.is_empty() {
        return Ok(());
    }

    // Use BufWriter for better performance with large datasets
    let file = File::create(path)?;
    let buf_writer = BufWriter::with_capacity(64 * 1024, file); // 64KB buffer
    let mut writer = Writer::from_writer(buf_writer);

    // Write in chunks to avoid memory pressure and provide progress feedback
    const CHUNK_SIZE: usize = 1000;
    let total_records = stats.len();

    println!("Writing {} records to CSV...", total_records);

    for (chunk_idx, chunk) in stats.chunks(CHUNK_SIZE).enumerate() {
        for stat in chunk {
            writer.serialize(stat)?;
        }

        // Flush periodically to ensure data is written and show progress
        writer.flush()?;

        let records_written = std::cmp::min((chunk_idx + 1) * CHUNK_SIZE, total_records);
        if chunk_idx % 10 == 0 || records_written == total_records {
            println!("Progress: {}/{} records written", records_written, total_records);
        }
    }

    // Final flush to ensure all data is written
    writer.flush()?;
    println!("Successfully wrote {} records to CSV", total_records);

    Ok(())
}
