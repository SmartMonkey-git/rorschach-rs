use chrono::NaiveDateTime;
use polars::prelude::{CsvParseOptions, CsvReadOptions, CsvReader, NullValues, SerReader};
use robinson_group_rust_template::answer::Answer;
use robinson_group_rust_template::available_tests::AvailableTest;
use robinson_group_rust_template::questionnaire_result::QuestionnaireResult;
use std::fs::File;

#[test]
fn test_integration_declared_version() {
    let phq9_patent_data = File::open("/Users/rouvenreuter/data/phq9/phq9_patent_data.csv")
        .expect("Could not open file");
    let phq9_questioner_data = File::open("/Users/rouvenreuter/data/phq9/phq9_questioner_data.csv")
        .expect("Could not open file");

    // 2. Read the CSV into a DataFrame
    let parse_options = CsvParseOptions::default()
        .with_null_values(Some(NullValues::AllColumnsSingle("NA".into())));
    let options = CsvReadOptions::default()
        .with_has_header(true)
        .with_parse_options(parse_options);
    let phq9_patent_data = CsvReader::new(phq9_patent_data)
        .with_options(options)
        .finish()
        .unwrap();

    let parse_options = CsvParseOptions::default()
        .with_null_values(Some(NullValues::AllColumnsSingle("NA".into())));
    let options = CsvReadOptions::default()
        .with_has_header(true)
        .with_parse_options(parse_options);
    let phq9_questioner_data = CsvReader::new(phq9_questioner_data)
        .with_options(options)
        .finish()
        .unwrap();

    // 3. Print the DataFrame
    println!("{:?}", phq9_questioner_data);
    println!("{:?}", phq9_patent_data);

    let mut questionnaire_results: Vec<QuestionnaireResult> = vec![];
    let quesstioneere = AvailableTest::PHQ9.build();
    let mut max_iterations = 0;

    for idx in 0..phq9_questioner_data.height() {
        let row = phq9_questioner_data.get(idx).unwrap();
        let mut answers: Vec<Answer> = vec![];
        for answer_idx in 0..(row.len() - 2) {
            let value = row
                .get(answer_idx)
                .unwrap()
                .str_value()
                .parse::<f32>()
                .unwrap();
            answers.push(Answer::new(answer_idx, value))
        }

        let date_str = row.get(row.len() - 2).unwrap().str_value().clone();

        let format = "%Y-%m-%d %H:%M:%S";

        let parsed_date = NaiveDateTime::parse_from_str(&date_str, format)
            .expect(&format!("Failed to parse the date string {}", date_str));

        let patient_id = row.get(row.len() - 3).unwrap().str_value().clone();

        match quesstioneere.evaluate(&patient_id, answers.as_slice(), Some(parsed_date.and_utc())) {
            Ok(qr) => questionnaire_results.push(qr),
            Err(err) => {
                dbg!(&answers);
                eprintln!("{}", err);
            }
        }
        max_iterations += 1;
        if max_iterations == 50 {
            //break;
        }
    }
    println!("Got {} results", questionnaire_results.len());

    for q in questionnaire_results {
        println!("{}", q);
        break;
    }
}
