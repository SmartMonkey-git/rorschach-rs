use crate::condition::Condition;
use crate::questionnaire::Questionnaire;
use crate::questionnaire_item::QuestionnaireItem;
use crate::score_calculations::sum_score::SumScore;
use crate::term::{DiagnosisTerms, PhenotypeTerms, SeverityTerms};
use chrono::Duration;
use maplit::btreemap;

pub(crate) fn phq9() -> Questionnaire {
    Questionnaire::new("PHQ-9", vec![
        QuestionnaireItem::new(
            Some("Little interest or pleasure in doing things".to_string()),
            PhenotypeTerms::Anhedonia,4.0
        ),
        QuestionnaireItem::new(
            Some("Feeling down, depressed, or hopeless".to_string()),
            PhenotypeTerms::PathologicalSadness, 4.0
        ),
        QuestionnaireItem::new(
            Some("Trouble falling or staying asleep, or sleeping too much".to_string()),
            PhenotypeTerms::SleepDisturbance, 4.0
        ),
        QuestionnaireItem::new(
            Some("Feeling tired or having little energy".to_string()),
            PhenotypeTerms::Fatigue, 4.0
        ),
        QuestionnaireItem::new(
            Some("Poor appetite or overeating".to_string()),
            PhenotypeTerms::AbnormalEatingBehavior, 4.0
        ),
        QuestionnaireItem::new(
            Some("Feeling bad about yourself — or that you are a failure or have let yourself or your family down".to_string()),
            PhenotypeTerms::Guilt, 4.0
        ),
        QuestionnaireItem::new(
            Some("Trouble concentrating on things, such as reading the newspaper or watching television".to_string()),
            PhenotypeTerms::DiminishedAbilityToConcentrate, 4.0
        ),
        QuestionnaireItem::new(
            Some("Moving or speaking so slowly that other people could have noticed? Or the opposite — being so fidgety or restless that you have been moving around a lot more than usual".to_string()),
            PhenotypeTerms::PsychomotorDeterioration, 4.0
        ),
        QuestionnaireItem::new(
            Some("Thoughts that you would be better off dead or of hurting yourself in some way".to_string()),
            PhenotypeTerms::SuicidalIdeation, 4.0
        ),
    ], btreemap! {
                    0 => None,
                    2 => Some(Condition::without_time(DiagnosisTerms::DepressiveDisorder, SeverityTerms::Borderline)),
                    5 => Some(Condition::without_time(DiagnosisTerms::DepressiveDisorder, SeverityTerms::Mild)),
                    10 => Some(Condition::without_time(DiagnosisTerms::DepressiveDisorder, SeverityTerms::Moderate)),
                    15 => Some(Condition::without_time(DiagnosisTerms::DepressiveDisorder, SeverityTerms::Severe)),
                    20 => Some(Condition::without_time(DiagnosisTerms::DepressiveDisorder, SeverityTerms::Profound)),
            }, SumScore,
                                                     Some(Duration::weeks(2))
    )
}
