use crate::condition::Condition;
use chrono::{DateTime, Utc};
use std::collections::HashSet;
use std::fmt;

#[derive(Debug)]
pub struct QuestionnaireResult {
    id: String,
    diagnosis: Option<Condition>,
    phenotypes: HashSet<Condition>,
    taken_at: Option<DateTime<Utc>>,
}

impl QuestionnaireResult {
    pub fn new(
        id: impl Into<String>,
        diagnosis: Option<Condition>,
        phenotypes: HashSet<Condition>,
        taken_at: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            id: id.into(),
            diagnosis: diagnosis.clone(),
            phenotypes,
            taken_at,
        }
    }
}

impl fmt::Display for QuestionnaireResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "╭─────────────────────────────────────────╮")?;
        writeln!(f, "│       ✦  Questionnaire Result  ✦        │")?;
        writeln!(f, "╰─────────────────────────────────────────╯")?;
        writeln!(f)?;

        writeln!(f, "  ID:        {}", self.id)?;

        writeln!(f)?;

        match &self.taken_at {
            Some(date) => writeln!(f, "  Taken on: {}", date)?,
            None => writeln!(f, "  Taken on: Unknown")?,
        }

        writeln!(f)?;

        match &self.diagnosis {
            Some(diag) => writeln!(f, "  Diagnosis:  {}", diag)?,
            None => writeln!(f, "  Diagnosis:  No diagnosis recorded yet.")?,
        }

        writeln!(f)?;

        writeln!(f, "  Observed Phenotypes:")?;
        if self.phenotypes.is_empty() {
            writeln!(f, "       None recorded.")?;
        } else {
            for phenotype in &self.phenotypes {
                writeln!(f, "       •  {}", phenotype)?;
            }
        }

        Ok(())
    }
}
