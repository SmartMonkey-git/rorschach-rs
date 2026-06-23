use crate::error::ToCsvError;
use std::fmt::Debug;
use std::io::Write;

pub trait CalculateScore: Debug {
    fn calculate_score(&self, scores: &[Option<f32>]) -> f32;
}

pub trait ToCsv {
    fn to_csv<W: Write>(&self, writer: &mut W, filter_duplicates: bool) -> Result<(), ToCsvError>;
}
