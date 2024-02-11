
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Spellchecked {
    pub original: String,
    pub spellchecked: String,
}