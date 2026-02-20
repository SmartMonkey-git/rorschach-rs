use std::fmt::Debug;

pub trait CalculateScore: Debug + PartialEq {
    fn calculate_score(&self, scores: &[f32]) -> f32;
}
