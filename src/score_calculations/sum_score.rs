use crate::traits::CalculateScore;

#[derive(Clone, Debug, PartialEq)]
pub struct SumScore;

impl CalculateScore for SumScore {
    fn calculate_score(&self, scores: &[f32]) -> f32 {
        scores.iter().sum::<f32>()
    }
}
