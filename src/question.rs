use crate::phenotype::Phenotype;
use std::collections::HashMap;

pub struct Question {
    min: usize,
    max: usize,
    phenotypes: HashMap<usize, Vec<Phenotype>>,
}

impl Question {
    pub fn new(min: usize, max: usize, phenotypes: HashMap<usize, Vec<Phenotype>>) -> Question {
        Self {
            min,
            max,
            phenotypes,
        }
    }

    pub fn answer(&self, answer: &usize) -> Vec<Phenotype> {
        match self.phenotypes.get(answer) {
            None => {
                vec![]
            }
            Some(phenotypes) => phenotypes.clone(),
        }
    }
}
