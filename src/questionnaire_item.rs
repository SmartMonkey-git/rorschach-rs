use crate::condition::Condition;
use crate::error::RorschachError;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct QuestionnaireItem {
    stem: Option<String>,
    conditions: HashMap<usize, HashMap<i16, Condition>>,
    n_answers: i16,
}

impl QuestionnaireItem {
    pub fn new(
        stem: Option<String>,
        conditions: HashMap<usize, HashMap<i16, Condition>>,
        n_answers: i16,
    ) -> QuestionnaireItem {
        Self {
            stem,
            conditions,
            n_answers,
        }
    }

    pub fn conditions(&self, question_index: usize) -> Option<&HashMap<i16, Condition>> {
        self.conditions.get(&question_index)
    }
    pub fn evaluate(
        &self,
        question_index: usize,
        score: i16,
    ) -> Result<&Condition, RorschachError> {
        let answers = self.conditions.get(&question_index).ok_or(
            RorschachError::IndexNonExistingQuestion(
                question_index,
                self.stem.clone().unwrap(), //TODO
                self.n_answers as usize,
            ),
        )?;

        let condition =
            answers
                .get(&score)
                .ok_or(RorschachError::IndexNonExsistingQuestionScore(
                    score,
                    self.stem.clone().unwrap(), //TODO
                ))?;
        Ok(condition)
    }

    pub fn n_answers(&self) -> i16 {
        self.n_answers
    }
}
