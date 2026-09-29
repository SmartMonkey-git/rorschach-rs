use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum DataLoaderError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("unsupported data file type: {0}")]
    UnsupportedFileType(PathBuf),

    #[error("failed to read Excel file: {0}")]
    Excel(#[from] calamine::Error),

    #[error("failed to read CSV file: {0}")]
    Csv(#[from] csv::Error),

    #[error("malformed XPT file: {0}")]
    Xpt(String),

    #[error("file {0} contains no sheet or no header row")]
    EmptyFile(PathBuf),

    #[error("questionnaire '{questionnaire}': columns not found in {file}: {columns:?}")]
    MissingColumns {
        questionnaire: String,
        file: PathBuf,
        columns: Vec<String>,
    },

    #[error(
        "questionnaire '{questionnaire}': invalid score '{value}' in column '{column}', data row {row}"
    )]
    InvalidScore {
        questionnaire: String,
        row: usize,
        column: String,
        value: String,
    },

    #[error(
        "questionnaire '{questionnaire}': invalid date '{value}' in column '{column}', data row {row}"
    )]
    InvalidDate {
        questionnaire: String,
        row: usize,
        column: String,
        value: String,
    },
}
