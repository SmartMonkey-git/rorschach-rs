use crate::presets::gad7::gad7;
use crate::presets::phq9::phq9;
use crate::questionnaire::Questionnaire;

pub enum QuestionnairePresets {
    PHQ9,
    GAD7,
}

impl QuestionnairePresets {
    pub fn build(&self) -> Questionnaire {
        match self {
            QuestionnairePresets::PHQ9 => phq9(),
            QuestionnairePresets::GAD7 => gad7(),
        }
    }
}
