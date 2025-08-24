#[derive(Debug)]
pub enum OwnershipForm {
    State,
    Municipal,
    Corporate,
    Private,
    Unknown,
}

impl From<&str> for OwnershipForm {
    fn from(value: &str) -> Self {
        match value {
            "Державна" => Self::State,
            "Комунальна" => Self::Municipal,
            "Корпоративна" => Self::Corporate,
            "Приватна" => Self::Private,
            _ => Self::Unknown,
        }
    }
}
