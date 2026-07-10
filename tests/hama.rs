use robinson_group_rust_template::answer::Answer;
use robinson_group_rust_template::questionnaire_presets::QuestionnairePresets;
use robinson_group_rust_template::term::{PhenotypeTerms, SeverityTerms};
use robinson_group_rust_template::traits::AsTerm;

#[test]
fn test_hama() {
    let hama = QuestionnairePresets::HAMA.build(); // adjust variant name if it differs

    assert_eq!(hama.name(), "HAM-A");
    // 14 items, each scored on a 5-point Likert scale (0..=4)
    assert_eq!(hama.max_score(), 14f32 * 4f32);

    let answers = vec![
        Answer::new(0, Some(0.0)),  // Anxious mood          -> excluded
        Answer::new(1, Some(1.0)),  // Tension               -> Borderline
        Answer::new(2, Some(2.0)),  // Fears                 -> Moderate
        Answer::new(3, Some(3.0)),  // Insomnia              -> Moderate
        Answer::new(4, Some(4.0)),  // Intellectual          -> Profound
        Answer::new(5, Some(0.0)),  // Depressed mood        -> excluded
        Answer::new(6, Some(3.0)),  // Somatic (muscular)    -> no condition attached
        Answer::new(7, Some(2.0)),  // Somatic (sensory)     -> no condition attached
        Answer::new(8, Some(1.0)),  // Cardiovascular        -> Borderline
        Answer::new(9, Some(2.0)),  // Respiratory           -> Moderate
        Answer::new(10, Some(3.0)), // Gastrointestinal      -> Moderate
        Answer::new(11, Some(4.0)), // Genitourinary         -> Profound
        Answer::new(12, Some(0.0)), // Autonomic             -> excluded
        Answer::new(13, Some(1.0)), // Behavior at interview -> Borderline
    ];

    let res = hama.evaluate("some_id", answers.as_ref(), None).unwrap();

    // One entry per item. `None` means "this item has no phenotype at all"
    // (items 6 and 7 never got a `.conditions(...)` call) -- distinct from a
    // phenotype that IS present but excluded (items 0, 5, 12 at score 0).
    let expected: Vec<Option<(String, Option<String>)>> = vec![
        Some((PhenotypeTerms::Anxiety.as_term().to_string(), None)),
        Some((
            PhenotypeTerms::Agitation.as_term().to_string(),
            Some(SeverityTerms::Borderline.as_term().to_string()),
        )),
        Some((
            PhenotypeTerms::Phobia.as_term().to_string(),
            Some(SeverityTerms::Moderate.as_term().to_string()),
        )),
        Some((
            PhenotypeTerms::Insomnia.as_term().to_string(),
            Some(SeverityTerms::Moderate.as_term().to_string()),
        )),
        Some((
            PhenotypeTerms::CognitiveImpairment.as_term().to_string(),
            Some(SeverityTerms::Profound.as_term().to_string()),
        )),
        Some((PhenotypeTerms::Depression.as_term().to_string(), None)),
        None,
        None,
        Some((
            PhenotypeTerms::AbnormalityOfTheCardiovascularSystem
                .as_term()
                .to_string(),
            Some(SeverityTerms::Borderline.as_term().to_string()),
        )),
        Some((
            PhenotypeTerms::AbnormalityOfTheRespiratorySystem
                .as_term()
                .to_string(),
            Some(SeverityTerms::Moderate.as_term().to_string()),
        )),
        Some((
            PhenotypeTerms::AbnormalityOfTheGastrointestinalTract
                .as_term()
                .to_string(),
            Some(SeverityTerms::Moderate.as_term().to_string()),
        )),
        Some((
            PhenotypeTerms::AbnormalityOfTheGenitourinarySystem
                .as_term()
                .to_string(),
            Some(SeverityTerms::Profound.as_term().to_string()),
        )),
        Some((
            PhenotypeTerms::AbnormalAutonomicNervousSystemPhysiology
                .as_term()
                .to_string(),
            None,
        )),
        Some((
            PhenotypeTerms::AtypicalBehavior.as_term().to_string(),
            Some(SeverityTerms::Borderline.as_term().to_string()),
        )),
    ];

    let phenotypes = res.phenotypes();
    assert_eq!(
        phenotypes.len(),
        expected.len(),
        "expected one phenotype slot per item"
    );

    for (idx, (actual, expected)) in phenotypes.iter().zip(expected.iter()).enumerate() {
        match (actual, expected) {
            (None, None) => {
                // item without a `ConditionBuilder` correctly produced no phenotype
            }
            (Some(condition), Some((exp_term, exp_severity))) => {
                assert_eq!(condition.term().id(), exp_term, "wrong term at item {idx}");
                assert_eq!(
                    &Some(condition.severity().unwrap().id().to_string()),
                    exp_severity,
                    "wrong severity at item {idx}"
                );
                assert_eq!(
                    condition.excluded(),
                    exp_severity.is_none(),
                    "wrong excluded flag at item {idx}"
                );
            }
            (None, Some((exp_term, _))) => {
                panic!("item {idx}: expected phenotype {exp_term:?} but got none");
            }
            (Some(condition), None) => {
                panic!(
                    "item {idx}: expected no phenotype but got {:?}",
                    condition.term()
                );
            }
        }
    }
}
