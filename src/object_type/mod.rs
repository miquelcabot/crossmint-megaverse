use std::str::FromStr;

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

impl FromStr for ObjectType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "POLYANET" => Ok(ObjectType::Polyanet),
            "SOLOON" => Ok(ObjectType::Soloon),
            "COMETH" => Ok(ObjectType::Cometh),
            _ => Err(format!("Invalid object type: {}", s)),
        }
    }
}
