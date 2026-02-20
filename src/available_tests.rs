use crate::diagnosis::Diagnosis;
use crate::phenotype::Phenotype;
use crate::questionnaire::Questionnaire;
use crate::questionnaire_item::QuestionnaireItem;
use crate::score_calculations::sum_score::SumScore;
use crate::term::{DiagnosisTerms, PhenotypeTerms, SeverityTerms};
use crate::traits::CalculateScore;
use maplit::{btreemap, hashmap};

enum AvailableTest {
    PHQ9,
}

impl AvailableTest {
    pub fn build(&self) -> Questionnaire {
        match self {
            AvailableTest::PHQ9 => Questionnaire::new("PATIENT HEALTH (PHQ-9) QUESTIONNAIRE-9".to_string(), vec![
                QuestionnaireItem::new(
                    Some("Little interest or pleasure in doing things".to_string()),
                    hashmap! {
                            0 => vec![],
                            1 => vec![Phenotype::new(PhenotypeTerms::Anhedonia, SeverityTerms::Mild)],
                            2 => vec![Phenotype::new(PhenotypeTerms::Anhedonia, SeverityTerms::Moderate)],
                            3 => vec![Phenotype::new(PhenotypeTerms::Anhedonia, SeverityTerms::Severe)],
                        },
                ),
                QuestionnaireItem::new(
                    Some("Feeling down, depressed, or hopeless".to_string()),
                    hashmap! {
                            0 => vec![],
                            1 => vec![Phenotype::new(PhenotypeTerms::Depression, SeverityTerms::Mild)],
                            2 => vec![Phenotype::new(PhenotypeTerms::Depression, SeverityTerms::Moderate)],
                            3 => vec![Phenotype::new(PhenotypeTerms::Depression, SeverityTerms::Severe)],
                        },
                ),
                QuestionnaireItem::new(
                    Some("Trouble falling or staying asleep, or sleeping too much".to_string()),
                    hashmap! {
                            0 => vec![],
                            1 => vec![Phenotype::new(PhenotypeTerms::SleepDisturbance, SeverityTerms::Mild)],
                            2 => vec![Phenotype::new(PhenotypeTerms::SleepDisturbance, SeverityTerms::Moderate)],
                            3 => vec![Phenotype::new(PhenotypeTerms::SleepDisturbance, SeverityTerms::Severe)],
                        },
                ),
                QuestionnaireItem::new(
                    Some("Feeling tired or having little energy".to_string()),
                    hashmap! {
                            0 => vec![],
                            1 => vec![Phenotype::new(PhenotypeTerms::Fatigue, SeverityTerms::Mild)],
                            2 => vec![Phenotype::new(PhenotypeTerms::Fatigue, SeverityTerms::Moderate)],
                            3 => vec![Phenotype::new(PhenotypeTerms::Fatigue, SeverityTerms::Severe)],
                        },
                ),
                QuestionnaireItem::new(
                    Some("Poor appetite or overeating".to_string()),
                    hashmap! {
                            0 => vec![],
                            1 => vec![Phenotype::new(PhenotypeTerms::AbnormalEatingBehavior, SeverityTerms::Mild)],
                            2 => vec![Phenotype::new(PhenotypeTerms::AbnormalEatingBehavior, SeverityTerms::Moderate)],
                            3 => vec![Phenotype::new(PhenotypeTerms::AbnormalEatingBehavior, SeverityTerms::Severe)],
                        },
                ),
                QuestionnaireItem::new(
                    Some("Feeling bad about yourself — or that you are a failure or have let yourself or your family down".to_string()),
                    hashmap! {
                            0 => vec![],
                            1 => vec![Phenotype::new(PhenotypeTerms::Guilt, SeverityTerms::Mild), Phenotype::new(PhenotypeTerms::LowSelfEsteem, SeverityTerms::Mild)],
                            2 => vec![Phenotype::new(PhenotypeTerms::Guilt, SeverityTerms::Moderate), Phenotype::new(PhenotypeTerms::LowSelfEsteem, SeverityTerms::Moderate)],
                            3 => vec![Phenotype::new(PhenotypeTerms::Guilt, SeverityTerms::Severe), Phenotype::new(PhenotypeTerms::LowSelfEsteem, SeverityTerms::Severe)],
                        },
                ),
                QuestionnaireItem::new(
                    Some("Trouble concentrating on things, such as reading the newspaper or watching television".to_string()),
                    hashmap! {
                            0 => vec![],
                            1 => vec![],
                            2 => vec![],
                            3 => vec![],
                        },
                ),
                QuestionnaireItem::new(
                    Some("Moving or speaking so slowly that other people could have noticed? Or the opposite — being so fidgety or restless that you have been moving around a lot more than usual".to_string()),
                    hashmap! {
                            0 => vec![],
                            1 => vec![Phenotype::new(PhenotypeTerms::AbnormalVolitionalState, SeverityTerms::Mild)],
                            2 => vec![Phenotype::new(PhenotypeTerms::AbnormalVolitionalState, SeverityTerms::Moderate)],
                            3 => vec![Phenotype::new(PhenotypeTerms::AbnormalVolitionalState, SeverityTerms::Severe)],
                        },
                ),
                QuestionnaireItem::new(
                    Some("Thoughts that you would be better off dead or of hurting yourself in some way".to_string()),
                    hashmap! {
                            0 => vec![],
                            1 => vec![Phenotype::new(PhenotypeTerms::SuicidalIdeation, SeverityTerms::Mild), Phenotype::new(PhenotypeTerms::LowSelfEsteem, SeverityTerms::Mild)],
                            2 => vec![Phenotype::new(PhenotypeTerms::SuicidalIdeation, SeverityTerms::Moderate), Phenotype::new(PhenotypeTerms::LowSelfEsteem, SeverityTerms::Moderate)],
                            3 => vec![Phenotype::new(PhenotypeTerms::SuicidalIdeation, SeverityTerms::Severe), Phenotype::new(PhenotypeTerms::LowSelfEsteem, SeverityTerms::Severe)],
                        },
                ),
            ], btreemap! {
                    0 => None,
                    2 => Some(Diagnosis::new(DiagnosisTerms::DepressiveDisorder, SeverityTerms::Borderline)),
                    5 => Some(Diagnosis::new(DiagnosisTerms::DepressiveDisorder, SeverityTerms::Mild)),
                    10 => Some(Diagnosis::new(DiagnosisTerms::DepressiveDisorder, SeverityTerms::Moderate)),
                    15 => Some(Diagnosis::new(DiagnosisTerms::DepressiveDisorder, SeverityTerms::Severe)),
                    20 => Some(Diagnosis::new(DiagnosisTerms::DepressiveDisorder, SeverityTerms::Severe)),
            }, SumScore),
        }
    }
}
