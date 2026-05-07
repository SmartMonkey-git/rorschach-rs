use crate::condition::Condition;

#[derive(Debug, PartialEq, Clone)]
pub struct QuestionnaireItem {
    stem: Option<String>,
    phenotype: Condition,
    n_answers: f32,
}

impl QuestionnaireItem {
    pub fn new(
        stem: Option<String>,
        phenotype: impl Into<Condition>,
        n_answers: f32,
    ) -> QuestionnaireItem {
        Self {
            stem,
            phenotype: phenotype.into(),
            n_answers,
        }
    }

    pub fn phenotype(&self) -> &Condition {
        &self.phenotype
    }

    pub fn n_answers(&self) -> f32 {
        self.n_answers
    }
}
