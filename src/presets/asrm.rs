use crate::questionnaire::Questionnaire;
use crate::questionnaire_item::QuestionnaireItem;
use crate::score_calculations::sum_score::SumScore;
use crate::term::PhenotypeTerms;
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
                0 => None,
                20 => None,
        },
        SumScore,
        Some(Duration::weeks(2)),
    )
}
