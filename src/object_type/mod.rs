use std::fmt;

#[derive(Debug)]
pub enum ObjectType {
    Polyanet,
    Soloon,
    Cometh,
}

impl ObjectType {
    pub fn as_url_segment(&self) -> &str {
        match self {
            ObjectType::Polyanet => "polyanets",
            ObjectType::Soloon => "soloons",
            ObjectType::Cometh => "comeths",
        }
    }
}

// Implement the Display trait for ObjectType
impl fmt::Display for ObjectType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            ObjectType::Polyanet => "Polyanet",
            ObjectType::Soloon => "Soloon",
            ObjectType::Cometh => "Cometh",
        };
        write!(f, "{}", name)
    }
}
