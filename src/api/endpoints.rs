pub struct Endpoints;

impl Endpoints {
    pub const BASE_URL: &'static str = "https://api.inaturalist.org/v1";
    pub const OBSERVATIONS_HISTOGRAM: &'static str = "observations/histogram";
    pub const OBSERVATIONS_OBSERVERS: &'static str = "observations/observers";
    pub const OBSERVATIONS_IDENTIFIERS: &'static str = "observations/identifiers";
    pub const OBSERVATIONS_SPECIES_COUNTS: &'static str = "observations/species_counts";
    pub const TAXA: &'static str = "taxa";

    pub fn full_url(endpoint: &str) -> String {
        format!("{}/{}", Self::BASE_URL, endpoint)
    }
}