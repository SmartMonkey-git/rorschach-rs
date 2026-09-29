mod common;

use common::xpt_writer::{self, Dataset, Value::*, Var};
use common::*;
use rorschach_rs::data_loader::error::DataLoaderError;

const QS: &[&str] = &["Q1", "Q2", "Q3"];

fn expected_scores() -> Vec<Vec<Option<f32>>> {
    vec![
        vec![Some(1.0), Some(2.5), Some(3.0)],
        vec![None, Some(4.0), Some(5.0)],
        vec![Some(2.0), Some(1.0), Some(0.0)],
    ]
}

// --- Fixtures written by ReadStat (independent implementation) ---------------

#[test]
fn v5_numeric_date_with_date9_format() {
    let r = load_ok(asset("q_v5.xpt"), QS, "VISITDT");
    assert_eq!(all_scores(&r), expected_scores());
    assert_eq!(
        dates(&r),
        vec![
            Some(utc(2024, 1, 15, 0, 0, 0)),
            Some(utc(2024, 2, 1, 0, 0, 0)),
            None
        ]
    );
}

#[test]
fn v5_numeric_datetime_with_datetime20_format() {
    let r = load_ok(asset("q_v5.xpt"), QS, "VISITDTM");
    assert_eq!(
        dates(&r),
        vec![
            Some(utc(2024, 1, 15, 10, 30, 0)),
            Some(utc(2024, 2, 1, 0, 0, 0)),
            None
        ]
    );
}

#[test]
fn v5_character_iso8601_date() {
    let r = load_ok(asset("q_v5.xpt"), QS, "CHARDT");
    assert_eq!(
        dates(&r),
        vec![
            Some(utc(2024, 1, 15, 10, 30, 0)),
            Some(utc(2024, 2, 1, 0, 0, 0)),
            None
        ]
    );
}

#[test]
fn v5_names_match_case_insensitively() {
    let r = load_ok(asset("q_v5.xpt"), &["q1", "q2", "q3"], "visitdt");
    assert_eq!(all_scores(&r), expected_scores());
}

#[test]
fn v8_long_variable_names() {
    let r = load_ok(
        asset("q_v8.xpt"),
        &["QUESTION_NUMBER_ONE_LONG", "Q2", "Q3"],
        "VISITDTM",
    );
    assert_eq!(all_scores(&r), expected_scores());
    assert_eq!(dates(&r)[0], Some(utc(2024, 1, 15, 10, 30, 0)));
}

#[test]
fn v8_with_long_label_records() {
    let r = load_ok(asset("q_v8_long_labels.xpt"), QS, "VISITDT");
    assert_eq!(all_scores(&r), expected_scores());
    assert_eq!(dates(&r)[1], Some(utc(2024, 2, 1, 0, 0, 0)));
}

#[test]
fn rows_shorter_than_record_padding_are_not_duplicated() {
    // 8-byte rows: the blank padding of the last 80-byte record must not become rows.
    let r = load_ok(asset("one_col.xpt"), &["Q1"], "Q1");
    assert_eq!(
        all_scores(&r),
        vec![vec![Some(1.0)], vec![Some(2.0)], vec![Some(3.0)]]
    );
}

#[test]
fn truncated_files_never_panic() {
    for name in [
        "q_v5.xpt",
        "q_v8.xpt",
        "q_v8_long_labels.xpt",
        "one_col.xpt",
    ] {
        let bytes = std::fs::read(asset(name)).unwrap();
        for len in 0..bytes.len() {
            let (_d, p) = temp_file("cut.xpt", &bytes[..len]);
            // Ok with fewer rows or an error are both fine; a panic is not.
            let _ = load(&p, &["Q1"], "Q1");
        }
    }
}

#[test]
fn corrupted_bytes_never_panic() {
    let bytes = std::fs::read(asset("q_v5.xpt")).unwrap();
    for i in (0..bytes.len()).step_by(3) {
        for v in [0x00, 0xFF, b' ', 0x7F] {
            let mut b = bytes.clone();
            b[i] = v;
            let (_d, p) = temp_file("bad.xpt", &b);
            let _ = load(&p, &["Q1"], "Q1");
        }
    }
}

// --- Files from the in-test writer (edge cases) --------------------------------

fn xpt(datasets: &[Dataset]) -> (tempfile::TempDir, std::path::PathBuf) {
    temp_file("data.xpt", &xpt_writer::write(datasets))
}

#[test]
fn writer_round_trips_basic_values() {
    let (_d, p) = xpt(&[Dataset {
        name: "QS",
        vars: vec![Var::num("Q1"), Var::num("Q2"), Var::num_fmt("DT", "DATE9")],
        rows: vec![
            vec![Num(0.0), Num(-1.5), Num(23390.0)],
            vec![Num(0.1), Num(123456.75), Num(0.0)],
        ],
    }]);
    let r = load_ok(&p, &["Q1", "Q2"], "DT");
    assert_eq!(
        all_scores(&r),
        vec![
            vec![Some(0.0), Some(-1.5)],
            vec![Some(0.1), Some(123456.75)]
        ]
    );
    assert_eq!(
        dates(&r),
        vec![
            Some(utc(2024, 1, 15, 0, 0, 0)),
            Some(utc(1960, 1, 1, 0, 0, 0))
        ]
    );
}

