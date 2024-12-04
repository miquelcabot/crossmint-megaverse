use reqwest::Client;
use serde_json::json;
use serde_json::Value; // For parsing dynamic JSON

const API_URL: &str = "https://challenge.crossmint.com/api";
const MEGAVERSE_SIZE: u32 = 11;

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

    /// Fetch and display the goal map
    pub async fn show_goal_map(&self) -> Result<(), Box<dyn std::error::Error>> {
        let goal_map = self.fetch_goal_map().await?;
        for row in goal_map {
            let line: Vec<String> = row
                .iter()
                .map(|cell| format!("{:<8}", cell)) // Format each cell to 8 characters
                .collect();
            println!("{}", line.join(" "));
        }
        Ok(())
    }

    /// Create Polyanets in positions defined by the goal map
    pub async fn create_polyanet_cross(&self) -> Result<(), Box<dyn std::error::Error>> {
        let goal_map = self.fetch_goal_map().await?;
        for (row_index, row) in goal_map.iter().enumerate() {
            for (col_index, cell) in row.iter().enumerate() {
                if cell == "POLYANET" {
                    // Create a POLYANET at this position
                    self.create_polyanet(row_index as u32, col_index as u32)
                        .await?;
                }
            }
        }
        println!("Polyanets created successfully based on the goal map!");
        Ok(())
    }

    /// Reset the Megaverse by deleting all Polyanets
    pub async fn reset_megaverse(&self) -> Result<(), Box<dyn std::error::Error>> {
        for row in 0..MEGAVERSE_SIZE {
            for column in 0..MEGAVERSE_SIZE {
                self.delete_polyanet(row, column).await?;
            }
        }
        println!("Megaverse reset successfully!");
        Ok(())
    }

    /// Fetch the goal map from the API
    async fn fetch_goal_map(&self) -> Result<Vec<Vec<String>>, Box<dyn std::error::Error>> {
        let url = format!("{}/map/{}/goal", API_URL, self.candidate_id);
        let response = self.client.get(&url).send().await?;

        if response.status().is_success() {
            let goal_map: Value = response.json().await?;
            if let Some(map) = goal_map.get("goal").and_then(|v| v.as_array()) {
                // Convert the JSON 2D array into a Vec<Vec<String>>
                let parsed_map = map
                    .iter()
                    .map(|row| {
                        row.as_array()
                            .unwrap_or(&vec![])
                            .iter()
                            .map(|cell| cell.as_str().unwrap_or("UNKNOWN").to_string())
                            .collect()
                    })
                    .collect();
                Ok(parsed_map)
            } else {
                Err("Failed to parse the goal map field".into())
            }
        } else {
            Err(format!(
                "Failed to fetch goal map: {}",
                response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Unknown error".to_string())
            )
            .into())
        }
    }

    /// Create a Polyanet at the specified row and column
    async fn create_polyanet(
        &self,
        row: u32,
        column: u32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!("{}/polyanets", API_URL);
        let payload = json!({
            "row": row,
            "column": column,
            "candidateId": self.candidate_id
        });

        let response = self.client.post(&url).json(&payload).send().await?;

        if response.status().is_success() {
            println!("Successfully created Polyanet at ({}, {})", row, column);
        } else {
            eprintln!(
                "Failed to create Polyanet at ({}, {}): {}",
                row,
                column,
                response.text().await?
            );
        }
        Ok(())
    }

    /// Delete a Polyanet at the specified row and column
    async fn delete_polyanet(
        &self,
        row: u32,
        column: u32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!("{}/polyanets", API_URL);
        let payload = json!({
            "row": row,
            "column": column,
            "candidateId": self.candidate_id
        });

        let response = self.client.delete(&url).json(&payload).send().await?;

        if response.status().is_success() {
            println!("Successfully deleted Polyanet at ({}, {})", row, column);
        } else {
            eprintln!(
                "Failed to delete Polyanet at ({}, {}): {}",
                row,
                column,
                response.text().await?
            );
        }
        Ok(())
    }
}
