use crate::answer::Answer;
use crate::error::RorschachError;
use crate::questionnaire_result::QuestionnaireResult;
use chrono::{DateTime, Utc};
use std::fmt::Debug;

pub trait EvaluateQuestionnaire {
    fn evaluate(
        &self,
        questionnaire_id: &str,
        answers: &[Answer],
        taken_at: Option<DateTime<Utc>>,
    ) -> Result<QuestionnaireResult, RorschachError>;
}

pub trait CalculateScore: Debug {
    fn calculate_score(&self, scores: &[f32]) -> f32;
}
