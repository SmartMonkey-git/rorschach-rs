use crate::diagnosis::Diagnosis;
use crate::phenotype::Phenotype;
use std::collections::HashSet;

pub struct QuestionnaireResult {
    diagnosis: Option<Diagnosis>,
    phenotypes: HashSet<Phenotype>,
    taken_at: Option<String>, // TODO: String for now. Later date
}

impl QuestionnaireResult {
    pub fn new(
        diagnosis: Option<Diagnosis>,
        phenotypes: HashSet<Phenotype>,
        taken_at: Option<String>,
    ) -> Self {
        Self {
            diagnosis: diagnosis.clone(),
            phenotypes,
            taken_at,
        }
    }

    pub fn from_diagnosis(diagnosis: Option<Diagnosis>) -> Self {
        Self::new(diagnosis, Default::default(), None)
    }

    pub fn push_phenotypes(&mut self, terms: Vec<Phenotype>) {
        self.phenotypes.extend(terms);
    }
}
