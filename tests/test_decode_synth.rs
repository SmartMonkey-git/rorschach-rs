use chrono::{DateTime, NaiveDate, TimeZone, Utc};
use csv::Reader;
use robinson_group_rust_template::answer::Answer;
use robinson_group_rust_template::questionnaire_presets::QuestionnairePresets;
use robinson_group_rust_template::questionnaire_result::QuestionnaireResult;
use robinson_group_rust_template::traits::ToCsv;
use serde::Deserialize;
use std::fs::File;
use std::io::BufWriter;

#[derive(Debug, Deserialize)]
struct PHQ9Row {
    record_id: String,
    phq_timestamp: String,
    phq9_1a: Option<f32>,
    phq9_1b: Option<f32>,
    phq9_1c: Option<f32>,
    phq9_1d: Option<f32>,
    phq9_1e: Option<f32>,
    phq9_1f: Option<f32>,
    phq9_1g: Option<f32>,
    phq9_1h: Option<f32>,
    phq9_1i: Option<f32>,
}

#[derive(Debug, Deserialize)]
struct GAD7Row {
    record_id: String,
    gad_timestamp: String,
    gad_1: Option<f32>,
    gad_2: Option<f32>,
    gad_3: Option<f32>,
    gad_4: Option<f32>,
    gad_5: Option<f32>,
    gad_6: Option<f32>,
    gad_7: Option<f32>,
}

#[derive(Debug, Deserialize)]
struct ASRMRow {
    record_id: String,
    asrm_timestamp: String,
    asrm_1: Option<f32>,
    asrm_2: Option<f32>,
    asrm_3: Option<f32>,
    asrm_4: Option<f32>,
    asrm_5: Option<f32>,
}

fn to_date(string_date: &str) -> Option<DateTime<Utc>> {
    let dt: Option<DateTime<Utc>> = match NaiveDate::parse_from_str(string_date, "%Y-%m-%d") {
        Ok(naive) => Some(Utc.from_utc_datetime(&naive.and_hms_opt(0, 0, 0).unwrap())),
        Err(_) => None,
    };
    dt
}

#[test]
#[ignore]
fn test_decode() {
    let mut phq9_data =
        Reader::from_path("/Users/rouvenreuter/data/prechter/out/phq_item_level.csv").unwrap();
    let mut gad7_data =
        Reader::from_path("/Users/rouvenreuter/data/prechter/out/gad_item_level.csv").unwrap();
    let mut asrm_data =
        Reader::from_path("/Users/rouvenreuter/data/prechter/out/asrm_item_level.csv").unwrap();

    let phq9 = QuestionnairePresets::PHQ9.build();
    let gad7 = QuestionnairePresets::GAD7.build();
    let asrm = QuestionnairePresets::ASRM.build();

    let mut results: Vec<QuestionnaireResult> = vec![];
    for result in phq9_data.deserialize() {
        let row: PHQ9Row = result.unwrap();
        let answers: Vec<Answer> = vec![
            Answer::new(1, row.phq9_1a),
            Answer::new(2, row.phq9_1b),
            Answer::new(3, row.phq9_1c),
            Answer::new(4, row.phq9_1d),
            Answer::new(5, row.phq9_1e),
            Answer::new(6, row.phq9_1f),
            Answer::new(7, row.phq9_1g),
            Answer::new(8, row.phq9_1h),
            Answer::new(9, row.phq9_1i),
        ];
        let dt: Option<DateTime<Utc>> = to_date(&row.phq_timestamp);

        results.push(
            phq9.evaluate(&row.record_id.to_string(), answers.as_slice(), dt.as_ref())
                .unwrap(),
        );
    }

    for result in gad7_data.deserialize() {
        let row: GAD7Row = result.unwrap();
        let answers: Vec<Answer> = vec![
            Answer::new(1, row.gad_1),
            Answer::new(2, row.gad_2),
            Answer::new(3, row.gad_3),
            Answer::new(4, row.gad_4),
            Answer::new(5, row.gad_5),
            Answer::new(6, row.gad_6),
            Answer::new(7, row.gad_7),
        ];

        let dt: Option<DateTime<Utc>> = to_date(&row.gad_timestamp);

        results.push(
            gad7.evaluate(&row.record_id.to_string(), answers.as_slice(), dt.as_ref())
                .unwrap(),
        );
    }

    for result in asrm_data.deserialize() {
        let row: ASRMRow = result.unwrap();
        let answers: Vec<Answer> = vec![
            Answer::new(1, row.asrm_1),
            Answer::new(2, row.asrm_2),
            Answer::new(3, row.asrm_3),
            Answer::new(4, row.asrm_4),
            Answer::new(5, row.asrm_5),
        ];

        let dt: Option<DateTime<Utc>> = to_date(&row.asrm_timestamp);

        results.push(
            asrm.evaluate(&row.record_id.to_string(), answers.as_slice(), dt.as_ref())
                .unwrap(),
        );
    }

    let mut output =
        File::create("/Users/rouvenreuter/data/prechter/rorschach_output.csv").unwrap();

    results
        .to_csv(&mut BufWriter::new(&mut output), true)
        .unwrap();
}
