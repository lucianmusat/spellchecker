
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Spellchecked {
    pub original: String,
    pub spellchecked: String,
}

impl PartialEq for Spellchecked {
    fn eq(&self, other: &Self) -> bool {
        self.original == other.original && self.spellchecked == other.spellchecked
    }
}
