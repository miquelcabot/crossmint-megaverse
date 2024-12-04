use dialoguer::{theme::ColorfulTheme, Input, Select};
use figlet_rs::FIGfont;
use megaverse::MegaverseApiClient;

#[tokio::main]
async fn main() {
    // Generate ASCII art
    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font
        .convert("Megaverse")
        .expect("Failed to generate ASCII art");
    println!("{}", figure);

    // Menu options
    let options = vec!["Do a 🪐POLYanet cross", "Reset Megaverse", "Exit"];

    // Loop to keep the menu active until "Exit" is selected
    loop {
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select an option:")
            .items(&options)
            .default(0)
            .interact()
            .unwrap();

        // Create a MegaverseApiClient instance
        let client = MegaverseApiClient::new("your_candidate_id_here");

        match selection {
            0 => todo!("Do a 🪐POLYanet cross"),
            1 => todo!("Reset Megaverse"),
            2 => {
                println!("Exiting...");
                break; // Exit the loop if "Exit" is selected
            }
            _ => println!("Invalid option"),
        }
    }
}
