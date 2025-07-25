use std::collections::HashMap;

mod identifier;
mod location;
mod observer;
mod species;
mod stats;

pub use identifier::{handle_identifier_processing, handle_identifier_stats};
pub use location::{handle_location_processing, handle_location_stats};
pub use observer::{handle_observer_processing, handle_observer_stats};
pub use species::{handle_species_processing, handle_species_stats};

/// Convert Vec<(String, String)> to HashMap<String, String>
pub fn params_to_hashmap(params: Vec<(String, String)>) -> HashMap<String, String> {
    params.into_iter().collect()
}
