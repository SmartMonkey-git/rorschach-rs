use crate::term::Term;
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TermConfig {
    id: String,
    label: String,
}

impl From<&TermConfig> for Term {
    fn from(value: &TermConfig) -> Self {
        Term::new(value.id.as_str(), value.label.as_str())
    }
}
