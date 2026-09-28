use crate::condition::Condition;
use crate::error::RorschachError;
use crate::questionnaire_item::QuestionnaireItem;
use std::collections::HashMap;

pub struct QuestionnaireItemBuilder {
    stem: Option<String>,
    //  Score and Condition
    conditions: HashMap<i16, Option<Condition>>,
    n_answers: i16,
    max_score: i16,
}

impl QuestionnaireItemBuilder {
    pub fn new(n_answers: i16, max_score: i16) -> Self {
        Self {
            stem: None,
            conditions: HashMap::new(),
            n_answers,
            max_score,
        }
    }

    pub fn stem(mut self, stem: impl Into<String>) -> Self {
        self.stem = Some(stem.into());
        self
    }

    pub fn condition(mut self, score: i16, condition: impl Into<Condition>) -> Self {
        let condition = condition.into();
        if score > self.max_score {
            panic!(
                "Score {score} is too high for {condition}; the maximum allowed score is {}",
                self.max_score
            );
        }
        self.conditions.insert(score, Some(condition));

        self
    }

    pub fn conditions(mut self, conditions: HashMap<i16, Condition>) -> Self {
        conditions.iter().for_each(|(score, condition)| {
            if score > &self.max_score {
                panic!(
                    "Score {score} is too high for {condition}; the maximum allowed score is {}",
                    self.max_score
                );
            }
        });

        let conditions: HashMap<i16, Option<Condition>> = conditions
            .into_iter()
            .map(|(score, condition)| (score, Some(condition)))
            .collect();

        self.conditions = conditions;
        self
    }

    pub fn empty_conditions(mut self) -> Self {
        self.conditions =
            HashMap::from_iter((0..self.n_answers).into_iter().map(|idx| (idx, None)));

        self
    }

    pub fn build(self) -> Result<QuestionnaireItem, RorschachError> {
        QuestionnaireItem::new(self.stem, self.conditions, self.n_answers)
    }
}

impl QuestionnaireItem {
    pub fn builder(n_answers: i16, max_score: i16) -> QuestionnaireItemBuilder {
        QuestionnaireItemBuilder::new(n_answers, max_score)
    }
}
