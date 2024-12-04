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
        let url = format!("{}/map/{}/goal", API_URL, self.candidate_id);
        let response = self.client.get(&url).send().await?;

        if response.status().is_success() {
            let goal_map: Value = response.json().await?;
            // Extract the "goal" field, which is a 2D array
            if let Some(map) = goal_map.get("goal").and_then(|v| v.as_array()) {
                for row in map {
                    if let Some(cells) = row.as_array() {
                        let line: Vec<String> = cells
                            .iter()
                            .map(|cell| {
                                // Format each cell to be 8 characters wide
                                format!("{:<8}", cell.as_str().unwrap_or("UNKNOWN"))
                            })
                            .collect();
                        println!("{}", line.join(" "));
                    }
                }
            } else {
                println!("Failed to parse goal map: {:?}", goal_map);
            }
        } else {
            eprintln!(
                "Failed to fetch goal map: {}",
                response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Unknown error".to_string())
            );
        }
        Ok(())
    }

    /// Create a Polyanet cross in the Megaverse
    pub async fn create_polyanet_cross(&self) -> Result<(), Box<dyn std::error::Error>> {
        for i in 0..11 {
            // Diagonal principal
            self.create_polyanet(i, i).await?;
            // Diagonal secundària
            self.create_polyanet(i, 10 - i).await?;
        }
        println!("Polyanet cross created successfully!");
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
