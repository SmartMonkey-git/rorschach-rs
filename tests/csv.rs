mod common;

use common::*;
use rorschach_rs::data_loader::error::DataLoaderError;

fn csv(contents: &str) -> (tempfile::TempDir, std::path::PathBuf) {
    temp_file("data.csv", contents.as_bytes())
}

#[test]
fn comma_separated() {
    let (_d, p) = csv("Q1,Q2,Date\n1,2.5,2024-01-15\n3,4,2024-02-01T08:00:00\n");
    let r = load_ok(&p, &["Q1", "Q2"], "Date");
    assert_eq!(
        all_scores(&r),
        vec![vec![Some(1.0), Some(2.5)], vec![Some(3.0), Some(4.0)]]
    );
    assert_eq!(
        dates(&r),
        vec![
            Some(utc(2024, 1, 15, 0, 0, 0)),
            Some(utc(2024, 2, 1, 8, 0, 0))
        ]
    );
}

#[test]
fn semicolon_separated_with_german_decimals_and_dates() {
    let (_d, p) = csv("Q1;Q2;Datum\n1,5;2;15.01.2024 10:30\n0,25;3;01.02.2024\n");
    let r = load_ok(&p, &["Q1", "Q2"], "Datum");
    assert_eq!(
        all_scores(&r),
        vec![vec![Some(1.5), Some(2.0)], vec![Some(0.25), Some(3.0)]]
    );
    assert_eq!(
        dates(&r),
        vec![
            Some(utc(2024, 1, 15, 10, 30, 0)),
            Some(utc(2024, 2, 1, 0, 0, 0))
        ]
    );
}

#[test]
fn tab_separated_tsv() {
    let (_d, p) = temp_file("data.tsv", b"Q1\tDate\n2\t2024-01-15\n");
    assert_eq!(
        all_scores(&load_ok(&p, &["Q1"], "Date")),
        vec![vec![Some(2.0)]]
    );
}

#[test]
fn txt_and_uppercase_extensions() {
    for name in ["data.txt", "DATA.CSV"] {
        let (_d, p) = temp_file(name, b"Q1,Date\n2,\n");
        assert_eq!(
            all_scores(&load_ok(&p, &["Q1"], "Date")),
            vec![vec![Some(2.0)]],
            "{name}"
        );
    }
}

#[test]
fn utf8_bom_is_stripped_from_first_header() {
    let (_d, p) = temp_file("data.csv", b"\xEF\xBB\xBFQ1;Date\n1;\n");
    assert_eq!(
        all_scores(&load_ok(&p, &["Q1"], "Date")),
        vec![vec![Some(1.0)]]
    );
}

#[test]
fn windows_1252_encoded_headers() {
    // "Größe" and "Überprüfung" encoded as Windows-1252 / Latin-1 (e.g. German Excel export)
    let (_d, p) = temp_file("data.csv", b"Gr\xF6\xDFe;\xDCberpr\xFCfung\n3;15.01.2024\n");
    let r = load_ok(&p, &["Größe"], "Überprüfung");
    assert_eq!(all_scores(&r), vec![vec![Some(3.0)]]);
    assert_eq!(dates(&r), vec![Some(utc(2024, 1, 15, 0, 0, 0))]);
}

#[test]
fn utf8_headers() {
    let (_d, p) = csv("Größe,Überprüfung\n3,2024-01-15\n");
    assert_eq!(
        all_scores(&load_ok(&p, &["Größe"], "Überprüfung")),
        vec![vec![Some(3.0)]]
    );
}

#[test]
fn crlf_line_endings() {
    let (_d, p) = csv("Q1,Date\r\n1,2024-01-15\r\n2,2024-01-16\r\n");
    let r = load_ok(&p, &["Q1"], "Date");
    assert_eq!(all_scores(&r), vec![vec![Some(1.0)], vec![Some(2.0)]]);
    assert_eq!(dates(&r)[1], Some(utc(2024, 1, 16, 0, 0, 0)));
}

#[test]
fn quoted_fields_and_headers() {
    let (_d, p) = csv("\"Q1\",\"Comment, with comma\",\"Date\"\n\"4\",\"a, b\",\"2024-01-15\"\n");
    assert_eq!(
        all_scores(&load_ok(&p, &["Q1"], "Date")),
        vec![vec![Some(4.0)]]
    );
}

