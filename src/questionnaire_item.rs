use crate::condition::Condition;
use crate::error::RorschachError;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct QuestionnaireItem {
    stem: Option<String>,
    /// Score to condition
    conditions: HashMap<i16, Option<Condition>>,
    n_answers: i16,
}

impl QuestionnaireItem {
    pub fn new(
        stem: Option<String>,
        conditions: HashMap<i16, Option<Condition>>,
        n_answers: i16,
    ) -> Result<QuestionnaireItem, RorschachError> {
        if conditions.is_empty() {
            return Err(RorschachError::BuildingError(format!(
                "Missing conditions for '{:?}'",
                stem
            )));
        }
        Ok(Self {
            stem,
            conditions,
            n_answers,
        })
    }

    pub fn conditions(&self) -> &HashMap<i16, Option<Condition>> {
        &self.conditions
    }
    pub fn evaluate(&self, score: i16) -> Result<Option<&Condition>, RorschachError> {
        let condition =
            self.conditions
                .get(&score)
                .ok_or(RorschachError::IndexNonExsistingQuestionScore(
                    score,
                    self.stem.clone().unwrap_or("NO-STEM".to_string()),
                ))?;
        Ok(condition.as_ref())
    }

    pub fn n_answers(&self) -> i16 {
        self.n_answers
    }

    pub fn max_score(&self) -> i16 {
        *self.conditions().keys().max().unwrap()
    }
}
