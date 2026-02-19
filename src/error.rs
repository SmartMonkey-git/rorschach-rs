#[derive(Debug)]
pub enum RorschachError {
    TotalScoreTooHigh(String, usize, usize),
    AnswerQuestionMismatch(String, usize, usize),
}

impl std::fmt::Display for RorschachError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RorschachError::TotalScoreTooHigh(test_name, max_score, found) => {
                write!(
                    f,
                    "Total Score to high max score for {} is {} found {}.",
                    test_name, max_score, found
                )
            }
            RorschachError::AnswerQuestionMismatch(test_name, n_questions, n_answers) => {
                write!(
                    f,
                    "{} has {} question, got {} answers.",
                    test_name, n_questions, n_answers
                )
            }
        }
    }
}

impl std::error::Error for RorschachError {}
