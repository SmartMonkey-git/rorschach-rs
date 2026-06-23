use crate::traits::CalculateScore;

#[derive(Clone, Debug, PartialEq)]
pub struct SumScore;

impl CalculateScore for SumScore {
    fn calculate_score(&self, scores: &[Option<f32>]) -> f32 {
        scores.iter().flatten().sum::<f32>()
    }
}
