use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub enum ScoreCalculatorConfig {
    SumScore,
    ProratedScore { n_questionnaire_items: usize },
}
