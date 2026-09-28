#![allow(clippy::result_large_err)]
use crate::config::error::ConfigError;
use crate::config::questionnaire_config::QuestionnaireConfig;
use crate::questionnaire::Questionnaire;
use figment::Figment;
use figment::providers::{Format, Yaml};
use std::path::PathBuf;

pub struct QuestionnaireConfigLoader;

impl QuestionnaireConfigLoader {
    pub fn load(file_path: PathBuf) -> Result<Questionnaire, ConfigError> {
        let figment = Figment::new().merge(Yaml::file(file_path));
        let config: QuestionnaireConfig = figment.extract()?;

        config.try_into()
    }
}
