use crate::presets::phq9::phq9;
use crate::questionnaire::Questionnaire;

pub enum QuestionnairePresets {
    PHQ9,
}

impl QuestionnairePresets {
    pub fn build(&self) -> Questionnaire {
        match self {
            QuestionnairePresets::PHQ9 => phq9(),
        }
    }
}
