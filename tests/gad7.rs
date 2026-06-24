use robinson_group_rust_template::answer::Answer;
use robinson_group_rust_template::questionnaire_presets::QuestionnairePresets;
use robinson_group_rust_template::term::{PhenotypeTerms, SeverityTerms};
use robinson_group_rust_template::traits::AsTerm;

#[test]
fn test_gad7() {
    let gad7 = QuestionnairePresets::GAD7.build();

    assert_eq!(gad7.name(), "GAD-7");
    assert_eq!(gad7.max_score(), 7f32 * 3f32);

    let answers = vec![
        Answer::new(0, Some(0.0)),
        Answer::new(1, Some(1.0)),
        Answer::new(2, Some(2.0)),
        Answer::new(3, Some(3.0)),
        Answer::new(4, Some(0.0)),
        Answer::new(5, Some(1.0)),
        Answer::new(6, Some(2.0)),
    ];

    let res = gad7.evaluate("some_id", answers.as_ref(), None).unwrap();

    let expected_terms = [
        (PhenotypeTerms::Anxiety.as_term(), None),
        (
            PhenotypeTerms::Anxiety.as_term(),
            Some(SeverityTerms::Borderline.as_term()),
        ),
        (
            PhenotypeTerms::Ruminations.as_term(),
            Some(SeverityTerms::Moderate.as_term()),
        ),
        (
            PhenotypeTerms::Agitation.as_term(),
            Some(SeverityTerms::Profound.as_term()),
        ),
        (PhenotypeTerms::Restlessness.as_term(), None),
        (
            PhenotypeTerms::Irritability.as_term(),
            Some(SeverityTerms::Borderline.as_term()),
        ),
        (
            PhenotypeTerms::AnticipatoryAnxiety.as_term(),
            Some(SeverityTerms::Moderate.as_term()),
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
