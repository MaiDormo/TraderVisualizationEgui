#[derive(PartialEq, Eq)]
pub enum Panel {
    Merged,
    Divided
}

impl Default for Panel {
    fn default() -> Self {
        Self::Merged
    }
}
