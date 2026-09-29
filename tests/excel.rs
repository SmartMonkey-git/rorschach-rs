mod common;

use common::*;
use rorschach_rs::data_loader::error::DataLoaderError;
use rust_xlsxwriter::{ExcelDateTime, Format, Formula, Workbook, Worksheet};
use tempfile::TempDir;

/// Builds an .xlsx in a temp dir. `fill` receives the first worksheet.
fn xlsx(fill: impl FnOnce(&mut Worksheet)) -> (TempDir, std::path::PathBuf) {
    xlsx_with(|wb| fill(wb.add_worksheet()))
}

fn xlsx_with(fill: impl FnOnce(&mut Workbook)) -> (TempDir, std::path::PathBuf) {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("data.xlsx");
    let mut wb = Workbook::new();
    fill(&mut wb);
    wb.save(&path).unwrap();
    (dir, path)
}

fn header(ws: &mut Worksheet, names: &[&str]) {
    for (c, n) in names.iter().enumerate() {
        ws.write_string(0, c as u16, *n).unwrap();
    }
}

fn datetime(ws: &mut Worksheet, row: u32, col: u16, (y, m, d, h, min): (u16, u8, u8, u16, u8)) {
    let dt = ExcelDateTime::from_ymd(y, m, d)
        .unwrap()
        .and_hms(h, min, 0)
        .unwrap();
    let fmt = Format::new().set_num_format("yyyy-mm-dd hh:mm");
    ws.write_datetime_with_format(row, col, &dt, &fmt).unwrap();
}

#[test]
fn loads_scores_and_datetime_cells() {
    let (_dir, path) = xlsx(|ws| {
        header(ws, &["Q1", "Q2", "Q3", "Datum"]);
        ws.write_number(1, 0, 1).unwrap();
        ws.write_number(1, 1, 2.5).unwrap();
        ws.write_number(1, 2, 3).unwrap();
        datetime(ws, 1, 3, (2024, 1, 15, 10, 30));
        // row 2: Q1 blank, Q3 stored as text
        ws.write_number(2, 1, 4).unwrap();
        ws.write_string(2, 2, "5").unwrap();
        datetime(ws, 2, 3, (2024, 2, 1, 0, 0));
    });

    let r = load_ok(&path, &["Q1", "Q2", "Q3"], "Datum");
    assert_eq!(
        all_scores(&r),
        vec![
            vec![Some(1.0), Some(2.5), Some(3.0)],
            vec![None, Some(4.0), Some(5.0)]
        ]
    );
    assert_eq!(
        dates(&r),
        vec![
            Some(utc(2024, 1, 15, 10, 30, 0)),
            Some(utc(2024, 2, 1, 0, 0, 0))
        ]
    );
    assert_eq!(
        r[0].answers().iter().map(|a| a.idx()).collect::<Vec<_>>(),
        vec![0, 1, 2]
    );
}

#[test]
fn answer_order_follows_configuration_not_file() {
    let (_dir, path) = xlsx(|ws| {
        header(ws, &["Date", "Q1", "Q2", "Q3", "Unused"]);
        for (c, v) in [1.0, 2.0, 3.0, 99.0].iter().enumerate() {
            ws.write_number(1, c as u16 + 1, *v).unwrap();
        }
    });
    let r = load_ok(&path, &["Q3", "Q1"], "Date");
    assert_eq!(scores(&r[0]), vec![Some(3.0), Some(1.0)]);
    assert_eq!(
        r[0].answers().iter().map(|a| a.idx()).collect::<Vec<_>>(),
        vec![0, 1]
    );
}

#[test]
fn date_column_as_unformatted_serial_number() {
    let (_dir, path) = xlsx(|ws| {
        header(ws, &["Q1", "Date"]);
        ws.write_number(1, 0, 1).unwrap();
        ws.write_number(1, 1, 45306.4375).unwrap();
    });
    assert_eq!(
        dates(&load_ok(&path, &["Q1"], "Date")),
        vec![Some(utc(2024, 1, 15, 10, 30, 0))]
    );
}

#[test]
fn date_column_as_text() {
    let (_dir, path) = xlsx(|ws| {
        header(ws, &["Q1", "Date"]);
        for (r, s) in ["15.01.2024", "2024-01-15T10:30:00", "", "NA"]
            .iter()
            .enumerate()
        {
            ws.write_number(r as u32 + 1, 0, 1).unwrap();
            ws.write_string(r as u32 + 1, 1, *s).unwrap();
        }
    });
    assert_eq!(
        dates(&load_ok(&path, &["Q1"], "Date")),
        vec![
            Some(utc(2024, 1, 15, 0, 0, 0)),
            Some(utc(2024, 1, 15, 10, 30, 0)),
            None,
            None
        ]
    );
}

#[test]
fn skips_blank_rows_but_keeps_rows_with_only_missing_markers() {
    let (_dir, path) = xlsx(|ws| {
        header(ws, &["Q1", "Date"]);
        ws.write_number(1, 0, 1).unwrap();
        // row 2 left blank
        ws.write_string(3, 0, "NA").unwrap();
        // rows 4..9 blank, then a value far below
        ws.write_number(10, 0, 2).unwrap();
    });
    let r = load_ok(&path, &["Q1"], "Date");
    assert_eq!(
        all_scores(&r),
        vec![vec![Some(1.0)], vec![None], vec![Some(2.0)]]
    );
    assert_eq!(dates(&r), vec![None, None, None]);
}

