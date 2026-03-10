use crate::error::ToCsvError;
use std::fmt::Debug;
use std::io::Write;

pub trait CalculateScore: Debug {
    fn calculate_score(&self, scores: &[f32]) -> f32;
}

pub trait ToCsv<T> {
    fn to_csv<W: Write>(&self, writer: &mut W) -> Result<(), ToCsvError>;
}
