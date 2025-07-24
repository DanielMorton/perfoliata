use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub total_results: Option<i64>,
    pub page: Option<i32>,
    pub per_page: Option<i32>,
    pub results: T,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub login: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Taxon {
    pub id: i64,
    pub name: String,
    pub rank: String,
    pub ancestor_ids: Vec<i64>,
}
