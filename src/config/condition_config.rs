use crate::condition::Condition;
use crate::config::term_config::TermConfig;
use chrono::{DateTime, Utc};
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ConditionConfig {
    term: TermConfig,
    severity: Option<TermConfig>,
    excluded: bool,
    observed_start: Option<DateTime<Utc>>,
    observed_end: Option<DateTime<Utc>>,
}

impl ConditionConfig {
    pub fn term(&self) -> &TermConfig {
        &self.term
    }
}

impl From<ConditionConfig> for Condition {
    fn from(config: ConditionConfig) -> Self {
        Condition::new(
            config.term(),
            config.severity.as_ref().map(|s| s.into()),
            config.excluded,
            config.observed_start,
            config.observed_end,
        )
    }
}

impl From<&ConditionConfig> for Condition {
    fn from(config: &ConditionConfig) -> Self {
        Condition::new(
            config.term(),
            config.severity.as_ref().map(|s| s.into()),
            config.excluded,
            config.observed_start,
            config.observed_end,
        )
    }
}
