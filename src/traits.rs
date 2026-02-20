use std::fmt::Debug;

pub trait CalculateScore: Debug {
    fn calculate_score(&self, scores: &[f32]) -> f32;
}
