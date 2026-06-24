use robinson_group_rust_template::answer::Answer;
use robinson_group_rust_template::questionnaire_presets::QuestionnairePresets;
use robinson_group_rust_template::term::{PhenotypeTerms, SeverityTerms};
use robinson_group_rust_template::traits::AsTerm;

#[test]
fn test_asrm() {
    let asrm = QuestionnairePresets::ASRM.build();

    assert_eq!(asrm.name(), "ASRM");
    assert_eq!(asrm.max_score(), 5f32 * 4f32);

    let answers = vec![
        Answer::new(0, Some(0.0)),
        Answer::new(1, Some(1.0)),
        Answer::new(2, Some(2.0)),
        Answer::new(3, Some(3.0)),
        Answer::new(4, Some(4.0)),
    ];

    let res = asrm.evaluate("some_id", answers.as_ref(), None).unwrap();

    let expected_terms = vec![
        (PhenotypeTerms::Euphoria.as_term(), None),
        (
            PhenotypeTerms::Grandiosity.as_term(),
            Some(SeverityTerms::Borderline.as_term()),
        ),
        (
            PhenotypeTerms::DecreasedNeedForSleep.as_term(),
            Some(SeverityTerms::Moderate.as_term()),
        ),
        (
            PhenotypeTerms::PressuredSpeech.as_term(),
            Some(SeverityTerms::Moderate.as_term()),
        ),
        (
            PhenotypeTerms::Agitation.as_term(),
            Some(SeverityTerms::Profound.as_term()),
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
