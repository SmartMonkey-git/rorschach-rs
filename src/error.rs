use crate::term::Term;
use std::fmt::Debug;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RorschachError {
    #[error(
        "Total score too high for '{test_name}': max score is {max_score}, found {found_score}."
    )]
    TotalScoreTooHigh {
        test_name: String,
        max_score: f32,
        found_score: f32,
    },
    #[error("'{test_name}' has {expected} questions, but got {found} answers.")]
    AnswerQuestionMismatch {
        test_name: String,
        expected: usize,
        found: usize,
    },
    #[error("No diagnostic interpretation found for a score of {found_score}.")]
    NoMatchingDiagnosis { found_score: f32 },
}

#[derive(Debug, Error)]
pub enum ToCsvError {
    #[error("Can't parse {value:?}")]
    CantParseSeverity { value: Option<Term> },
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}
