use crate::term::Term;
use chrono::{DateTime, Utc};
use std::fmt;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Condition {
    term: Term,
    severity: Option<Term>,
    observed_start: Option<DateTime<Utc>>,
    observed_end: Option<DateTime<Utc>>,
}

impl Condition {
    pub(crate) fn without_time(term: impl Into<Term>, severity: impl Into<Term>) -> Self {
        Self {
            term: term.into(),
            severity: Some(severity.into()),
            observed_start: None,
            observed_end: None,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn new(
        term: impl Into<Term>,
        severity: impl Into<Term>,
        observed_start: Option<DateTime<Utc>>,
        observed_end: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            term: term.into(),
            severity: Some(severity.into()),
            observed_start,
            observed_end,
        }
    }

    pub fn term(&self) -> &Term {
        &self.term
    }
    pub fn severity(&self) -> Option<&Term> {
        self.severity.as_ref()
    }
    pub fn observed_start(&self) -> Option<&DateTime<Utc>> {
        self.observed_start.as_ref()
    }
    pub fn observed_end(&self) -> Option<&DateTime<Utc>> {
        self.observed_end.as_ref()
    }

    pub fn from_type(term: impl Into<Term>) -> Self {
        Self {
            term: term.into(),
            severity: None,
            observed_start: None,
            observed_end: None,
        }
    }

    pub fn set_time(&mut self, observed_start: &DateTime<Utc>, observed_end: &DateTime<Utc>) {
        self.observed_start = Some(*observed_start);
        self.observed_end = Some(*observed_end);
    }

    pub fn set_severity(&mut self, severity: &Term) {
        self.severity = Some(severity.clone());
    }
}

impl fmt::Display for Condition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.severity {
            Some(severity) => write!(f, "{} ({})", self.term, severity),
            None => write!(f, "{}", self.term),
        }
    }
}