#[test]
fn quoted_semicolons_do_not_change_detected_delimiter() {
    let (_d, p) = csv("\"a;b;c;d;e\",Q1,Date\nx,1,\n");
    assert_eq!(
        all_scores(&load_ok(&p, &["Q1"], "Date")),
        vec![vec![Some(1.0)]]
    );
}

#[test]
fn padded_headers_and_values() {
    let (_d, p) = csv(" Q1 , Date \n 3 , 2024-01-15 \n");
    let r = load_ok(&p, &["Q1"], "Date");
    assert_eq!(all_scores(&r), vec![vec![Some(3.0)]]);
    assert_eq!(dates(&r), vec![Some(utc(2024, 1, 15, 0, 0, 0))]);
}

#[test]
fn missing_value_markers() {
    let (_d, p) = csv("Q1,Q2,Q3,Q4,Q5,Q6,Date\n,NA,N/A,.,NaN,null,NA\n");
    let r = load_ok(&p, &["Q1", "Q2", "Q3", "Q4", "Q5", "Q6"], "Date");
    assert_eq!(all_scores(&r), vec![vec![None; 6]]);
    assert_eq!(dates(&r), vec![None]);
}

#[test]
fn short_rows_are_padded_with_missing_values() {
    let (_d, p) = csv("Q1,Q2,Date\n1\n2,3,2024-01-15,extra\n");
    let r = load_ok(&p, &["Q1", "Q2"], "Date");
    assert_eq!(
        all_scores(&r),
        vec![vec![Some(1.0), None], vec![Some(2.0), Some(3.0)]]
    );
    assert_eq!(dates(&r), vec![None, Some(utc(2024, 1, 15, 0, 0, 0))]);
}

#[test]
fn blank_lines_and_delimiter_only_rows_are_skipped() {
    let (_d, p) = csv("Q1;Date\n1;\n\n;\n  ;  \n2;\n");
    assert_eq!(
        all_scores(&load_ok(&p, &["Q1"], "Date")),
        vec![vec![Some(1.0)], vec![Some(2.0)]]
    );
}

#[test]
fn header_only_yields_no_responses() {
    let (_d, p) = csv("Q1,Date\n");
    assert!(load_ok(&p, &["Q1"], "Date").is_empty());
}

#[test]
fn empty_file_is_an_error() {
    let (_d, p) = csv("");
    assert!(matches!(
        load_err(&p, &["Q1"], "Date"),
        DataLoaderError::EmptyFile(_)
    ));
}

#[test]
fn invalid_score_reports_row_column_and_value() {
    let (_d, p) = csv("Q1,Q2,Date\n1,2,\n3,often,\n");
    match load_err(&p, &["Q1", "Q2"], "Date") {
        DataLoaderError::InvalidScore {
            row, column, value, ..
        } => {
            assert_eq!((row, column.as_str(), value.as_str()), (2, "Q2", "often"));
        }
        e => panic!("unexpected error: {e}"),
    }
}

#[test]
fn invalid_score_error_message_is_readable() {
    let (_d, p) = csv("Q1,Date\nx,\n");
    let msg = load_err(&p, &["Q1"], "Date").to_string();
    assert!(
        msg.contains("test-questionnaire") && msg.contains("'x'") && msg.contains("Q1"),
        "{msg}"
    );
}

#[test]
fn ambiguous_slash_date_is_rejected() {
    let (_d, p) = csv("Q1,Date\n1,01/02/2024\n");
    match load_err(&p, &["Q1"], "Date") {
        DataLoaderError::InvalidDate {
            row, column, value, ..
        } => {
            assert_eq!(
                (row, column.as_str(), value.as_str()),
                (1, "Date", "01/02/2024")
            );
        }
        e => panic!("unexpected error: {e}"),
    }
}

#[test]
fn numeric_text_in_date_column_is_rejected() {
    // A bare number in a CSV date column could be an Excel serial, a SAS date or a
    // Unix timestamp - refusing is safer than guessing.
    let (_d, p) = csv("Q1,Date\n1,45306\n");
    assert!(matches!(
        load_err(&p, &["Q1"], "Date"),
        DataLoaderError::InvalidDate { .. }
    ));
}

#[test]
fn nonexistent_file_is_io_error() {
    assert!(matches!(
        load_err("/definitely/not/here.csv", &["Q1"], "Date"),
        DataLoaderError::Io(_)
    ));
}