#[test]
fn all_missing_observation_is_kept_not_skipped() {
    // Unlike a blank spreadsheet row, an XPT observation is always a real record
    // (e.g. a participant who answered nothing).
    let (_d, p) = xpt(&[Dataset {
        name: "QS",
        vars: vec![Var::num("Q1"), Var::num("DT")],
        rows: vec![
            vec![Num(1.0), Missing(b'.')],
            vec![Missing(b'.'), Missing(b'.')],
            vec![Num(2.0), Missing(b'.')],
        ],
    }]);
    assert_eq!(
        all_scores(&load_ok(&p, &["Q1"], "DT")),
        vec![vec![Some(1.0)], vec![Some(2.0)]]
    );
}

#[test]
fn special_missing_values_are_none() {
    let (_d, p) = xpt(&[Dataset {
        name: "QS",
        vars: vec![
            Var::num("Q1"),
            Var::num("Q2"),
            Var::num("Q3"),
            Var::num("Q4"),
            Var::num_fmt("DT", "DATE9"),
        ],
        rows: vec![vec![
            Missing(b'.'),
            Missing(b'_'),
            Missing(b'A'),
            Missing(b'Z'),
            Missing(b'.'),
        ]],
    }]);
    let r = load_ok(&p, &["Q1", "Q2", "Q3", "Q4"], "DT");

    let expected: Vec<Vec<Option<f32>>> = vec![];
    assert_eq!(all_scores(&r), expected);
    assert_eq!(dates(&r), vec![]);
}

#[test]
fn short_numeric_lengths() {
    let (_d, p) = xpt(&[Dataset {
        name: "QS",
        vars: vec![
            Var::num_len("Q1", 3),
            Var::num_len("Q2", 4),
            Var::num_len("Q3", 3),
            Var::num("DT"),
        ],
        rows: vec![
            vec![Num(3.0), Num(100.0), Missing(b'.'), Missing(b'.')],
            vec![Num(4.0), Num(2.5), Num(1.0), Missing(b'.')],
        ],
    }]);
    let r = load_ok(&p, &["Q1", "Q2", "Q3"], "DT");
    assert_eq!(
        all_scores(&r),
        vec![
            vec![Some(3.0), Some(100.0), None],
            vec![Some(4.0), Some(2.5), Some(1.0)]
        ]
    );
}

#[test]
fn character_score_columns() {
    let (_d, p) = xpt(&[Dataset {
        name: "QS",
        vars: vec![Var::chr("QSORRES", 8), Var::chr("QSDTC", 19)],
        rows: vec![
            vec![Str("3"), Str("2024-01-15T10:30:00")],
            vec![Str(""), Str("2024-01-15")],
            vec![Str("NA"), Str("")],
        ],
    }]);
    let r = load_ok(&p, &["QSORRES"], "QSDTC");
    assert_eq!(
        all_scores(&r),
        vec![vec![Some(3.0)], vec![None], vec![None]]
    );
    assert_eq!(
        dates(&r),
        vec![
            Some(utc(2024, 1, 15, 10, 30, 0)),
            Some(utc(2024, 1, 15, 0, 0, 0)),
            None
        ]
    );
}

#[test]
fn unformatted_numeric_dates_use_magnitude_heuristic() {
    let (_d, p) = xpt(&[Dataset {
        name: "QS",
        vars: vec![Var::num("Q1"), Var::num("DT")],
        rows: vec![
            vec![Num(1.0), Num(23390.0)],                       // days
            vec![Num(1.0), Num(23390.0 * 86_400.0 + 37_800.0)], // seconds
        ],
    }]);
    assert_eq!(
        dates(&load_ok(&p, &["Q1"], "DT")),
        vec![
            Some(utc(2024, 1, 15, 0, 0, 0)),
            Some(utc(2024, 1, 15, 10, 30, 0))
        ]
    );
}

#[test]
fn only_first_dataset_is_read() {
    let ds = |name, values: &[f64]| Dataset {
        name,
        vars: vec![Var::num("Q1"), Var::num("DT")],
        rows: values
            .iter()
            .map(|&v| vec![Num(v), Missing(b'.')])
            .collect(),
    };
    let (_d, p) = xpt(&[ds("FIRST", &[1.0, 2.0]), ds("SECOND", &[7.0, 8.0, 9.0])]);
    assert_eq!(
        all_scores(&load_ok(&p, &["Q1"], "DT")),
        vec![vec![Some(1.0)], vec![Some(2.0)]]
    );
}

#[test]
fn dataset_without_observations() {
    let (_d, p) = xpt(&[Dataset {
        name: "QS",
        vars: vec![Var::num("Q1"), Var::num("DT")],
        rows: vec![],
    }]);
    assert!(load_ok(&p, &["Q1"], "DT").is_empty());
}

#[test]
fn many_short_rows_across_record_boundaries() {
    // 3-byte rows straddle the 80-byte record boundaries.
    let values: Vec<f64> = (0..100).map(f64::from).collect();
    let (_d, p) = xpt(&[Dataset {
        name: "QS",
        vars: vec![Var::num_len("Q1", 3)],
        rows: values.iter().map(|&v| vec![Num(v)]).collect(),
    }]);
    let r = load_ok(&p, &["Q1"], "Q1");
    assert_eq!(r.len(), 100);
    assert_eq!(scores(&r[99]), vec![Some(99.0)]);
}

#[test]
fn non_xpt_content_is_rejected() {
    let (_d, p) = temp_file("data.xpt", b"Q1,Date\n1,2024-01-15\n");
    assert!(matches!(
        load_err(&p, &["Q1"], "Date"),
        DataLoaderError::Xpt(_)
    ));
}

#[test]
fn nonexistent_file_is_io_error() {
    assert!(matches!(
        load_err("/definitely/not/here.xpt", &["Q1"], "D"),
        DataLoaderError::Io(_)
    ));
}
