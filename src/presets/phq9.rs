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

pub(crate) fn phq9() -> Result<Questionnaire, RorschachError> {
    let name = "PHQ-9";
    let severities = vec![
        (1, SeverityTerms::Borderline),
        (2, SeverityTerms::Moderate),
        (3, SeverityTerms::Profound),
    ];

    let builder = QuestionnaireBuilder::new(name, Box::new(SumScore))
        .recall_period(Duration::weeks(2))
        .interpretations(btreemap! {
            0  => Some(Condition::new_excluded(PhenotypeTerms::Depression)),
            2  => Some(Condition::without_time(PhenotypeTerms::Depression, SeverityTerms::Borderline)),
            5  => Some(Condition::without_time(PhenotypeTerms::Depression, SeverityTerms::Mild)),
            10 => Some(Condition::without_time(PhenotypeTerms::Depression, SeverityTerms::Moderate)),
            15 => Some(Condition::without_time(PhenotypeTerms::Depression, SeverityTerms::Severe)),
            20 => Some(Condition::without_time(PhenotypeTerms::Depression, SeverityTerms::Profound)),
        })
        .items([
            QuestionnaireItemBuilder::new(4)
                .stem("Little interest or pleasure in doing things")
                .conditions(

                    ConditionBuilder::new(PhenotypeTerms::Anhedonia)
                        .build_with_severities(severities.as_slice(), true),
                ),
            QuestionnaireItemBuilder::new(4)
                .stem("Feeling down, depressed, or hopeless")
                .conditions(
                    ConditionBuilder::new(PhenotypeTerms::PathologicalSadness)
                        .build_with_severities(severities.as_slice(), true),
                ),
            QuestionnaireItemBuilder::new(4)
                .stem("Trouble falling or staying asleep, or sleeping too much")
                .conditions(
                    ConditionBuilder::new(PhenotypeTerms::SleepDisturbance)
                        .build_with_severities(severities.as_slice(), true),
                ),
            QuestionnaireItemBuilder::new(4)
                .stem("Feeling tired or having little energy")
                .conditions(
                    ConditionBuilder::new(PhenotypeTerms::Fatigue)
                        .build_with_severities(severities.as_slice(), true),
                ),
            QuestionnaireItemBuilder::new(4)
                .stem("Poor appetite or overeating")
                .conditions(
                    ConditionBuilder::new(PhenotypeTerms::AbnormalEatingBehavior)
                        .build_with_severities(severities.as_slice(), true),
                ),
            QuestionnaireItemBuilder::new(4)
                .stem("Feeling bad about yourself — or that you are a failure or have let yourself or your family down")
                .conditions(
                    ConditionBuilder::new(PhenotypeTerms::Guilt)
                        .build_with_severities(severities.as_slice(), true),
                ),
            QuestionnaireItemBuilder::new(4)
                .stem("Trouble concentrating on things, such as reading the newspaper or watching television")
                .conditions(
                    ConditionBuilder::new(PhenotypeTerms::DiminishedAbilityToConcentrate)
                        .build_with_severities(severities.as_slice(), true),
                ),
            QuestionnaireItemBuilder::new(4)
                .stem("Moving or speaking so slowly that other people could have noticed? Or the opposite — being so fidgety or restless that you have been moving around a lot more than usual")
                .conditions(
                    ConditionBuilder::new(PhenotypeTerms::PsychomotorDeterioration)
                        .build_with_severities(severities.as_slice(), true),
                ),
            QuestionnaireItemBuilder::new(4)
                .stem("Thoughts that you would be better off dead or of hurting yourself in some way")
                .conditions(

                    ConditionBuilder::new(PhenotypeTerms::SuicidalIdeation)
                        .build_with_severities(severities.as_slice(), true),
                ),
        ]);

    let builder = builder.map_err(|err| {
        RorschachError::BuildingError(format!("Error when building {name}: {}", err))
    })?;

    Ok(builder.build())
}
