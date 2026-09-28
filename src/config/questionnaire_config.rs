#![allow(clippy::result_large_err)]
use crate::condition::Condition;
use crate::config::condition_config::ConditionConfig;
use crate::config::error::ConfigError;
use crate::config::questionnaire_item_config::QuestionnaireItemConfig;
use crate::config::score_calculation_config::ScoreCalculatorConfig;
use crate::questionnaire::Questionnaire;
use crate::questionnaire_item::QuestionnaireItem;
use crate::score_calculations::prorated_score::ProratedScore;
use crate::score_calculations::sum_score::SumScore;
use crate::traits::CalculateScore;
use chrono::Duration;
use serde_derive::Deserialize;
use std::collections::BTreeMap;

#[derive(Debug, Deserialize)]
pub struct QuestionnaireConfig {
    name: String,
    items: Vec<QuestionnaireItemConfig>,
    interpretation: BTreeMap<i32, Option<ConditionConfig>>,
    score_calculator: ScoreCalculatorConfig,
    recall_period: Option<Duration>,
}

impl TryFrom<QuestionnaireConfig> for Questionnaire {
    type Error = ConfigError;

    fn try_from(config: QuestionnaireConfig) -> Result<Self, ConfigError> {
        let score_calculator: Box<dyn CalculateScore> = match config.score_calculator {
            ScoreCalculatorConfig::SumScore => Box::new(SumScore) as Box<dyn CalculateScore>,
            ScoreCalculatorConfig::ProratedScore {
                n_questionnaire_items,
            } => Box::new(ProratedScore::new(n_questionnaire_items)) as Box<dyn CalculateScore>,
        };

        let items: Vec<QuestionnaireItem> = config
            .items
            .iter()
            .map(|item| item.try_into())
            .collect::<Result<Vec<_>, _>>()?;

        let interpretation: BTreeMap<_, Option<Condition>> = config
            .interpretation
            .iter()
            .map(|(idx, conf)| (*idx, conf.as_ref().map(Condition::from)))
            .collect();

        Ok(Questionnaire::new(
            config.name.as_str(),
            items,
            interpretation,
            score_calculator,
            config.recall_period,
        ))
    }
}
