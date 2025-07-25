use crate::ClientError;
use csv::Writer;
use serde::Serialize;
use std::path::Path;

// Save any serializable stats collection to CSV
pub fn save_stats_to_csv<T, P>(stats: &[T], path: P) -> Result<(), ClientError>
where
    T: Serialize,
    P: AsRef<Path>,
{
    let mut writer = Writer::from_path(path)?;

    for stat in stats {
        writer.serialize(stat)?;
    }

    writer.flush()?;

    Ok(())
}
