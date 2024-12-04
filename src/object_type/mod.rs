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
            ObjectType::Space => write!(f, "Space"),
            ObjectType::Polyanet => write!(f, "Polyanet"),
            ObjectType::Soloon(color) => match color {
                Some(c) => write!(f, "Soloon ({})", c),
                None => write!(f, "Soloon"),
            },
            ObjectType::Cometh(direction) => match direction {
                Some(d) => write!(f, "Cometh ({})", d),
                None => write!(f, "Cometh"),
            },
        }
    }
}
