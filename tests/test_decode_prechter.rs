use chrono::{DateTime, Days, NaiveDate, TimeZone, Utc};
use csv::Reader;

use rorschach_rs::answer::Answer;
use rorschach_rs::questionnaire_presets::QuestionnairePresets;
use rorschach_rs::questionnaire_result::QuestionnaireResult;
use rorschach_rs::traits::ToCsv;
use serde::Deserialize;
use std::fs::File;
use std::io::BufWriter;

#[derive(Debug, Deserialize)]
struct PHQ9Row {
    patient_id: String,
    t: u64,
    answer_0: Option<f32>,
    answer_1: Option<f32>,
    answer_2: Option<f32>,
    answer_3: Option<f32>,
    answer_4: Option<f32>,
    answer_5: Option<f32>,
    answer_6: Option<f32>,
    answer_7: Option<f32>,
    answer_8: Option<f32>,
}

#[derive(Debug, Deserialize)]
struct GAD7Row {
    patient_id: String,
    t: u64,
    answer_0: Option<f32>,
    answer_1: Option<f32>,
    answer_2: Option<f32>,
    answer_3: Option<f32>,
    answer_4: Option<f32>,
    answer_5: Option<f32>,
    answer_6: Option<f32>,
}

#[derive(Debug, Deserialize)]
struct ASRMRow {
    patient_id: String,
    t: u64,
    answer_0: Option<f32>,
    answer_1: Option<f32>,
    answer_2: Option<f32>,
    answer_3: Option<f32>,
    answer_4: Option<f32>,
}

fn to_date(t: u64) -> Option<DateTime<Utc>> {
    if let Ok(start_date) = NaiveDate::parse_from_str("2020-01-01", "%Y-%m-%d") {
        match start_date.checked_add_days(Days::new(t)) {
            None => {
                panic!("Could not convert time to date")
            }
            Some(naive) => Some(Utc.from_utc_datetime(&naive.and_hms_opt(0, 0, 0).unwrap())),
        }
    } else {
        panic!("Could not convert time to date")
    }
}

#[test]
#[ignore]
fn test_decode() {
    let mut phq9_data =
        Reader::from_path("/Users/rouvenreuter/data/bogus/balanced groups/PHQ-9.csv").unwrap();
    let mut gad7_data =
        Reader::from_path("/Users/rouvenreuter/data/bogus/balanced groups/GAD-7.csv").unwrap();
    let mut asrm_data =
        Reader::from_path("/Users/rouvenreuter/data/bogus/balanced groups/ASRM.csv").unwrap();

    let phq9 = QuestionnairePresets::PHQ9.build();
    let gad7 = QuestionnairePresets::GAD7.build();
    let asrm = QuestionnairePresets::ASRM.build();

    let mut results: Vec<QuestionnaireResult> = vec![];
    for result in phq9_data.deserialize() {
        let row: PHQ9Row = result.unwrap();
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
        let dt: Option<DateTime<Utc>> = to_date(row.t);

        results.push(
            phq9.evaluate(&row.patient_id.to_string(), answers.as_slice(), dt.as_ref())
                .unwrap(),
        );
    }

    for result in gad7_data.deserialize() {
        let row: GAD7Row = result.unwrap();
        let answers: Vec<Answer> = vec![
            Answer::new(1, row.answer_0),
            Answer::new(2, row.answer_1),
            Answer::new(3, row.answer_2),
            Answer::new(4, row.answer_3),
            Answer::new(5, row.answer_4),
            Answer::new(6, row.answer_5),
            Answer::new(7, row.answer_6),
        ];

        let dt: Option<DateTime<Utc>> = to_date(row.t);

        results.push(
            gad7.evaluate(&row.patient_id.to_string(), answers.as_slice(), dt.as_ref())
                .unwrap(),
        );
    }

    for result in asrm_data.deserialize() {
        let row: ASRMRow = result.unwrap();
        let answers: Vec<Answer> = vec![
            Answer::new(1, row.answer_0),
            Answer::new(2, row.answer_1),
            Answer::new(3, row.answer_2),
            Answer::new(4, row.answer_3),
            Answer::new(5, row.answer_4),
        ];

        let dt: Option<DateTime<Utc>> = to_date(row.t);

        results.push(
            asrm.evaluate(&row.patient_id.to_string(), answers.as_slice(), dt.as_ref())
                .unwrap(),
        );
    }

    let mut output =
        File::create("/Users/rouvenreuter/data/bogus/balanced groups/rorschach_output.csv")
            .unwrap();

    results
        .to_csv(&mut BufWriter::new(&mut output), true)
        .unwrap();
}
