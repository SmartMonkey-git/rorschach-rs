use crate::condition::Condition;
use crate::error::RorschachError;
use crate::questionnaire_item::QuestionnaireItem;
use std::collections::HashMap;

pub struct QuestionnaireItemBuilder {
    stem: Option<String>,
    //  Score and Condition
    conditions: HashMap<i16, Condition>,
    n_answers: i16,
}

impl QuestionnaireItemBuilder {
    pub fn new(n_answers: i16) -> Self {
        Self {
            stem: None,
            conditions: HashMap::new(),
            n_answers,
        }
    }

    pub fn stem(mut self, stem: impl Into<String>) -> Self {
        self.stem = Some(stem.into());
        self
    }

    pub fn condition(mut self, score: i16, condition: impl Into<Condition>) -> Self {
        self.conditions.insert(score, condition.into());

        self
    }

    pub fn conditions(mut self, conditions: HashMap<i16, Condition>) -> Self {
        self.conditions = conditions;
        self
    }

    pub fn build(self) -> Result<QuestionnaireItem, RorschachError> {
        QuestionnaireItem::new(self.stem, self.conditions, self.n_answers)
    }
}

impl QuestionnaireItem {
    pub fn builder(n_answers: i16) -> QuestionnaireItemBuilder {
        QuestionnaireItemBuilder::new(n_answers)
    }
}
