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

pub(crate) fn ymrs() -> Result<Questionnaire, RorschachError> {
    let name = "YMRS";
    let linear_severities = vec![
        (1, SeverityTerms::Borderline),
        (2, SeverityTerms::Moderate),
        (3, SeverityTerms::Moderate),
        (4, SeverityTerms::Profound),
    ];

    let amplified_severities = vec![
        (2, SeverityTerms::Borderline),
        (4, SeverityTerms::Moderate),
        (6, SeverityTerms::Moderate),
        (8, SeverityTerms::Profound),
    ];

    let builder = QuestionnaireBuilder::new(name, Box::new(SumScore))
        .recall_period(Duration::weeks(2))
        .interpretations(btreemap! {
            0  => Some(Condition::new_excluded(PhenotypeTerms::Mania)),
            12 => Some(Condition::without_time(PhenotypeTerms::Mania, SeverityTerms::Borderline)),
            20 => Some(Condition::without_time(PhenotypeTerms::Mania, SeverityTerms::Mild)),
            26 => Some(Condition::without_time(PhenotypeTerms::Mania, SeverityTerms::Moderate)),
            38 => Some(Condition::without_time(PhenotypeTerms::Mania, SeverityTerms::Severe)),
            50 => Some(Condition::without_time(PhenotypeTerms::Mania, SeverityTerms::Profound)),
        })
        .items([
            QuestionnaireItemBuilder::new(5)
                .stem("Elevated Mood")
                .conditions(
                    ConditionBuilder::new(PhenotypeTerms::Euphoria)
                        .build_with_severities(linear_severities.as_slice(), true),
                ),
            QuestionnaireItemBuilder::new(5)
                .stem("Increased Motor Activity-Energy")
                .conditions(
                    ConditionBuilder::new(PhenotypeTerms::Agitation)
                        .build_with_severities(linear_severities.as_slice(), true),
                ),
            QuestionnaireItemBuilder::new(5)
                .stem("Sexual Interest")
                .conditions(
                    ConditionBuilder::new(PhenotypeTerms::AmplificationOfSexualBehavior)
                        .build_with_severities(linear_severities.as_slice(), true),
                ),
            QuestionnaireItemBuilder::new(5).stem("Sleep").conditions(
                ConditionBuilder::new(PhenotypeTerms::DecreasedNeedForSleep)
                    .build_with_severities(linear_severities.as_slice(), true),
            ),
            QuestionnaireItemBuilder::new(5)
                .stem("Irritability")
                .conditions(
                    ConditionBuilder::new(PhenotypeTerms::Irritability)
                        .build_with_severities(amplified_severities.as_slice(), true),
                ),
            QuestionnaireItemBuilder::new(5)
                .stem("Speech (Rate and Amount)")
                .conditions(
                    ConditionBuilder::new(PhenotypeTerms::PressuredSpeech)
                        .build_with_severities(amplified_severities.as_slice(), true),
                ),
            QuestionnaireItemBuilder::new(5)
                .stem("Language-Thought Disorder")
                .conditions(
                    ConditionBuilder::new(PhenotypeTerms::AbnormallyRapidThoughtProcess)
                        .build_with_severities(linear_severities.as_slice(), true),
                ),
            QuestionnaireItemBuilder::new(5).stem("Content").conditions(
                ConditionBuilder::new(PhenotypeTerms::DisorderOfThoughtContent)
                    .build_with_severities(amplified_severities.as_slice(), true),
            ),
            QuestionnaireItemBuilder::new(5)
                .stem("Disruptive-Aggressive Behavior")
                .conditions(
                    ConditionBuilder::new(PhenotypeTerms::AggressiveBehavior)
                        .build_with_severities(amplified_severities.as_slice(), true),
                ),
            QuestionnaireItemBuilder::new(5)
                .stem("Appearance")
                .conditions(
                    ConditionBuilder::new(PhenotypeTerms::SelfNeglect)
                        .build_with_severities(linear_severities.as_slice(), true),
                ),
            QuestionnaireItemBuilder::new(5).stem("Insight").conditions(
                ConditionBuilder::new(PhenotypeTerms::LackOfInsight)
                    .build_with_severities(linear_severities.as_slice(), true),
            ),
        ]);

    let builder = builder.map_err(|err| {
        RorschachError::BuildingError(format!("Error when building {name}: {}", err))
    })?;

    Ok(builder.build())
}
