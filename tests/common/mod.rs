#![allow(dead_code)] // each test crate uses a different subset of these helpers

pub mod xpt_writer;

use chrono::{DateTime, TimeZone, Utc};

use rorschach_rs::answer::QuestionnaireResponse;
use rorschach_rs::data_loader::error::DataLoaderError;
use rorschach_rs::data_loader::questionnaire_data_loader::QuestionnaireDataLoader;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

pub fn asset(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("assets")
        .join(name)
}

pub fn cols(names: &[&str]) -> Vec<String> {
    names.iter().map(|s| s.to_string()).collect()
}

pub fn utc(y: i32, m: u32, d: u32, h: u32, min: u32, s: u32) -> DateTime<Utc> {
    Utc.with_ymd_and_hms(y, m, d, h, min, s).unwrap()
}

pub fn scores(r: &QuestionnaireResponse) -> Vec<Option<f32>> {
    r.answers().iter().map(|a| a.score()).collect()
}

pub fn all_scores(rs: &[QuestionnaireResponse]) -> Vec<Vec<Option<f32>>> {
    rs.iter().map(scores).collect()
}

pub fn dates(rs: &[QuestionnaireResponse]) -> Vec<Option<DateTime<Utc>>> {
    rs.iter().map(|r| r.taken_at().copied()).collect()
}

/// Loads `path` with the given question columns and date column.
pub fn load(
    path: impl AsRef<Path>,
    questions: &[&str],
    date: &str,
) -> Result<Vec<QuestionnaireResponse>, DataLoaderError> {
    let q = cols(questions);
    QuestionnaireDataLoader::new("test-questionnaire", &q, date).load(path)
}

/// Like `load` but panics with the error message on failure.
pub fn load_ok(
    path: impl AsRef<Path>,
    questions: &[&str],
    date: &str,
) -> Vec<QuestionnaireResponse> {
    load(path, questions, date).unwrap_or_else(|e| panic!("load failed: {e}"))
}

/// Like `load` but expects an error.
pub fn load_err(path: impl AsRef<Path>, questions: &[&str], date: &str) -> DataLoaderError {
    match load(path, questions, date) {
        Ok(r) => panic!("expected an error, got {} responses", r.len()),
        Err(e) => e,
    }
}

/// A temp dir that lives as long as the returned guard; the file is written inside it.
pub fn temp_file(name: &str, contents: &[u8]) -> (TempDir, PathBuf) {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join(name);
    std::fs::write(&path, contents).unwrap();
    (dir, path)
}
