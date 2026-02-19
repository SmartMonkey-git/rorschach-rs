use crate::phenotype::Phenotype;
use std::collections::HashSet;

pub struct QuestionnaireResult {
    diagnose: Phenotype,
    phenotypes: HashSet<Phenotype>,
}

impl QuestionnaireResult {
    pub fn new(diagnose: Phenotype) -> Self {
        Self {
            diagnose,
            phenotypes: HashSet::new(),
        }
    }

    pub fn push_phenotypes(&mut self, terms: Vec<Phenotype>) {
        self.phenotypes.extend(terms);
    }
}
