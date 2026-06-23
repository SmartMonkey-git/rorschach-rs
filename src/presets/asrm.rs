use crate::condition::Condition;
use crate::questionnaire::Questionnaire;
use crate::questionnaire_item::QuestionnaireItem;
use crate::score_calculations::sum_score::SumScore;
use crate::term::{PhenotypeTerms, SeverityTerms};
use chrono::Duration;
use maplit::btreemap;

pub(crate) fn asrm() -> Questionnaire {
    Questionnaire::new(
        "ASRM",
        vec![
            QuestionnaireItem::new(Some("".to_string()), PhenotypeTerms::Euphoria, 5.0),
            QuestionnaireItem::new(Some("".to_string()), PhenotypeTerms::Grandiosity, 5.0),
            QuestionnaireItem::new(
                Some("".to_string()),
                PhenotypeTerms::DecreasedNeedForSleep,
                5.0,
            ),
            QuestionnaireItem::new(Some("".to_string()), PhenotypeTerms::PressuredSpeech, 5.0),
            QuestionnaireItem::new(Some("".to_string()), PhenotypeTerms::Agitation, 5.0),
        ],
        btreemap! {
            0  => Some(Condition::new_excluded(PhenotypeTerms::Mania)),
            5  => Some(Condition::new_excluded(PhenotypeTerms::Mania)),
            8  => Some(Condition::without_time(PhenotypeTerms::Mania, SeverityTerms::Borderline)),
            11 => Some(Condition::without_time(PhenotypeTerms::Mania, SeverityTerms::Mild)),
            14 => Some(Condition::without_time(PhenotypeTerms::Mania, SeverityTerms::Moderate)),
            17 => Some(Condition::without_time(PhenotypeTerms::Mania, SeverityTerms::Severe)),
            20 => Some(Condition::without_time(PhenotypeTerms::Mania, SeverityTerms::Profound)),
        },
        SumScore,
        Some(Duration::weeks(2)),
    )
}
