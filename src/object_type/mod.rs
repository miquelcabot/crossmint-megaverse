use std::fmt;

/// Enum for colors used by Soloon
#[derive(Debug)]
pub enum Color {
    Blue,
    Red,
    Purple,
    White,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let color_name = match self {
            Color::Blue => "Blue",
            Color::Red => "Red",
            Color::Purple => "Purple",
            Color::White => "White",
        };
        write!(f, "{}", color_name)
    }
}

/// Enum for directions used by Cometh
#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let direction_name = match self {
            Direction::Up => "Up",
            Direction::Down => "Down",
            Direction::Left => "Left",
            Direction::Right => "Right",
        };
        write!(f, "{}", direction_name)
    }
}

/// Enum for Object Types in the Megaverse
#[derive(Debug)]
pub enum ObjectType {
    Space,
    Polyanet,
    Soloon(Option<Color>),
    Cometh(Option<Direction>),
}

impl ObjectType {
    /// Returns the URL segment for the object type
    pub fn as_url_segment(&self) -> Result<&str, &'static str> {
        match self {
            ObjectType::Space => Err("Space objects do not have a URL segment."),
            ObjectType::Polyanet => Ok("polyanets"),
            ObjectType::Soloon(_) => Ok("soloons"),
            ObjectType::Cometh(_) => Ok("comeths"),
        }
    }
}

// Implement Display for ObjectType
impl fmt::Display for ObjectType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ObjectType::Space => write!(f, "🌌"),    // Space icon
            ObjectType::Polyanet => write!(f, "🪐"), // Polyanet icon
            ObjectType::Soloon(color) => match color {
                Some(Color::Blue) => write!(f, "🔵"),   // Blue Soloon icon
                Some(Color::Red) => write!(f, "🔴"),    // Red Soloon icon
                Some(Color::Purple) => write!(f, "🟣"), // Purple Soloon icon
                Some(Color::White) => write!(f, "⚪"),  // White Soloon icon
                None => write!(f, "🌕"),                // Generic Soloon icon
            },
            ObjectType::Cometh(direction) => match direction {
                Some(Direction::Up) => write!(f, "☄️↑"),    // Cometh up icon
                Some(Direction::Down) => write!(f, "☄️↓"),  // Cometh down icon
                Some(Direction::Left) => write!(f, "☄️←"),  // Cometh left icon
                Some(Direction::Right) => write!(f, "☄️→"), // Cometh right icon
                None => write!(f, "☄️"),                    // Generic Cometh icon
            },
        }
    }
}
