use reqwest::Client;
use serde_json::json;

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
