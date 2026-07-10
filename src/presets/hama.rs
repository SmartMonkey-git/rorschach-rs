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

pub(crate) fn hama() -> Result<Questionnaire, RorschachError> {
    let name = "HAM-A";
    let severities = vec![
        (1, SeverityTerms::Borderline),
        (2, SeverityTerms::Moderate),
        (3, SeverityTerms::Moderate),
        (4, SeverityTerms::Profound),
    ];
    let builder = QuestionnaireBuilder::new(name, Box::new(SumScore))
        .recall_period(Duration::weeks(0))
        .interpretations(btreemap! {
            0  => Some(Condition::new_excluded(PhenotypeTerms::Mania)),
            5  => Some(Condition::new_excluded(PhenotypeTerms::Mania)),
            17  => Some(Condition::without_time(PhenotypeTerms::Mania, SeverityTerms::Mild)),
            24 => Some(Condition::without_time(PhenotypeTerms::Mania, SeverityTerms::Moderate)),
            30 => Some(Condition::without_time(PhenotypeTerms::Mania, SeverityTerms::Severe)),
            56 => Some(Condition::without_time(PhenotypeTerms::Mania, SeverityTerms::Profound)),
        })
        .items([
            QuestionnaireItemBuilder::new(5)
                .conditions(
                    ConditionBuilder::new(PhenotypeTerms::Anxiety)
                        .build_with_severities(severities.as_slice(), true),
                )
                .stem("Anxious mood"),
            QuestionnaireItemBuilder::new(5)
                .conditions(
                    ConditionBuilder::new(PhenotypeTerms::Agitation) // TODO: Does not fit well
                        .build_with_severities(severities.as_slice(), true),
                )
                .stem("Tension"),
            QuestionnaireItemBuilder::new(5)
                .conditions(
                    ConditionBuilder::new(PhenotypeTerms::Phobia)
                        .build_with_severities(severities.as_slice(), true),
                )
                .stem("Fears"),
            QuestionnaireItemBuilder::new(5)
                .conditions(
                    ConditionBuilder::new(PhenotypeTerms::Insomnia)
                        .build_with_severities(severities.as_slice(), true),
                )
                .stem("Insomnia"),
            QuestionnaireItemBuilder::new(5)
                .conditions(
                    ConditionBuilder::new(PhenotypeTerms::CognitiveImpairment)
                        .build_with_severities(severities.as_slice(), true),
                )
                .stem("Intellectual"),
            QuestionnaireItemBuilder::new(5)
                .conditions(
                    ConditionBuilder::new(PhenotypeTerms::Depression)
                        .build_with_severities(severities.as_slice(), true),
                )
                .stem("Depressed mood"),
            QuestionnaireItemBuilder::new(5).stem("Somatic (muscular)"),
            QuestionnaireItemBuilder::new(5).stem("Somatic (sensory)"),
            QuestionnaireItemBuilder::new(5)
                .conditions(
                    ConditionBuilder::new(PhenotypeTerms::AbnormalityOfTheCardiovascularSystem)
                        .build_with_severities(severities.as_slice(), true),
                )
                .stem("Cardiovascular symptoms"),
            QuestionnaireItemBuilder::new(5)
                .conditions(
                    ConditionBuilder::new(PhenotypeTerms::AbnormalityOfTheRespiratorySystem)
                        .build_with_severities(severities.as_slice(), true),
                )
                .stem("Respiratory symptoms"),
            QuestionnaireItemBuilder::new(5)
                .conditions(
                    ConditionBuilder::new(PhenotypeTerms::AbnormalityOfTheGastrointestinalTract)
                        .build_with_severities(severities.as_slice(), true),
                )
                .stem("Gastrointestinal symptoms"),
            QuestionnaireItemBuilder::new(5)
                .conditions(
                    ConditionBuilder::new(PhenotypeTerms::AbnormalityOfTheGenitourinarySystem)
                        .build_with_severities(severities.as_slice(), true),
                )
                .stem("Genitourinary symptoms"),
            QuestionnaireItemBuilder::new(5)
                .conditions(
                    ConditionBuilder::new(PhenotypeTerms::AbnormalAutonomicNervousSystemPhysiology)
                        .build_with_severities(severities.as_slice(), true),
                )
                .stem("Autonomic symptoms"),
            QuestionnaireItemBuilder::new(5)
                .conditions(
                    ConditionBuilder::new(PhenotypeTerms::AtypicalBehavior)
                        .build_with_severities(severities.as_slice(), true),
                )
                .stem("Behavior at interview"),
        ]);

    let builder = builder.map_err(|err| {
        RorschachError::BuildingError(format!("Error when building {name}: {}", err))
    })?;
    Ok(builder.build())
}
