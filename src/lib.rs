mod object_type;

use reqwest::Client;
use serde_json::json;
use serde_json::Value; // For parsing dynamic JSON
use tokio::time::{sleep, Duration};

pub use object_type::{Color, Direction, ObjectType};

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

    /// Fetch and display the goal map
    pub async fn show_goal_map(&self) -> Result<(), Box<dyn std::error::Error>> {
        let goal_map = self.fetch_goal_map().await?;
        for row in goal_map {
            let line: Vec<String> = row
                .iter()
                .map(|cell| format!("{}", cell)) // Format each cell with Display implementation
                .collect();
            println!("{}", line.join(" "));
        }
        Ok(())
    }

    /// Create the map based on the goal map
    pub async fn create_map(&self) -> Result<(), Box<dyn std::error::Error>> {
        let goal_map = self.fetch_goal_map().await?;

        for (row_index, row) in goal_map.iter().enumerate() {
            for (col_index, cell) in row.iter().enumerate() {
                match cell {
                    ObjectType::Polyanet => {
                        self.create_object(row_index as u32, col_index as u32, cell)
                            .await?;
                    }
                    ObjectType::Soloon(Some(_)) => {
                        self.create_object(row_index as u32, col_index as u32, cell)
                            .await?;
                    }
                    ObjectType::Cometh(Some(_)) => {
                        self.create_object(row_index as u32, col_index as u32, cell)
                            .await?;
                    }
                    _ => {
                        // Skip Space or invalid objects
                    }
                }
            }
        }

        println!("Map created successfully based on the goal map!");
        Ok(())
    }

    /// Fetch the goal map from the API and parse it into a 2D Vec of ObjectType
    pub async fn fetch_goal_map(&self) -> Result<Vec<Vec<ObjectType>>, Box<dyn std::error::Error>> {
        let url = format!("{}/map/{}/goal", API_URL, self.candidate_id);
        let response = self.client.get(&url).send().await?;

        if response.status().is_success() {
            let goal_map: Value = response.json().await?;
            if let Some(map) = goal_map.get("goal").and_then(|v| v.as_array()) {
                let parsed_map = map
                    .iter()
                    .map(|row| {
                        row.as_array()
                            .unwrap_or(&vec![])
                            .iter()
                            .map(|cell| Self::parse_cell(cell.as_str().unwrap_or("SPACE")))
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

    /// Parse a single cell from the goal map into an ObjectType
    fn parse_cell(cell: &str) -> ObjectType {
        match cell {
            "POLYANET" => ObjectType::Polyanet,
            "BLUE_SOLOON" => ObjectType::Soloon(Some(Color::Blue)),
            "RED_SOLOON" => ObjectType::Soloon(Some(Color::Red)),
            "PURPLE_SOLOON" => ObjectType::Soloon(Some(Color::Purple)),
            "WHITE_SOLOON" => ObjectType::Soloon(Some(Color::White)),
            "UP_COMETH" => ObjectType::Cometh(Some(Direction::Up)),
            "DOWN_COMETH" => ObjectType::Cometh(Some(Direction::Down)),
            "LEFT_COMETH" => ObjectType::Cometh(Some(Direction::Left)),
            "RIGHT_COMETH" => ObjectType::Cometh(Some(Direction::Right)),
            "SPACE" => ObjectType::Space,
            _ => ObjectType::Space,
        }
    }

    /// Create an object (Polyanet, Soloon, or Cometh) at the specified row and column
    pub async fn create_object(
        &self,
        row: u32,
        column: u32,
        object_type: &ObjectType,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let url = match object_type.as_url_segment() {
            Ok(segment) => format!("{}/{}", API_URL, segment),
            Err(e) => return Err(e.into()),
        };

        let payload = match object_type {
            ObjectType::Polyanet => json!({
                "row": row,
                "column": column,
                "candidateId": self.candidate_id
            }),
            ObjectType::Soloon(Some(color)) => json!({
                "row": row,
                "column": column,
                "candidateId": self.candidate_id,
                "color": format!("{}", color).to_lowercase()
            }),
            ObjectType::Cometh(Some(direction)) => json!({
                "row": row,
                "column": column,
                "candidateId": self.candidate_id,
                "direction": format!("{}", direction).to_lowercase()
            }),
            _ => return Err("Invalid object type for creation.".into()),
        };

        loop {
            let response = self.client.post(&url).json(&payload).send().await;

            match response {
                Ok(resp) if resp.status().is_success() => {
                    println!(
                        "Successfully created {} at ({}, {})",
                        object_type, row, column
                    );
                    return Ok(());
                }
                Ok(resp) => {
                    let error_message = resp
                        .text()
                        .await
                        .unwrap_or_else(|_| "Unknown error".to_string());
                    eprintln!(
                        "Failed to create {} at ({}, {}): {}. Retrying...",
                        object_type, row, column, error_message
                    );
                }
                Err(e) => {
                    eprintln!(
                        "Error creating {} at ({}, {}): {}. Retrying...",
                        object_type, row, column, e
                    );
                }
            }

            sleep(Duration::from_secs(1)).await; // Wait before retrying
        }
    }

    /// Delete an object (Polyanet, Soloon, or Cometh) at the specified row and column
    pub async fn delete_object(
        &self,
        row: u32,
        column: u32,
        object_type: ObjectType,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let url = match object_type.as_url_segment() {
            Ok(segment) => format!("{}/{}", API_URL, segment),
            Err(e) => return Err(e.into()),
        };

        loop {
            let payload = json!({
                "row": row,
                "column": column,
                "candidateId": self.candidate_id
            });

            let response = self.client.delete(&url).json(&payload).send().await;

            match response {
                Ok(resp) if resp.status().is_success() => {
                    println!(
                        "Successfully deleted {} at ({}, {})",
                        object_type, row, column
                    );
                    return Ok(());
                }
                Ok(resp) => {
                    let error_message = resp
                        .text()
                        .await
                        .unwrap_or_else(|_| "Unknown error".to_string());
                    eprintln!(
                        "Failed to delete {} at ({}, {}): {}. Retrying...",
                        object_type, row, column, error_message
                    );
                }
                Err(e) => {
                    eprintln!(
                        "Error deleting {} at ({}, {}): {}. Retrying...",
                        object_type, row, column, e
                    );
                }
            }

            sleep(Duration::from_secs(1)).await; // Wait before retrying
        }
    }
}
