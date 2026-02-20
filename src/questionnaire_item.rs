use crate::phenotype::Phenotype;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub struct QuestionnaireItem {
    stem: Option<String>,
    phenotypes: HashMap<usize, Vec<Phenotype>>,
}

impl QuestionnaireItem {
    pub fn new(
        stem: Option<String>,
        phenotypes: HashMap<usize, Vec<Phenotype>>,
    ) -> QuestionnaireItem {
        Self { stem, phenotypes }
    }

    pub fn answer(&self, answer_idx: usize) -> &[Phenotype] {
        self.phenotypes
            .get(&answer_idx)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    pub fn max_score(&self) -> f32 {
        self.phenotypes.len() as f32
    }
}
