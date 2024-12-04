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
