use crate::term::Term;
use std::fmt;

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

    pub fn term(&self) -> &Term {
        &self.term
    }
    pub fn severity(&self) -> Option<&Term> {
        self.severity.as_ref()
    }
}

impl fmt::Display for Diagnosis {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.severity {
            Some(severity) => write!(f, "{} ({})", self.term, severity),
            None => write!(f, "{}", self.term),
        }
    }
}
