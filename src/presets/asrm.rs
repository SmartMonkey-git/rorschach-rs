use crate::builders::condition_builder::ConditionBuilder;
use crate::builders::questionnaire_builder::QuestionnaireBuilder;
use crate::builders::questionnaire_item_builder::QuestionnaireItemBuilder;
use crate::condition::Condition;
use crate::error::RorschachError;
use crate::questionnaire::Questionnaire;
use crate::score_calculations::sum_score::SumScore;
use crate::term::{PhenotypeTerms, SeverityTerms};
use chrono::Duration;
use maplit::btreemap;

pub(crate) fn asrm() -> Result<Questionnaire, RorschachError> {
    let severities = vec![
        (1, SeverityTerms::Borderline),
        (2, SeverityTerms::Moderate),
        (3, SeverityTerms::Moderate),
        (4, SeverityTerms::Profound),
    ];
    let builder = QuestionnaireBuilder::new("ASRM", Box::new(SumScore))
        .recall_period(Duration::weeks(2))
        .interpretations(btreemap! {
            0  => Some(Condition::new_excluded(PhenotypeTerms::Mania)),
            5  => Some(Condition::new_excluded(PhenotypeTerms::Mania)),
            8  => Some(Condition::without_time(PhenotypeTerms::Mania, SeverityTerms::Borderline)),
            11 => Some(Condition::without_time(PhenotypeTerms::Mania, SeverityTerms::Mild)),
            14 => Some(Condition::without_time(PhenotypeTerms::Mania, SeverityTerms::Moderate)),
            17 => Some(Condition::without_time(PhenotypeTerms::Mania, SeverityTerms::Severe)),
            20 => Some(Condition::without_time(PhenotypeTerms::Mania, SeverityTerms::Profound)),
        })
        .items([
            QuestionnaireItemBuilder::new(5).conditions(
                ConditionBuilder::new(PhenotypeTerms::Euphoria)
                    .build_with_severities(severities.as_slice(), true),
            ),
            QuestionnaireItemBuilder::new(5).conditions(
                ConditionBuilder::new(PhenotypeTerms::Grandiosity)
                    .build_with_severities(severities.as_slice(), true),
            ),
            QuestionnaireItemBuilder::new(5).conditions(
                ConditionBuilder::new(PhenotypeTerms::DecreasedNeedForSleep)
                    .build_with_severities(severities.as_slice(), true),
            ),
            QuestionnaireItemBuilder::new(5).conditions(
                ConditionBuilder::new(PhenotypeTerms::PressuredSpeech)
                    .build_with_severities(severities.as_slice(), true),
            ),
            QuestionnaireItemBuilder::new(5).conditions(
                ConditionBuilder::new(PhenotypeTerms::Agitation)
                    .build_with_severities(severities.as_slice(), true),
            ),
        ]);

    let builder = builder.map_err(|err| {
        RorschachError::BuildingError(format!("Error when building ASRM: {}", err.to_string()))
    })?;
    Ok(builder.build())
}
