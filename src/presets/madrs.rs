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

pub(crate) fn madrs() -> Result<Questionnaire, RorschachError> {
    let name = "MADRS";

    let severities = vec![
        (1, SeverityTerms::Borderline),
        (2, SeverityTerms::Mild),
        (3, SeverityTerms::Moderate),
        (4, SeverityTerms::Moderate),
        (5, SeverityTerms::Severe),
        (6, SeverityTerms::Profound),
    ];
    let builder = QuestionnaireBuilder::new(name, Box::new(SumScore))
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
            QuestionnaireItemBuilder::new(7)
                .conditions(
                    ConditionBuilder::new(PhenotypeTerms::PathologicalSadness)
                        .build_with_severities(severities.as_slice(), true),
                )
                .stem("Apparent Sadness"),
            QuestionnaireItemBuilder::new(7)
                .conditions(
                    ConditionBuilder::new(PhenotypeTerms::PathologicalSadness)
                        .build_with_severities(severities.as_slice(), true),
                )
                .stem("Reported sadness"),
            QuestionnaireItemBuilder::new(7)
                .conditions(
                    ConditionBuilder::new(PhenotypeTerms::Anxiety)
                        .build_with_severities(severities.as_slice(), true),
                )
                .stem("Inner tension"),
            QuestionnaireItemBuilder::new(7)
                .conditions(
                    ConditionBuilder::new(PhenotypeTerms::DecreasedNeedForSleep)
                        .build_with_severities(severities.as_slice(), true),
                )
                .stem("Reduced sleep"),
            QuestionnaireItemBuilder::new(7)
                .conditions(
                    ConditionBuilder::new(PhenotypeTerms::PoorAppetite)
                        .build_with_severities(severities.as_slice(), true),
                )
                .stem("Reduced appetite"),
            QuestionnaireItemBuilder::new(7)
                .conditions(
                    ConditionBuilder::new(PhenotypeTerms::DiminishedAbilityToConcentrate)
                        .build_with_severities(severities.as_slice(), true),
                )
                .stem("Concentration Difficulties"),
            QuestionnaireItemBuilder::new(7)
                .conditions(
                    ConditionBuilder::new(PhenotypeTerms::Fatigue)
                        .build_with_severities(severities.as_slice(), true),
                )
                .stem("Lassitude"),
            QuestionnaireItemBuilder::new(7)
                .conditions(
                    ConditionBuilder::new(PhenotypeTerms::Apathy)
                        .build_with_severities(severities.as_slice(), true),
                )
                .stem("Inability to feel"),
            QuestionnaireItemBuilder::new(7)
                .conditions(
                    ConditionBuilder::new(PhenotypeTerms::Hopelessness)
                        .build_with_severities(severities.as_slice(), true),
                )
                .stem("Pessimistic thoughts"),
            QuestionnaireItemBuilder::new(7)
                .conditions(
                    ConditionBuilder::new(PhenotypeTerms::SuicidalIdeation)
                        .build_with_severities(severities.as_slice(), true),
                )
                .stem("Suicidal thoughts"),
        ]);

    let builder = builder.map_err(|err| {
        RorschachError::BuildingError(format!("Error when building {name}: {}", err))
    })?;
    Ok(builder.build())
}
