use crate::condition::Condition;
use crate::error::RorschachError;
use crate::term::{SeverityTerms, Term};
use crate::traits::AsTerm;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use strum::EnumCount;

#[derive(Debug, Clone)]
pub struct ConditionBuilder {
    term: Term,
    severity: Option<Term>,
    excluded: bool,
    observed_start: Option<DateTime<Utc>>,
    observed_end: Option<DateTime<Utc>>,
}

impl ConditionBuilder {
    pub fn new(term: impl Into<Term>) -> Self {
        Self {
            term: term.into(),
            severity: None,
            excluded: false,
            observed_start: None,
            observed_end: None,
        }
    }

    pub fn severity(mut self, severity: Term) -> Self {
        self.severity = Some(severity);
        self
    }

    pub fn severity_from_max_questions(
        mut self,
        n_answers: i16,
        question_index: i16,
    ) -> Result<Self, RorschachError> {
        if question_index == 0 {
            return Ok(self.exclude());
        }

        if n_answers <= 1 {
            let severity = SeverityTerms::from_category(1)?.as_term();
            self.severity = Some(severity);
            return Ok(self);
        }

        let i = (question_index - 1) as f32;
        let n = (n_answers - 1) as f32;

        let category = ((SeverityTerms::COUNT - 1) as f32 * i / n).round() as i16;

        let severity = SeverityTerms::from_category(category)?.as_term();

        self.severity = Some(severity);
        Ok(self)
    }

    pub fn excluded(mut self, excluded: bool) -> Self {
        self.excluded = excluded;

        if excluded {
            self.severity = None;
        }

        self
    }

    pub fn exclude(self) -> Self {
        let mut new = self.excluded(true);
        new.severity = None;

        new
    }

    pub fn observed_start(mut self, start: DateTime<Utc>) -> Self {
        self.observed_start = Some(start);
        self
    }

    pub fn observed_end(mut self, end: DateTime<Utc>) -> Self {
        self.observed_end = Some(end);
        self
    }

    pub fn build(self) -> Condition {
        Condition::new(
            self.term,
            self.severity,
            self.excluded,
            self.observed_start,
            self.observed_end,
        )
    }
    pub fn build_with_severities(
        self,
        entries: &[(i16, SeverityTerms)],
        excluded: bool,
    ) -> HashMap<i16, Condition> {
        let mut conditions: HashMap<i16, Condition> = entries
            .iter()
            .map(|(idx, s)| (*idx, self.clone().severity(s.into()).build()))
            .collect();

        if excluded {
            conditions.insert(0, self.clone().exclude().build());
        }

        conditions
    }
}

impl Condition {
    pub fn builder(term: Term) -> ConditionBuilder {
        ConditionBuilder::new(term)
    }
}

impl From<ConditionBuilder> for Condition {
    fn from(builder: ConditionBuilder) -> Self {
        builder.build()
    }
}
