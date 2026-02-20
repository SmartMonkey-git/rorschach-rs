use crate::term::Term;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Phenotype {
    r#type: Term,
    severity: Option<Term>,
}

impl Phenotype {
    pub(crate) fn new(r#type: impl Into<Term>, severity: impl Into<Term>) -> Self {
        Self {
            r#type: r#type.into(),
            severity: Some(severity.into()),
        }
    }

    pub fn from_type(r#type: impl Into<Term>) -> Self {
        Self {
            r#type: r#type.into(),
            severity: None,
        }
    }
}
