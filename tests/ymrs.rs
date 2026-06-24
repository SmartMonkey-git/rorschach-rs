use robinson_group_rust_template::answer::Answer;
use robinson_group_rust_template::questionnaire_presets::QuestionnairePresets;
use robinson_group_rust_template::term::{PhenotypeTerms, SeverityTerms};
use robinson_group_rust_template::traits::AsTerm;

#[test]
fn test_ymrs() {
    let ymrs = QuestionnairePresets::YMRS.build();

    assert_eq!(ymrs.name(), "YMRS");
    assert_eq!(ymrs.max_score(), (7f32 * 4f32) + (4f32 * 8f32));

    let answers = vec![
        Answer::new(0, Some(0.0)),  // Linear: Excluded
        Answer::new(1, Some(1.0)),  // Linear: Borderline
        Answer::new(2, Some(2.0)),  // Linear: Moderate
        Answer::new(3, Some(3.0)),  // Linear: Moderate
        Answer::new(4, Some(8.0)),  // Amplified: Profound
        Answer::new(5, Some(2.0)),  // Amplified: Borderline
        Answer::new(6, Some(4.0)),  // Linear: Profound
        Answer::new(7, Some(4.0)),  // Amplified: Moderate
        Answer::new(8, Some(6.0)),  // Amplified: Moderate
        Answer::new(9, Some(0.0)),  // Linear: Excluded
        Answer::new(10, Some(1.0)), // Linear: Borderline
    ];

    let res = ymrs.evaluate("some_id", answers.as_ref(), None).unwrap();

    let expected_terms = vec![
        (PhenotypeTerms::Euphoria.as_term(), None),
        (
            PhenotypeTerms::Agitation.as_term(),
            Some(SeverityTerms::Borderline.as_term()),
        ),
        (
            PhenotypeTerms::AmplificationOfSexualBehavior.as_term(),
            Some(SeverityTerms::Moderate.as_term()),
        ),
        (
            PhenotypeTerms::DecreasedNeedForSleep.as_term(),
            Some(SeverityTerms::Moderate.as_term()),
        ),
        (
            PhenotypeTerms::Irritability.as_term(),
            Some(SeverityTerms::Profound.as_term()),
        ),
        (
            PhenotypeTerms::PressuredSpeech.as_term(),
            Some(SeverityTerms::Borderline.as_term()),
        ),
        (
            PhenotypeTerms::AbnormallyRapidThoughtProcess.as_term(),
            Some(SeverityTerms::Profound.as_term()),
        ),
        (
            PhenotypeTerms::DisorderOfThoughtContent.as_term(),
            Some(SeverityTerms::Moderate.as_term()),
        ),
        (
            PhenotypeTerms::AggressiveBehavior.as_term(),
            Some(SeverityTerms::Moderate.as_term()),
        ),
        (PhenotypeTerms::SelfNeglect.as_term(), None),
        (
            PhenotypeTerms::Delusion.as_term(),
            Some(SeverityTerms::Borderline.as_term()),
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
