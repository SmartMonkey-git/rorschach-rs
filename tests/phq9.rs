use rorschach_rs::answer::Answer;
use rorschach_rs::questionnaire_presets::QuestionnairePresets;
use rorschach_rs::term::{PhenotypeTerms, SeverityTerms};
use rorschach_rs::traits::AsTerm;

#[test]
fn test_phq9() {
    let phq9 = QuestionnairePresets::PHQ9.build();

    assert_eq!(phq9.name(), "PHQ-9");
    assert_eq!(phq9.max_score(), 9f32 * 3f32);

    let answers = vec![
        Answer::new(0, Some(2.0)),
        Answer::new(1, Some(3.0)),
        Answer::new(2, Some(0.0)),
        Answer::new(3, Some(0.0)),
        Answer::new(4, Some(0.0)),
        Answer::new(5, Some(0.0)),
        Answer::new(6, Some(1.0)),
        Answer::new(7, Some(0.0)),
        Answer::new(8, Some(0.0)),
    ];

    let res = phq9.evaluate("some_id", answers.as_ref(), None).unwrap();

    let expected_terms = vec![
        (
            PhenotypeTerms::Anhedonia.as_term(),
            Some(SeverityTerms::Moderate.as_term()),
        ),
        (
            PhenotypeTerms::PathologicalSadness.as_term(),
            Some(SeverityTerms::Profound.as_term()),
        ),
        (PhenotypeTerms::SleepDisturbance.as_term(), None),
        (PhenotypeTerms::Fatigue.as_term(), None),
        (PhenotypeTerms::AbnormalEatingBehavior.as_term(), None),
        (PhenotypeTerms::Guilt.as_term(), None),
        (
            PhenotypeTerms::DiminishedAbilityToConcentrate.as_term(),
            Some(SeverityTerms::Borderline.as_term()),
        ),
        (PhenotypeTerms::PsychomotorDeterioration.as_term(), None),
        (PhenotypeTerms::SuicidalIdeation.as_term(), None),
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
