use crate::condition::Condition;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct QuestionnaireItem {
    stem: Option<String>,
    conditions: HashMap<i16, Vec<Condition>>,
    n_answers: i16,
}

impl QuestionnaireItem {
    pub fn new(
        stem: Option<String>,
        conditions: HashMap<i16, Vec<Condition>>,
        n_answers: i16,
    ) -> QuestionnaireItem {
        Self {
            stem,
            conditions,
            n_answers,
        }
    }

    pub fn conditions(&self, idx: i16) -> &Vec<Condition> {
        &self.conditions[&idx]
    }
    pub fn evaluate(&self, _score: i16) -> &[Condition] {
        //TODO
        &[]
    }

    pub fn n_answers(&self) -> i16 {
        self.n_answers
    }
}
