use crate::term::Term;

#[derive(Debug, Clone, PartialEq)]
pub struct Diagnosis {
    term: Term,
    severity: Option<Term>,
}

impl Diagnosis {
    pub(crate) fn new(term: impl Into<Term>, severity: impl Into<Term>) -> Self {
        Self {
            r#term: term.into(),
            severity: Some(severity.into()),
        }
    }

    pub fn from_term(term: impl Into<Term>) -> Self {
        Self {
            term: term.into(),
            severity: None,
        }
    }
}
