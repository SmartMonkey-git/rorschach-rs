use crate::condition::Condition;
use crate::questionnaire::Questionnaire;
use crate::questionnaire_item::QuestionnaireItem;
use crate::score_calculations::sum_score::SumScore;
use crate::term::{DiagnosisTerms, PhenotypeTerms, SeverityTerms};
use chrono::Duration;
use maplit::{btreemap, hashmap};

pub(crate) fn phq9() -> Questionnaire {
    Questionnaire::new("PATIENT HEALTH (PHQ-9) QUESTIONNAIRE-9", vec![
        QuestionnaireItem::new(
            Some("Little interest or pleasure in doing things".to_string()),
            hashmap! {
                            0 => vec![],
                            1 => vec![Condition::without_time(PhenotypeTerms::Anhedonia, SeverityTerms::Mild)],
                            2 => vec![Condition::without_time(PhenotypeTerms::Anhedonia, SeverityTerms::Moderate)],
                            3 => vec![Condition::without_time(PhenotypeTerms::Anhedonia, SeverityTerms::Severe)],
                        },
        ),
        QuestionnaireItem::new(
            Some("Feeling down, depressed, or hopeless".to_string()),
            hashmap! {
                            0 => vec![],
                            1 => vec![Condition::without_time(PhenotypeTerms::Depression, SeverityTerms::Mild)],
                            2 => vec![Condition::without_time(PhenotypeTerms::Depression, SeverityTerms::Moderate)],
                            3 => vec![Condition::without_time(PhenotypeTerms::Depression, SeverityTerms::Severe)],
                        },
        ),
        QuestionnaireItem::new(
            Some("Trouble falling or staying asleep, or sleeping too much".to_string()),
            hashmap! {
                            0 => vec![],
                            1 => vec![Condition::without_time(PhenotypeTerms::SleepDisturbance, SeverityTerms::Mild)],
                            2 => vec![Condition::without_time(PhenotypeTerms::SleepDisturbance, SeverityTerms::Moderate)],
                            3 => vec![Condition::without_time(PhenotypeTerms::SleepDisturbance, SeverityTerms::Severe)],
                        },
        ),
        QuestionnaireItem::new(
            Some("Feeling tired or having little energy".to_string()),
            hashmap! {
                            0 => vec![],
                            1 => vec![Condition::without_time(PhenotypeTerms::Fatigue, SeverityTerms::Mild)],
                            2 => vec![Condition::without_time(PhenotypeTerms::Fatigue, SeverityTerms::Moderate)],
                            3 => vec![Condition::without_time(PhenotypeTerms::Fatigue, SeverityTerms::Severe)],
                        },
        ),
        QuestionnaireItem::new(
            Some("Poor appetite or overeating".to_string()),
            hashmap! {
                            0 => vec![],
                            1 => vec![Condition::without_time(PhenotypeTerms::AbnormalEatingBehavior, SeverityTerms::Mild)],
                            2 => vec![Condition::without_time(PhenotypeTerms::AbnormalEatingBehavior, SeverityTerms::Moderate)],
                            3 => vec![Condition::without_time(PhenotypeTerms::AbnormalEatingBehavior, SeverityTerms::Severe)],
                        },
        ),
        QuestionnaireItem::new(
            Some("Feeling bad about yourself — or that you are a failure or have let yourself or your family down".to_string()),
            hashmap! {
                            0 => vec![],
                            1 => vec![Condition::without_time(PhenotypeTerms::Guilt, SeverityTerms::Mild), Condition::without_time(PhenotypeTerms::LowSelfEsteem, SeverityTerms::Mild)],
                            2 => vec![Condition::without_time(PhenotypeTerms::Guilt, SeverityTerms::Moderate), Condition::without_time(PhenotypeTerms::LowSelfEsteem, SeverityTerms::Moderate)],
                            3 => vec![Condition::without_time(PhenotypeTerms::Guilt, SeverityTerms::Severe), Condition::without_time(PhenotypeTerms::LowSelfEsteem, SeverityTerms::Severe)],
                        },
        ),
        QuestionnaireItem::new(
            Some("Trouble concentrating on things, such as reading the newspaper or watching television".to_string()),
            hashmap! {
                            0 => vec![],
                            1 => vec![Condition::without_time(PhenotypeTerms::DiminishedAbilityToConcentrate, SeverityTerms::Mild)],
                            2 => vec![Condition::without_time(PhenotypeTerms::DiminishedAbilityToConcentrate, SeverityTerms::Moderate)],
                            3 => vec![Condition::without_time(PhenotypeTerms::DiminishedAbilityToConcentrate, SeverityTerms::Severe)],
                        },
        ),
        QuestionnaireItem::new(
            Some("Moving or speaking so slowly that other people could have noticed? Or the opposite — being so fidgety or restless that you have been moving around a lot more than usual".to_string()),
            hashmap! {
                            0 => vec![],
                            1 => vec![Condition::without_time(PhenotypeTerms::AbnormalVolitionalState, SeverityTerms::Mild)],
                            2 => vec![Condition::without_time(PhenotypeTerms::AbnormalVolitionalState, SeverityTerms::Moderate)],
                            3 => vec![Condition::without_time(PhenotypeTerms::AbnormalVolitionalState, SeverityTerms::Severe)],
                        },
        ),
        QuestionnaireItem::new(
            Some("Thoughts that you would be better off dead or of hurting yourself in some way".to_string()),
            hashmap! {
                            0 => vec![],
                            1 => vec![Condition::without_time(PhenotypeTerms::SuicidalIdeation, SeverityTerms::Mild), Condition::without_time(PhenotypeTerms::LowSelfEsteem, SeverityTerms::Mild)],
                            2 => vec![Condition::without_time(PhenotypeTerms::SuicidalIdeation, SeverityTerms::Moderate), Condition::without_time(PhenotypeTerms::LowSelfEsteem, SeverityTerms::Moderate)],
                            3 => vec![Condition::without_time(PhenotypeTerms::SuicidalIdeation, SeverityTerms::Severe), Condition::without_time(PhenotypeTerms::LowSelfEsteem, SeverityTerms::Severe)],
                        },
        ),
    ], btreemap! {
                    0 => None,
                    2 => Some(Condition::without_time(DiagnosisTerms::DepressiveDisorder, SeverityTerms::Borderline)),
                    5 => Some(Condition::without_time(DiagnosisTerms::DepressiveDisorder, SeverityTerms::Mild)),
                    10 => Some(Condition::without_time(DiagnosisTerms::DepressiveDisorder, SeverityTerms::Moderate)),
                    15 => Some(Condition::without_time(DiagnosisTerms::DepressiveDisorder, SeverityTerms::Severe)),
                    20 => Some(Condition::without_time(DiagnosisTerms::DepressiveDisorder, SeverityTerms::Severe)),
            }, SumScore,
                                                     Some(Duration::weeks(2))
    )
}
