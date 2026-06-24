use crate::condition::Condition;
use crate::error::RorschachError;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct QuestionnaireItem {
    stem: Option<String>,
    /// Score to condition
    conditions: HashMap<i16, Condition>,
    n_answers: i16,
}

impl QuestionnaireItem {
    pub fn new(
        stem: Option<String>,
        conditions: HashMap<i16, Condition>,
        n_answers: i16,
    ) -> QuestionnaireItem {
        Self {
            stem,
            conditions,
            n_answers,
        }
    }

    pub fn conditions(&self) -> &HashMap<i16, Condition> {
        &self.conditions
    }
    pub fn evaluate(&self, score: i16) -> Result<&Condition, RorschachError> {
        let condition =
            self.conditions
                .get(&score)
                .ok_or(RorschachError::IndexNonExsistingQuestionScore(
                    score,
                    self.stem.clone().unwrap_or("NO-STEM".to_string()),
                ))?;
        Ok(condition)
    }

    pub fn n_answers(&self) -> i16 {
        self.n_answers
    }
}
