use chrono::NaiveDateTime;
use polars::prelude::{CsvParseOptions, CsvReadOptions, CsvReader, NullValues, SerReader};
use robinson_group_rust_template::answer::Answer;
use robinson_group_rust_template::available_tests::AvailableTest;
use robinson_group_rust_template::questionnaire_result::QuestionnaireResult;
use std::error::Error;
use std::fs::File;

use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize)]
pub struct DiagnosisRow<'a> {
    pub questionnaire_id: &'a str,
    pub taken_at: Option<DateTime<Utc>>,
    pub diagnosis_term_id: &'a str,
    pub diagnosis_term_label: &'a str,
    pub severity_id: Option<&'a str>,
    pub severity_label: Option<&'a str>,
}

#[derive(Serialize)]
pub struct PhenotypeRow<'a> {
    pub questionnaire_id: &'a str,
    pub phenotype_type_id: &'a str,
    pub phenotype_type_label: &'a str,
    pub severity_id: Option<&'a str>,
    pub severity_label: Option<&'a str>,
    pub observed_start: Option<DateTime<Utc>>,
    pub observed_end: Option<DateTime<Utc>>,
}

pub fn export_to_csvs(results: &[QuestionnaireResult]) -> Result<(), Box<dyn Error>> {
    // Initialize the CSV writers
    let mut diagnosis_wtr = csv::Writer::from_path("/Users/smart_monkey/data/phq9/diagnoses.csv")?;
    let mut phenotype_wtr = csv::Writer::from_path("/Users/smart_monkey/data/phq9/phenotypes.csv")?;

    for res in results {
        // 1. Process Diagnoses
        if let Some(diag) = &res.diagnosis() {
            let row = DiagnosisRow {
                questionnaire_id: &res.id(),
                taken_at: res.taken_at(),
                diagnosis_term_id: &diag.term().id(),
                diagnosis_term_label: &diag.term().label(),
                severity_id: diag.severity().as_ref().map(|s| s.id()),
                severity_label: diag.severity().as_ref().map(|s| s.label()),
            };
            diagnosis_wtr.serialize(row)?;
        }

        // 2. Process Phenotypes
        for pheno in res.phenotypes() {
            let row = PhenotypeRow {
                questionnaire_id: &res.id(),
                phenotype_type_id: &pheno.r#type().id(),
                phenotype_type_label: &pheno.r#type().label(),
                severity_id: pheno.severity().as_ref().map(|s| s.id()),
                severity_label: pheno.severity().as_ref().map(|s| s.label()),
                observed_start: pheno.observed_start().copied(),
                observed_end: pheno.observed_end().copied(),
            };
            phenotype_wtr.serialize(row)?;
        }
    }

    // Flush to ensure all buffers are written to disk
    diagnosis_wtr.flush()?;
    phenotype_wtr.flush()?;

    Ok(())
}

#[test]
fn test_integration_declared_version() {
    let phq9_patent_data = File::open("/Users/smart_monkey/data/phq9/phq9_questioner_data.csv")
        .expect("Could not open file");
    let phq9_questioner_data = File::open("/Users/smart_monkey/data/phq9/phq9_questioner_data.csv")
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

        let patient_id = row.get(row.len() - 1).unwrap().str_value().clone();
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

    println!("{}", questionnaire_results[9000]);

    export_to_csvs(questionnaire_results.as_slice()).expect("Failed to write CSVs");
}
