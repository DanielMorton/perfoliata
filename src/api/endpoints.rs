pub struct Endpoints;

impl Endpoints {
    pub const BASE_URL: &'static str = "https://api.inaturalist.org/v1";

    pub fn observations_histogram() -> &'static str {
        "observations/histogram"
    }

    pub fn observations_observers() -> &'static str {
        "observations/observers"
    }

    pub fn observations_identifiers() -> &'static str {
        "observations/identifiers"
    }

    pub fn observations_species_counts() -> &'static str {
        "observations/species_counts"
    }

    pub fn taxa() -> &'static str {
        "taxa"
    }

    pub fn full_url(endpoint: &str) -> String {
        format!("{}/{}", Self::BASE_URL, endpoint)
    }
}
