use dialoguer::{theme::ColorfulTheme, Select};
use dotenv::dotenv;
use figlet_rs::FIGfont;
use megaverse::MegaverseApiClient;
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
        "Reset Megaverse",
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
            2 => client.reset_megaverse().await.unwrap(),
            3 => {
                println!("Exiting...");
                break; // Exit the loop if "Exit" is selected
            }
            _ => println!("Invalid option"),
        }
    }
}
