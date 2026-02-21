use crate::diagnosis::Diagnosis;
use crate::phenotype::Phenotype;
use chrono::{DateTime, Utc};
use std::collections::HashSet;
use std::fmt;

#[derive(Debug)]
pub struct QuestionnaireResult {
    id: String,
    diagnosis: Option<Diagnosis>,
    phenotypes: HashSet<Phenotype>,
    taken_at: Option<DateTime<Utc>>,
}

impl QuestionnaireResult {
    pub fn new(
        id: impl Into<String>,
        diagnosis: Option<Diagnosis>,
        phenotypes: HashSet<Phenotype>,
        taken_at: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            id: id.into(),
            diagnosis: diagnosis.clone(),
            phenotypes,
            taken_at,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn set_taken_at(&mut self, taken_at: Option<DateTime<Utc>>) {
        self.taken_at = taken_at;
    }
    pub fn push_phenotypes(&mut self, terms: Vec<Phenotype>) {
        self.phenotypes.extend(terms);
    }

    pub fn phenotypes(&self) -> &HashSet<Phenotype> {
        &self.phenotypes
    }
    pub fn diagnosis(&self) -> Option<&Diagnosis> {
        self.diagnosis.as_ref()
    }

    pub fn taken_at(&self) -> Option<DateTime<Utc>> {
        self.taken_at
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
