use crate::traits::CalculateScore;

#[derive(Debug)]
pub struct ProratedScore {
    n_questionnaire_items: usize,
}

impl ProratedScore {
    pub fn new(n_questionnaire_items: usize) -> Self {
        ProratedScore {
            n_questionnaire_items,
        }
    }
}

impl CalculateScore for ProratedScore {
    fn calculate_score(&self, scores: &[Option<f32>]) -> f32 {
        if scores.is_empty() {
            return 0.0;
        }
        let partial_raw_score: f32 = scores.iter().flatten().sum();
        (partial_raw_score * self.n_questionnaire_items as f32) / scores.len() as f32
    }
}
