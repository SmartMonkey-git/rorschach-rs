use crate::presets::asrm::asrm;
use crate::presets::gad7::gad7;
use crate::presets::phq9::phq9;
use crate::questionnaire::Questionnaire;

pub enum QuestionnairePresets {
    PHQ9,
    GAD7,
    ASRM,
}

impl QuestionnairePresets {
    pub fn build(&self) -> Questionnaire {
        match self {
            QuestionnairePresets::PHQ9 => {
                phq9().expect("Should always work, because its hard coded")
            }
            QuestionnairePresets::GAD7 => {
                gad7().expect("Should always work, because its hard coded")
            }
            QuestionnairePresets::ASRM => {
                asrm().expect("Should always work, because its hard coded")
            }
        }
    }
}