#[test]
fn ignores_rows_with_content_only_in_unrelated_columns_as_non_blank() {
    // A row with data only in an unrelated column is not blank -> all answers None.
    let (_dir, path) = xlsx(|ws| {
        header(ws, &["Q1", "Date", "Comment"]);
        ws.write_string(1, 2, "patient refused").unwrap();
    });
    let r = load_ok(&path, &["Q1"], "Date");
    assert_eq!(all_scores(&r), vec![vec![None]]);
}

#[test]
fn only_the_first_sheet_is_read() {
    let (_dir, path) = xlsx_with(|wb| {
        let first = wb.add_worksheet();
        header(first, &["Q1", "Date"]);
        first.write_number(1, 0, 1).unwrap();
        let second = wb.add_worksheet();
        header(second, &["Q1", "Date"]);
        second.write_number(1, 0, 7).unwrap();
        second.write_number(2, 0, 8).unwrap();
    });
    assert_eq!(
        all_scores(&load_ok(&path, &["Q1"], "Date")),
        vec![vec![Some(1.0)]]
    );
}

#[test]
fn header_only_yields_no_responses() {
    let (_dir, path) = xlsx(|ws| header(ws, &["Q1", "Date"]));
    assert!(load_ok(&path, &["Q1"], "Date").is_empty());
}

#[test]
fn numeric_and_padded_headers_are_matched() {
    let (_dir, path) = xlsx(|ws| {
        ws.write_number(0, 0, 1).unwrap(); // header "1"
        ws.write_string(0, 1, "  Date  ").unwrap();
        ws.write_number(1, 0, 3).unwrap();
    });
    assert_eq!(
        all_scores(&load_ok(&path, &["1"], "Date")),
        vec![vec![Some(3.0)]]
    );
}

#[test]
fn formula_error_cells_are_missing() {
    let (_dir, path) = xlsx(|ws| {
        header(ws, &["Q1", "Q2", "Date"]);
        ws.write_formula(1, 0, Formula::new("=NA()").set_result("#N/A"))
            .unwrap();
        ws.write_number(1, 1, 2).unwrap();
        // A row whose only content is an error cell counts as blank.
        ws.write_formula(2, 0, Formula::new("=NA()").set_result("#N/A"))
            .unwrap();
    });
    assert_eq!(
        all_scores(&load_ok(&path, &["Q1", "Q2"], "Date")),
        vec![vec![None, Some(2.0)]]
    );
}

#[test]
fn empty_workbook_is_an_error() {
    let (_dir, path) = xlsx(|_| {});
    assert!(matches!(
        load_err(&path, &["Q1"], "Date"),
        DataLoaderError::EmptyFile(_)
    ));
}

#[test]
fn invalid_score_reports_row_and_column() {
    let (_dir, path) = xlsx(|ws| {
        header(ws, &["Q1", "Q2", "Date"]);
        ws.write_number(1, 0, 1).unwrap();
        ws.write_number(2, 0, 1).unwrap();
        ws.write_string(2, 1, "sometimes").unwrap();
    });
    match load_err(&path, &["Q1", "Q2"], "Date") {
        DataLoaderError::InvalidScore {
            questionnaire,
            row,
            column,
            value,
        } => {
            assert_eq!(
                (questionnaire.as_str(), row, column.as_str(), value.as_str()),
                ("test-questionnaire", 2, "Q2", "sometimes")
            );
        }
        e => panic!("unexpected error: {e}"),
    }
}

#[test]
fn boolean_and_datetime_cells_are_invalid_scores() {
    let (_dir, path) = xlsx(|ws| {
        header(ws, &["Q1", "Date"]);
        ws.write_boolean(1, 0, true).unwrap();
    });
    assert!(matches!(
        load_err(&path, &["Q1"], "Date"),
        DataLoaderError::InvalidScore { .. }
    ));

    let (_dir, path) = xlsx(|ws| {
        header(ws, &["Q1", "Date"]);
        datetime(ws, 1, 0, (2024, 1, 15, 0, 0));
    });
    assert!(matches!(
        load_err(&path, &["Q1"], "Date"),
        DataLoaderError::InvalidScore { .. }
    ));
}

#[test]
fn invalid_date_reports_value() {
    let (_dir, path) = xlsx(|ws| {
        header(ws, &["Q1", "Date"]);
        ws.write_string(1, 1, "01/15/2024").unwrap();
    });
    match load_err(&path, &["Q1"], "Date") {
        DataLoaderError::InvalidDate {
            row, column, value, ..
        } => {
            assert_eq!(
                (row, column.as_str(), value.as_str()),
                (1, "Date", "01/15/2024")
            );
        }
        e => panic!("unexpected error: {e}"),
    }
}

#[test]
fn corrupt_file_is_an_excel_error() {
    let (_dir, path) = temp_file("broken.xlsx", b"this is not a zip archive");
    assert!(matches!(
        load_err(&path, &["Q1"], "Date"),
        DataLoaderError::Excel(_)
    ));
}

#[test]
fn nonexistent_file_is_an_error() {
    let err = load_err("/definitely/not/here.xlsx", &["Q1"], "Date");
    assert!(
        matches!(err, DataLoaderError::Excel(_) | DataLoaderError::Io(_)),
        "{err}"
    );
}
