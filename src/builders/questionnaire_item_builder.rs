use crate::condition::Condition;
use crate::questionnaire_item::QuestionnaireItem;
use std::collections::HashMap;

pub struct QuestionnaireItemBuilder {
    stem: Option<String>,
    conditions: HashMap<i16, Vec<Condition>>,
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

    pub fn condition(mut self, key: i16, condition: impl Into<Condition>) -> Self {
        self.conditions
            .entry(key)
            .or_default()
            .push(condition.into());

        self
    }

    pub fn conditions(mut self, key: i16, conditions: Vec<Condition>) -> Self {
        self.conditions.entry(key).or_insert(conditions);
        self
    }

    pub fn build(self) -> QuestionnaireItem {
        QuestionnaireItem::new(self.stem, self.conditions, self.n_answers)
    }
}

impl QuestionnaireItem {
    pub fn builder(n_answers: i16) -> QuestionnaireItemBuilder {
        QuestionnaireItemBuilder::new(n_answers)
    }
}
