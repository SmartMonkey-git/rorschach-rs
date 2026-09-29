//! Behaviour that is independent of the file format.

mod common;

use common::*;
use rorschach_rs::data_loader::data_file_type::DataFileType;
use rorschach_rs::data_loader::error::DataLoaderError;
use rorschach_rs::data_loader::questionnaire_data_loader::QuestionnaireDataLoader;
use std::path::Path;

#[test]
fn file_type_from_extension() {
    let cases = [
        ("a.xlsx", Some(DataFileType::Excel)),
        ("a.XLSX", Some(DataFileType::Excel)),
        ("a.xlsm", Some(DataFileType::Excel)),
        ("a.xlsb", Some(DataFileType::Excel)),
        ("a.xls", Some(DataFileType::Excel)),
        ("a.ods", Some(DataFileType::Excel)),
        ("a.csv", Some(DataFileType::CSV)),
        ("a.tsv", Some(DataFileType::CSV)),
        ("a.txt", Some(DataFileType::CSV)),
        ("a.xpt", Some(DataFileType::XPT)),
        ("dir.v2/a.XPT", Some(DataFileType::XPT)),
        ("a.json", None),
        ("a.sas7bdat", None),
        ("xpt", None),
        ("a", None),
        ("", None),
    ];
    for (p, expected) in cases {
        assert_eq!(DataFileType::from_path(Path::new(p)), expected, "{p}");
    }
}

#[test]
fn unsupported_extension_is_rejected_before_reading() {
    // The file doesn't exist; we must get UnsupportedFileType, not an I/O error.
    for p in ["/nope/data.json", "/nope/data", "/nope/data.sas7bdat"] {
        match load_err(p, &["Q1"], "Date") {
            DataLoaderError::UnsupportedFileType(path) => assert_eq!(path, Path::new(p)),
            e => panic!("{p}: unexpected error {e}"),
        }
    }
}

#[test]
fn accepts_pathbuf_str_and_path() {
    let (_d, p) = temp_file("d.csv", b"Q1,Date\n1,\n");
    let q = cols(&["Q1"]);
    let loader = QuestionnaireDataLoader::new("x", &q, "Date");
    assert_eq!(loader.load(p.clone()).unwrap().len(), 1);
    assert_eq!(loader.load(&p).unwrap().len(), 1);
    assert_eq!(loader.load(p.to_str().unwrap()).unwrap().len(), 1);
}

#[test]
fn loader_is_reusable_across_files_and_formats() {
    let q = cols(&["Q1", "Q2", "Q3"]);
    let loader = QuestionnaireDataLoader::new("x", &q, "VISITDT");
    let (_d, csv) = temp_file("d.csv", b"Q1,Q2,Q3,VISITDT\n1,2,3,2024-01-15\n");
    assert_eq!(loader.load(&csv).unwrap().len(), 1);
    assert_eq!(loader.load(asset("q_v5.xpt")).unwrap().len(), 3);
    assert_eq!(loader.load(&csv).unwrap().len(), 1);
}

#[test]
fn missing_columns_are_all_reported_with_context() {
    let (_d, p) = temp_file("d.csv", b"Q1,Date\n1,\n");
    match load_err(&p, &["Q1", "Q2", "Q3"], "When") {
        DataLoaderError::MissingColumns {
            questionnaire,
            file,
            columns,
        } => {
            assert_eq!(questionnaire, "test-questionnaire");
            assert_eq!(file, p);
            assert_eq!(columns, vec!["Q2", "Q3", "When"]);
        }
        e => panic!("unexpected error {e}"),
    }
}

#[test]
fn missing_columns_are_checked_before_rows() {
    // Row 1 has an invalid score, but the missing column must be reported first.
    let (_d, p) = temp_file("d.csv", b"Q1,Date\nbad,\n");
    assert!(matches!(
        load_err(&p, &["Q1", "Q2"], "Date"),
        DataLoaderError::MissingColumns { .. }
    ));
}

#[test]
fn ambiguous_case_insensitive_header_is_missing() {
    let (_d, p) = temp_file("d.csv", b"Score,SCORE,Date\n1,2,\n");
    assert!(matches!(
        load_err(&p, &["score"], "Date"),
        DataLoaderError::MissingColumns { .. }
    ));
    // An exact match is still unambiguous.
    assert_eq!(
        all_scores(&load_ok(&p, &["SCORE"], "Date")),
        vec![vec![Some(2.0)]]
    );
}

#[test]
fn no_question_columns_gives_empty_answers() {
    let (_d, p) = temp_file("d.csv", b"Q1,Date\n1,2024-01-15\n");
    let r = load_ok(&p, &[], "Date");
    assert_eq!(r.len(), 1);
    assert!(r[0].answers().is_empty());
    assert_eq!(dates(&r), vec![Some(utc(2024, 1, 15, 0, 0, 0))]);
}

#[test]
fn same_column_twice_yields_two_answers() {
    let (_d, p) = temp_file("d.csv", b"Q1,Date\n4,\n");
    let r = load_ok(&p, &["Q1", "Q1"], "Date");
    assert_eq!(
        r[0].answers()
            .iter()
            .map(|a| (a.idx(), a.score()))
            .collect::<Vec<_>>(),
        vec![(0, Some(4.0)), (1, Some(4.0))]
    );
}

#[test]
fn date_column_may_double_as_question_column() {
    let (_d, p) = temp_file("d.csv", b"Q1\n1\n");
    // Q1 is a valid score but "1" is not a valid CSV date -> InvalidDate.
    assert!(matches!(
        load_err(&p, &["Q1"], "Q1"),
        DataLoaderError::InvalidDate { .. }
    ));
}

#[test]
fn same_data_gives_same_result_in_every_format() {
    let (_d1, csv) = temp_file(
        "d.csv",
        b"Q1,Q2,Q3,VISITDT\n1,2.5,3,2024-01-15\n,4,5,2024-02-01\n2,1,0,\n",
    );
    let from_csv = load_ok(&csv, QS, "VISITDT");
    let from_xpt = load_ok(asset("q_v5.xpt"), QS, "VISITDT");
    assert_eq!(all_scores(&from_csv), all_scores(&from_xpt));
    assert_eq!(dates(&from_csv), dates(&from_xpt));
}

const QS: &[&str] = &["Q1", "Q2", "Q3"];
