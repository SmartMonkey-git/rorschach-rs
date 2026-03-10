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

    pub fn from_type(term: impl Into<Term>) -> Self {
        Self {
            term: term.into(),
            severity: None,
            observed_start: None,
            observed_end: None,
        }
    }

    pub fn set_observed_start(&mut self, observed_start: Option<DateTime<Utc>>) {
        self.observed_start = observed_start;
    }
    pub fn set_observed_end(&mut self, observed_end: Option<DateTime<Utc>>) {
        self.observed_end = observed_end;
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
