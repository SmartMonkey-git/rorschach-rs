use chrono::{DateTime, Duration, NaiveDateTime, TimeZone, Utc};
use csv::Reader;
use robinson_group_rust_template::answer::Answer;
use robinson_group_rust_template::questionnaire_presets::QuestionnairePresets;
use robinson_group_rust_template::questionnaire_result::QuestionnaireResult;
use robinson_group_rust_template::traits::ToCsv;
use serde::Deserialize;
use std::fs::File;
use std::io::BufWriter;

#[derive(Debug, Deserialize)]
struct Row {
    patient_id: String,
    t: u64,
    answer_0: f32,
    answer_1: f32,
    answer_2: f32,
    answer_3: f32,
    answer_4: f32,
    answer_5: f32,
    answer_6: f32,
    answer_7: f32,
    answer_8: f32,
}

#[test]
fn test_decode() {
    let mut rdr = Reader::from_path("/Users/rouvenreuter/data/bogus/PHQ-9.csv").unwrap();
    let phq9 = QuestionnairePresets::PHQ9.build();

    let mut results: Vec<QuestionnaireResult> = vec![];
    for result in rdr.deserialize() {
        let row: Row = result.unwrap();
        let answers: Vec<Answer> = vec![
            Answer::new(1, row.answer_0),
            Answer::new(2, row.answer_1),
            Answer::new(3, row.answer_2),
            Answer::new(4, row.answer_3),
            Answer::new(5, row.answer_4),
            Answer::new(6, row.answer_5),
            Answer::new(7, row.answer_6),
            Answer::new(8, row.answer_7),
            Answer::new(9, row.answer_8),
        ];

        let sim_start: DateTime<Utc> = Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap();

        let dt: DateTime<Utc> = sim_start + Duration::days(row.t as i64);

        results.push(
            phq9.evaluate(&row.patient_id.to_string(), answers.as_slice(), Some(&dt))
                .unwrap(),
        );
    }

    let mut output = File::create("/Users/rouvenreuter/data/bogus/rorschach_output.csv").unwrap();

    results.to_csv(&mut BufWriter::new(&mut output)).unwrap();
}
