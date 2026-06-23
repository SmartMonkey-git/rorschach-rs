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

pub(crate) fn gad7() -> Result<Questionnaire, RorschachError> {
    let severities = vec![
        (1, SeverityTerms::Borderline),
        (2, SeverityTerms::Moderate),
        (3, SeverityTerms::Profound),
    ];

    let builder = QuestionnaireBuilder::new("GAD-7", Box::new(SumScore))
        .recall_period(Duration::weeks(2))
        .interpretations(btreemap! {
            0  => Some(Condition::new_excluded(PhenotypeTerms::Anxiety)),
            1  => Some(Condition::without_time(PhenotypeTerms::Anxiety, SeverityTerms::Borderline)),
            5  => Some(Condition::without_time(PhenotypeTerms::Anxiety, SeverityTerms::Mild)),
            10 => Some(Condition::without_time(PhenotypeTerms::Anxiety, SeverityTerms::Moderate)),
            15 => Some(Condition::without_time(PhenotypeTerms::Anxiety, SeverityTerms::Severe)),
            20 => Some(Condition::without_time(PhenotypeTerms::Anxiety, SeverityTerms::Profound)),
        })
        .items([
            QuestionnaireItemBuilder::new(4)
                .stem("Feeling nervous, anxious, or on edge")
                .conditions(
                    0,
                    ConditionBuilder::new(PhenotypeTerms::Anxiety)
                        .build_with_severities(severities.as_slice(), true),
                ),
            QuestionnaireItemBuilder::new(4)
                .stem("Not being able to stop or control worrying") // TODO: Needs to get its own phenotype, current one is wrong
                .conditions(
                    1,
                    ConditionBuilder::new(PhenotypeTerms::Anxiety)
                        .build_with_severities(severities.as_slice(), true),
                ),
            QuestionnaireItemBuilder::new(4)
                .stem("Worrying too much about different things")
                .conditions(
                    2,
                    ConditionBuilder::new(PhenotypeTerms::Ruminations)
                        .build_with_severities(severities.as_slice(), true),
                ),
            QuestionnaireItemBuilder::new(4)
                .stem("Trouble relaxing")
                .conditions(
                    3,
                    ConditionBuilder::new(PhenotypeTerms::Agitation)
                        .build_with_severities(severities.as_slice(), true),
                ),
            QuestionnaireItemBuilder::new(4)
                .stem("Being so restless that it is hard to sit still")
                .conditions(
                    4,
                    ConditionBuilder::new(PhenotypeTerms::Restlessness)
                        .build_with_severities(severities.as_slice(), true),
                ),
            QuestionnaireItemBuilder::new(4)
                .stem("Becoming easily annoyed or irritable")
                .conditions(
                    5,
                    ConditionBuilder::new(PhenotypeTerms::Irritability)
                        .build_with_severities(severities.as_slice(), true),
                ),
            QuestionnaireItemBuilder::new(4)
                .stem("Feeling afraid, as if something awful might happen")
                .conditions(
                    6,
                    ConditionBuilder::new(PhenotypeTerms::AnticipatoryAnxiety)
                        .build_with_severities(severities.as_slice(), true),
                ),
        ]);

    Ok(builder.build())
}
