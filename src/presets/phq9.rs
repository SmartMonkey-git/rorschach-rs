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
    let severities = vec![
        SeverityTerms::Borderline,
        SeverityTerms::Moderate,
        SeverityTerms::Profound,
    ];

    let builder = QuestionnaireBuilder::new("PHQ-9", Box::new(SumScore))
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
                    0,
                    ConditionBuilder::new(PhenotypeTerms::Anhedonia)
                        .build_with_severities(severities.as_slice(), true)?,
                ),
            QuestionnaireItemBuilder::new(4)
                .stem("Feeling down, depressed, or hopeless")
                .conditions(
                    1,
                    ConditionBuilder::new(PhenotypeTerms::PathologicalSadness)
                        .build_with_severities(severities.as_slice(), true)?,
                ),
            QuestionnaireItemBuilder::new(4)
                .stem("Trouble falling or staying asleep, or sleeping too much")
                .conditions(
                    2,
                    ConditionBuilder::new(PhenotypeTerms::SleepDisturbance)
                        .build_with_severities(severities.as_slice(), true)?,
                ),
            QuestionnaireItemBuilder::new(4)
                .stem("Feeling tired or having little energy")
                .conditions(
                    3,
                    ConditionBuilder::new(PhenotypeTerms::Fatigue)
                        .build_with_severities(severities.as_slice(), true)?,
                ),
            QuestionnaireItemBuilder::new(4)
                .stem("Poor appetite or overeating")
                .conditions(
                    4,
                    ConditionBuilder::new(PhenotypeTerms::AbnormalEatingBehavior)
                        .build_with_severities(severities.as_slice(), true)?,
                ),
            QuestionnaireItemBuilder::new(4)
                .stem("Feeling bad about yourself — or that you are a failure or have let yourself or your family down")
                .conditions(
                    5,
                    ConditionBuilder::new(PhenotypeTerms::Guilt)
                        .build_with_severities(severities.as_slice(), true)?,
                ),
            QuestionnaireItemBuilder::new(4)
                .stem("Trouble concentrating on things, such as reading the newspaper or watching television")
                .conditions(
                    6,
                    ConditionBuilder::new(PhenotypeTerms::DiminishedAbilityToConcentrate)
                        .build_with_severities(severities.as_slice(), true)?,
                ),
            QuestionnaireItemBuilder::new(4)
                .stem("Moving or speaking so slowly that other people could have noticed? Or the opposite — being so fidgety or restless that you have been moving around a lot more than usual")
                .conditions(
                    7,
                    ConditionBuilder::new(PhenotypeTerms::PsychomotorDeterioration)
                        .build_with_severities(severities.as_slice(), true)?,
                ),
            QuestionnaireItemBuilder::new(4)
                .stem("Thoughts that you would be better off dead or of hurting yourself in some way")
                .conditions(
                    8,
                    ConditionBuilder::new(PhenotypeTerms::SuicidalIdeation)
                        .build_with_severities(severities.as_slice(), true)?,
                ),
        ]);

    Ok(builder.build())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_values() {
        let questionnaire = phq9().unwrap();

        assert_eq!(questionnaire.name(), "PHQ-9");
        assert_eq!(questionnaire.items().len(), 9);

        assert_eq!(questionnaire.duration(), Some(&Duration::weeks(2)));

        let expected_phenotypes = [
            vec![
                Condition::new_excluded(PhenotypeTerms::Anhedonia),
                Condition::without_time(PhenotypeTerms::Anhedonia, SeverityTerms::Borderline),
                Condition::without_time(PhenotypeTerms::Anhedonia, SeverityTerms::Moderate),
                Condition::without_time(PhenotypeTerms::Anhedonia, SeverityTerms::Profound),
            ],
            vec![
                Condition::new_excluded(PhenotypeTerms::PathologicalSadness),
                Condition::without_time(
                    PhenotypeTerms::PathologicalSadness,
                    SeverityTerms::Borderline,
                ),
                Condition::without_time(
                    PhenotypeTerms::PathologicalSadness,
                    SeverityTerms::Moderate,
                ),
                Condition::without_time(
                    PhenotypeTerms::PathologicalSadness,
                    SeverityTerms::Profound,
                ),
            ],
            vec![
                Condition::new_excluded(PhenotypeTerms::SleepDisturbance),
                Condition::without_time(
                    PhenotypeTerms::SleepDisturbance,
                    SeverityTerms::Borderline,
                ),
                Condition::without_time(PhenotypeTerms::SleepDisturbance, SeverityTerms::Moderate),
                Condition::without_time(PhenotypeTerms::SleepDisturbance, SeverityTerms::Profound),
            ],
            vec![
                Condition::new_excluded(PhenotypeTerms::Fatigue),
                Condition::without_time(PhenotypeTerms::Fatigue, SeverityTerms::Borderline),
                Condition::without_time(PhenotypeTerms::Fatigue, SeverityTerms::Moderate),
                Condition::without_time(PhenotypeTerms::Fatigue, SeverityTerms::Profound),
            ],
            vec![
                Condition::new_excluded(PhenotypeTerms::AbnormalEatingBehavior),
                Condition::without_time(
                    PhenotypeTerms::AbnormalEatingBehavior,
                    SeverityTerms::Borderline,
                ),
                Condition::without_time(
                    PhenotypeTerms::AbnormalEatingBehavior,
                    SeverityTerms::Moderate,
                ),
                Condition::without_time(
                    PhenotypeTerms::AbnormalEatingBehavior,
                    SeverityTerms::Profound,
                ),
            ],
            vec![
                Condition::new_excluded(PhenotypeTerms::Guilt),
                Condition::without_time(PhenotypeTerms::Guilt, SeverityTerms::Borderline),
                Condition::without_time(PhenotypeTerms::Guilt, SeverityTerms::Moderate),
                Condition::without_time(PhenotypeTerms::Guilt, SeverityTerms::Profound),
            ],
            vec![
                Condition::new_excluded(PhenotypeTerms::DiminishedAbilityToConcentrate),
                Condition::without_time(
                    PhenotypeTerms::DiminishedAbilityToConcentrate,
                    SeverityTerms::Borderline,
                ),
                Condition::without_time(
                    PhenotypeTerms::DiminishedAbilityToConcentrate,
                    SeverityTerms::Moderate,
                ),
                Condition::without_time(
                    PhenotypeTerms::DiminishedAbilityToConcentrate,
                    SeverityTerms::Profound,
                ),
            ],
            vec![
                Condition::new_excluded(PhenotypeTerms::PsychomotorDeterioration),
                Condition::without_time(
                    PhenotypeTerms::PsychomotorDeterioration,
                    SeverityTerms::Borderline,
                ),
                Condition::without_time(
                    PhenotypeTerms::PsychomotorDeterioration,
                    SeverityTerms::Moderate,
                ),
                Condition::without_time(
                    PhenotypeTerms::PsychomotorDeterioration,
                    SeverityTerms::Profound,
                ),
            ],
            vec![
                Condition::new_excluded(PhenotypeTerms::SuicidalIdeation),
                Condition::without_time(
                    PhenotypeTerms::SuicidalIdeation,
                    SeverityTerms::Borderline,
                ),
                Condition::without_time(PhenotypeTerms::SuicidalIdeation, SeverityTerms::Moderate),
                Condition::without_time(PhenotypeTerms::SuicidalIdeation, SeverityTerms::Profound),
            ],
        ];

        for (idx, item) in questionnaire.items().iter().enumerate() {
            item.conditions(idx as i16)
                .iter()
                .enumerate()
                .for_each(|(c_idx, c)| assert_eq!(&expected_phenotypes[idx][c_idx], c))
        }

        assert!(questionnaire.interpretation().contains_key(&0));
        assert!(questionnaire.interpretation().contains_key(&2));
        assert!(questionnaire.interpretation().contains_key(&5));
        assert!(questionnaire.interpretation().contains_key(&10));
        assert!(questionnaire.interpretation().contains_key(&15));
        assert!(questionnaire.interpretation().contains_key(&20));
    }
}
