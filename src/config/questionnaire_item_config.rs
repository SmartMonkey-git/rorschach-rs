use crate::condition;
use crate::condition::Condition;
use crate::config::condition_config::ConditionConfig;
use crate::config::error::ConfigError;
use crate::error::RorschachError;
use crate::questionnaire_item::QuestionnaireItem;
use serde_derive::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct QuestionnaireItemConfig {
    stem: Option<String>,
    /// Score to condition
    conditions: HashMap<i16, Option<ConditionConfig>>,
    n_answers: i16,
}

impl TryFrom<&QuestionnaireItemConfig> for QuestionnaireItem {
    type Error = ConfigError;

    fn try_from(config: &QuestionnaireItemConfig) -> Result<Self, ConfigError> {
        let conditions = config
            .conditions
            .iter()
            .map(|(s, condition)| {
                let condition: Option<Condition> = condition.as_ref().map(|c| c.into());
                (s.clone(), condition)
            })
            .collect();

        Ok(QuestionnaireItem::new(
            config.stem.clone(),
            conditions,
            config.n_answers,
        )?)
    }
}
