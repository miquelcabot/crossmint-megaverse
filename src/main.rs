use dialoguer::{theme::ColorfulTheme, Input, Select};
use dotenv::dotenv;
use figlet_rs::FIGfont;
use megaverse::{MegaverseApiClient, ObjectType};
use std::env;

#[tokio::main]
async fn main() {
    // Load environment variables from .env file
    dotenv().ok();

    // Get candidate_id from environment variables
    let candidate_id = match env::var("CANDIDATE_ID") {
        Ok(id) => id,
        Err(_) => {
            eprintln!("Error: CANDIDATE_ID is not set in the .env file");
            std::process::exit(1);
        }
    };

    // Generate ASCII art
    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font
        .convert("Megaverse")
        .expect("Failed to generate ASCII art");
    println!("{}", figure);

    // Create a MegaverseApiClient instance
    let client = MegaverseApiClient::new(&candidate_id);

    // Menu options
    let options = vec![
        "Show goal map",
        "Do a 🪐POLYanet cross",
        "Delete object at specific position",
        "Exit",
    ];

    // Loop to keep the menu active until "Exit" is selected
    loop {
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select an option:")
            .items(&options)
            .default(0)
            .interact()
            .unwrap();

        match selection {
            0 => client.show_goal_map().await.unwrap(),
            1 => client.create_polyanet_cross().await.unwrap(),
            2 => {
                // Get input for row, column, and type of object
                let row: u32 = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter the row")
                    .interact_text()
                    .unwrap();

                let col: u32 = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter the column")
                    .interact_text()
                    .unwrap();

                // Select the object type from a menu
                let object_types = vec!["Polyanet", "Soloon", "Cometh"];
                let object_selection = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("Select the object type:")
                    .items(&object_types)
                    .default(0)
                    .interact()
                    .unwrap();

                let object_type = match object_selection {
                    0 => ObjectType::Polyanet,
                    1 => ObjectType::Soloon,
                    2 => ObjectType::Cometh,
                    _ => unreachable!(),
                };

                client.delete_object(row, col, object_type).await.unwrap();
            }
            3 => {
                println!("Exiting...");
                break; // Exit the loop if "Exit" is selected
            }
            _ => println!("Invalid option"),
        }
    }
}
