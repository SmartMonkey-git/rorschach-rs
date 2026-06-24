use robinson_group_rust_template::answer::Answer;
use robinson_group_rust_template::questionnaire_presets::QuestionnairePresets;
use robinson_group_rust_template::term::{PhenotypeTerms, SeverityTerms};
use robinson_group_rust_template::traits::AsTerm;

#[test]
fn test_madrs() {
    let madrs = QuestionnairePresets::MADRS.build();

    assert_eq!(madrs.name(), "MADRS");
    assert_eq!(madrs.max_score(), 10f32 * 6f32); // 10 items, max score 6 (0-6 scale)

    let answers = vec![
        Answer::new(0, Some(0.0)),
        Answer::new(1, Some(1.0)),
        Answer::new(2, Some(2.0)),
        Answer::new(3, Some(3.0)),
        Answer::new(4, Some(4.0)),
        Answer::new(5, Some(5.0)),
        Answer::new(6, Some(6.0)),
        Answer::new(7, Some(0.0)),
        Answer::new(8, Some(1.0)),
        Answer::new(9, Some(2.0)),
    ];

    let res = madrs.evaluate("some_id", answers.as_ref(), None).unwrap();

    let expected_terms = [
        (PhenotypeTerms::PathologicalSadness.as_term(), None),
        (
            PhenotypeTerms::PathologicalSadness.as_term(),
            Some(SeverityTerms::Borderline.as_term()),
        ),
        (
            PhenotypeTerms::Anxiety.as_term(),
            Some(SeverityTerms::Mild.as_term()),
        ),
        (
            PhenotypeTerms::DecreasedNeedForSleep.as_term(),
            Some(SeverityTerms::Moderate.as_term()),
        ),
        (
            PhenotypeTerms::PoorAppetite.as_term(),
            Some(SeverityTerms::Moderate.as_term()),
        ),
        (
            PhenotypeTerms::DiminishedAbilityToConcentrate.as_term(),
            Some(SeverityTerms::Severe.as_term()),
        ),
        (
            PhenotypeTerms::Fatigue.as_term(),
            Some(SeverityTerms::Profound.as_term()),
        ),
        (PhenotypeTerms::Apathy.as_term(), None),
        (
            PhenotypeTerms::Hopelessness.as_term(),
            Some(SeverityTerms::Borderline.as_term()),
        ),
        (
            PhenotypeTerms::SuicidalIdeation.as_term(),
            Some(SeverityTerms::Mild.as_term()),
        ),
    ];

    for (idx, pt) in res.phenotypes().iter().enumerate() {
        if let Some(condition) = pt {
            let (exp_term, exp_severity) = &expected_terms[idx];
            assert_eq!(condition.term(), exp_term);
            assert_eq!(condition.severity(), exp_severity.as_ref());

            if exp_severity.is_none() {
                assert!(condition.excluded())
            }
        }
    }
}
