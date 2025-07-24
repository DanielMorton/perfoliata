//! iNaturalist API Client Library
//!
//! A Rust client for interacting with the iNaturalist API with rate limiting,
//! retry logic, and parallel processing capabilities.

pub mod api;
pub mod client;
pub mod error;
pub mod models;
pub mod utils;
pub mod cli;

// Re-export main types for easier use
pub use client::INaturalistClient;
pub use error::{ClientError, Result};
pub use models::stats::{LocationStats, ObserverStats, SpeciesStats};

// Re-export commonly used types
pub use std::collections::HashMap;
