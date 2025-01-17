#[derive(Debug, Clone)]
pub enum Type {
    Ping,
    Exec,
    Note,
    Event,
    NotFound
}
impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Ping => write!(f, "Ping"),
            Self::Exec => write!(f, "Exec"),
            Self::Note => write!(f, "Note"),
            Self::Event => write!(f, "Event"),
            Self::NotFound => write!(f, "NotFound"),
        }
    }
}