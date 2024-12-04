use figlet_rs::FIGfont;

#[tokio::main]
async fn main() {
    // Generate ASCII art
    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font
        .convert("Megaverse")
        .expect("Failed to generate ASCII art");
    println!("{}", figure);

    println!("Hello, world!");
}
