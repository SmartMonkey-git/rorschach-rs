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
        SeverityTerms::Borderline,
        SeverityTerms::Moderate,
        SeverityTerms::Moderate,
        SeverityTerms::Profound,
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
                0,
                ConditionBuilder::new(PhenotypeTerms::Euphoria)
                    .build_with_severities(severities.as_slice(), true)?,
            ),
            QuestionnaireItemBuilder::new(5).conditions(
                1,
                ConditionBuilder::new(PhenotypeTerms::Grandiosity)
                    .build_with_severities(severities.as_slice(), true)?,
            ),
            QuestionnaireItemBuilder::new(5).conditions(
                2,
                ConditionBuilder::new(PhenotypeTerms::DecreasedNeedForSleep)
                    .build_with_severities(severities.as_slice(), true)?,
            ),
            QuestionnaireItemBuilder::new(5).conditions(
                3,
                ConditionBuilder::new(PhenotypeTerms::PressuredSpeech)
                    .build_with_severities(severities.as_slice(), true)?,
            ),
            QuestionnaireItemBuilder::new(5).conditions(
                4,
                ConditionBuilder::new(PhenotypeTerms::Agitation)
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
        let questionnaire = asrm().unwrap();

        assert_eq!(questionnaire.name(), "ASRM");
        assert_eq!(questionnaire.items().len(), 5);

        assert_eq!(questionnaire.duration(), Some(&Duration::weeks(2)));

        let expected_phenotypes = [
            vec![
                Condition::new_excluded(PhenotypeTerms::Euphoria),
                Condition::without_time(PhenotypeTerms::Euphoria, SeverityTerms::Borderline),
                Condition::without_time(PhenotypeTerms::Euphoria, SeverityTerms::Moderate),
                Condition::without_time(PhenotypeTerms::Euphoria, SeverityTerms::Moderate),
                Condition::without_time(PhenotypeTerms::Euphoria, SeverityTerms::Profound),
            ],
            vec![
                Condition::new_excluded(PhenotypeTerms::Grandiosity),
                Condition::without_time(PhenotypeTerms::Grandiosity, SeverityTerms::Borderline),
                Condition::without_time(PhenotypeTerms::Grandiosity, SeverityTerms::Moderate),
                Condition::without_time(PhenotypeTerms::Grandiosity, SeverityTerms::Moderate),
                Condition::without_time(PhenotypeTerms::Grandiosity, SeverityTerms::Profound),
            ],
            vec![
                Condition::new_excluded(PhenotypeTerms::DecreasedNeedForSleep),
                Condition::without_time(
                    PhenotypeTerms::DecreasedNeedForSleep,
                    SeverityTerms::Borderline,
                ),
                Condition::without_time(
                    PhenotypeTerms::DecreasedNeedForSleep,
                    SeverityTerms::Moderate,
                ),
                Condition::without_time(
                    PhenotypeTerms::DecreasedNeedForSleep,
                    SeverityTerms::Moderate,
                ),
                Condition::without_time(
                    PhenotypeTerms::DecreasedNeedForSleep,
                    SeverityTerms::Profound,
                ),
            ],
            vec![
                Condition::new_excluded(PhenotypeTerms::PressuredSpeech),
                Condition::without_time(PhenotypeTerms::PressuredSpeech, SeverityTerms::Borderline),
                Condition::without_time(PhenotypeTerms::PressuredSpeech, SeverityTerms::Moderate),
                Condition::without_time(PhenotypeTerms::PressuredSpeech, SeverityTerms::Moderate),
                Condition::without_time(PhenotypeTerms::PressuredSpeech, SeverityTerms::Profound),
            ],
            vec![
                Condition::new_excluded(PhenotypeTerms::Agitation),
                Condition::without_time(PhenotypeTerms::Agitation, SeverityTerms::Borderline),
                Condition::without_time(PhenotypeTerms::Agitation, SeverityTerms::Moderate),
                Condition::without_time(PhenotypeTerms::Agitation, SeverityTerms::Moderate),
                Condition::without_time(PhenotypeTerms::Agitation, SeverityTerms::Profound),
            ],
        ];

        for (idx, item) in questionnaire.items().iter().enumerate() {
            item.conditions(idx as i16)
                .iter()
                .enumerate()
                .for_each(|(c_idx, c)| assert_eq!(&expected_phenotypes[idx][c_idx], c))
        }

        assert!(questionnaire.interpretation().contains_key(&0));
        assert!(questionnaire.interpretation().contains_key(&5));
        assert!(questionnaire.interpretation().contains_key(&8));
        assert!(questionnaire.interpretation().contains_key(&11));
        assert!(questionnaire.interpretation().contains_key(&14));
        assert!(questionnaire.interpretation().contains_key(&17));
        assert!(questionnaire.interpretation().contains_key(&20));
    }
}
