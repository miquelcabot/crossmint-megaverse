use reqwest::Client;
use serde_json::json;

const API_URL: &str = "https://challenge.crossmint.com/api";

#[derive(Debug)]
pub struct MegaverseApiClient {
    client: Client,
    candidate_id: String,
}

impl MegaverseApiClient {
    /// Create a new MegaverseApiClient instance
    pub fn new(candidate_id: &str) -> Self {
        Self {
            client: Client::new(),
            candidate_id: candidate_id.to_string(),
        }
    }
}
