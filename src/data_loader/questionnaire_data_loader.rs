use crate::answer::{Answer, QuestionnaireResponse};
use crate::data_loader::data_file_type::DataFileType;
use crate::data_loader::error::DataLoaderError;
use calamine::{Data, Reader, open_workbook_auto};
use chrono::{DateTime, NaiveDate, NaiveDateTime, TimeDelta, Utc};
use std::path::Path;

pub struct QuestionnaireDataLoader {
    name: String,
    question_answer_col_names: Vec<String>,
    date_col_name: String,
}

impl QuestionnaireDataLoader {
    pub fn new(
        name: impl Into<String>,
        question_answer_col_names: impl IntoIterator<Item = impl Into<String>>,
        date_col_name: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            question_answer_col_names: question_answer_col_names
                .into_iter()
                .map(Into::into)
                .collect(),
            date_col_name: date_col_name.into(),
        }
    }

    /// Loads one `QuestionnaireResponse` per data row.
    ///
    /// * Excel: first worksheet, first row is the header.
    /// * CSV: first line is the header; `,` `;` or tab delimiter is detected.
    /// * XPT: SAS transport v5 or v8, first member (dataset) in the file.
    ///
    /// `Answer::idx` is the position of the column in `question_answer_col_names`.
    /// Empty cells (and `.`, `NA`, `N/A`, `NaN`) become `None`. Rows that are
    /// completely empty are skipped.
    pub fn load(
        &self,
        file_path: impl AsRef<Path>,
    ) -> Result<Vec<QuestionnaireResponse>, DataLoaderError> {
        let path = file_path.as_ref();
        let file_type = DataFileType::from_path(path)
            .ok_or_else(|| DataLoaderError::UnsupportedFileType(path.to_path_buf()))?;

        let table = match file_type {
            DataFileType::Excel => read_excel(path)?,
            DataFileType::CSV => read_csv(path)?,
            DataFileType::XPT => xpt::read(&std::fs::read(path)?)?,
        };

        let (question_idxs, date_idx) = self.resolve_columns(&table.headers, path)?;

        let mut responses = Vec::with_capacity(table.rows.len());
        for (row_idx, row) in table.rows.iter().enumerate() {
            if row.iter().all(Cell::is_empty) {
                continue;
            }
            let row_no = row_idx + 1;
            let cell = |i: usize| row.get(i).unwrap_or(&Cell::Empty);

            let answers = question_idxs
                .iter()
                .enumerate()
                .map(|(answer_idx, &col)| {
                    let score =
                        cell(col)
                            .to_score()
                            .ok_or_else(|| DataLoaderError::InvalidScore {
                                questionnaire: self.name.to_string(),
                                row: row_no,
                                column: table.headers[col].clone(),
                                value: cell(col).to_string(),
                            })?;
                    Ok(Answer::new(answer_idx, score))
                })
                .collect::<Result<Vec<_>, DataLoaderError>>()?;

            let taken_at = cell(date_idx).to_datetime(file_type).ok_or_else(|| {
                DataLoaderError::InvalidDate {
                    questionnaire: self.name.to_string(),
                    row: row_no,
                    column: table.headers[date_idx].clone(),
                    value: cell(date_idx).to_string(),
                }
            })?;

            responses.push(QuestionnaireResponse::new(answers, taken_at));
        }
        Ok(responses)
    }

    /// Maps configured column names to column indices. Exact match first, then a
    /// case-insensitive match (XPT v5 variable names are always upper case).
    fn resolve_columns(
        &self,
        headers: &[String],
        path: &Path,
    ) -> Result<(Vec<usize>, usize), DataLoaderError> {
        let find = |name: &str| {
            let name = name.trim();
            headers.iter().position(|h| h == name).or_else(|| {
                let mut hits = headers
                    .iter()
                    .enumerate()
                    .filter(|(_, h)| h.eq_ignore_ascii_case(name));
                match (hits.next(), hits.next()) {
                    (Some((i, _)), None) => Some(i),
                    _ => None,
                }
            })
        };

        let mut missing = Vec::new();
        let question_idxs: Vec<usize> = self
            .question_answer_col_names
            .iter()
            .filter_map(|name| {
                find(name).or_else(|| {
                    missing.push(name.clone());
                    None
                })
            })
            .collect();
        let date_idx = find(self.date_col_name.as_str());
        if date_idx.is_none() {
            missing.push(self.date_col_name.to_string());
        }

        match date_idx {
            Some(date_idx) if missing.is_empty() => Ok((question_idxs, date_idx)),
            _ => Err(DataLoaderError::MissingColumns {
                questionnaire: self.name.to_string(),
                file: path.to_path_buf(),
                columns: missing,
            }),
        }
    }
}

struct RawTable {
    headers: Vec<String>,
    rows: Vec<Vec<Cell>>,
}

#[derive(Debug, Clone, PartialEq)]
enum Cell {
    Empty,
    Number(f64),
    Text(String),
    DateTime(NaiveDateTime),
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Empty => Ok(()),
            Cell::Number(n) => write!(f, "{n}"),
            Cell::Text(s) => write!(f, "{s}"),
            Cell::DateTime(dt) => write!(f, "{dt}"),
        }
    }
}

const MISSING_TOKENS: [&str; 6] = ["", ".", "na", "n/a", "nan", "null"];

fn is_missing_token(s: &str) -> bool {
    let s = s.trim();
    MISSING_TOKENS.iter().any(|t| s.eq_ignore_ascii_case(t))
}

impl Cell {
    fn is_empty(&self) -> bool {
        match self {
            Cell::Empty => true,
            Cell::Number(n) => n.is_nan(),
            Cell::Text(s) => s.trim().is_empty(),
            Cell::DateTime(_) => false,
        }
    }

    /// `Some(None)` = missing answer, `None` = value is not a valid score.
    fn to_score(&self) -> Option<Option<f32>> {
        match self {
            Cell::Empty => Some(None),
            Cell::Number(n) if n.is_nan() => Some(None),
            Cell::Number(n) => Some(Some(*n as f32)),
            Cell::Text(s) if is_missing_token(s) => Some(None),
            Cell::Text(s) => {
                let s = s.trim();
                // Accept a decimal comma ("2,5") as used in German locales.
                let parsed = s.parse::<f32>().ok().or_else(|| {
                    (!s.contains('.'))
                        .then(|| s.replace(',', ".").parse::<f32>().ok())
                        .flatten()
                });
                parsed.filter(|v| v.is_finite()).map(Some)
            }
            Cell::DateTime(_) => None,
        }
    }

    /// `Some(None)` = no date, `None` = value is not a valid date.
    fn to_datetime(&self, source: DataFileType) -> Option<Option<DateTime<Utc>>> {
        let naive = match self {
            Cell::Empty => return Some(None),
            Cell::Number(n) if n.is_nan() => return Some(None),
            Cell::Text(s) if is_missing_token(s) => return Some(None),
            Cell::DateTime(dt) => *dt,
            Cell::Text(s) => return parse_date_text(s.trim()).map(Some),
            Cell::Number(n) => match source {
                DataFileType::Excel => excel_serial_to_datetime(*n)?,
                DataFileType::XPT => sas_number_to_datetime(*n, None)?,
                DataFileType::CSV => return None,
            },
        };
        Some(Some(naive.and_utc()))
    }
}

/// Parses common textual date formats. Values without a time zone are taken as UTC.
fn parse_date_text(s: &str) -> Option<DateTime<Utc>> {
    if let Ok(dt) = DateTime::parse_from_rfc3339(s) {
        return Some(dt.with_timezone(&Utc));
    }
    const DATETIME_FORMATS: [&str; 8] = [
        "%Y-%m-%dT%H:%M:%S%.f",
        "%Y-%m-%d %H:%M:%S%.f",
        "%Y-%m-%dT%H:%M",
        "%Y-%m-%d %H:%M",
        "%d.%m.%Y %H:%M:%S",
        "%d.%m.%Y %H:%M",
        "%d%b%Y:%H:%M:%S", // SAS DATETIME, e.g. 01JAN2024:13:45:00
        "%d%b%Y %H:%M:%S",
    ];
    const DATE_FORMATS: [&str; 4] = [
        "%Y-%m-%d", "%d.%m.%Y", "%d%b%Y", // SAS DATE9, e.g. 01JAN2024
        "%Y%m%d",
    ];
    DATETIME_FORMATS
        .iter()
        .find_map(|f| NaiveDateTime::parse_from_str(s, f).ok())
        .or_else(|| {
            DATE_FORMATS
                .iter()
                .find_map(|f| NaiveDate::parse_from_str(s, f).ok())
                .and_then(|d| d.and_hms_opt(0, 0, 0))
        })
        .map(|dt| dt.and_utc())
}

fn excel_serial_to_datetime(serial: f64) -> Option<NaiveDateTime> {
    // Excel's day 0 is 1899-12-30 once the 1900 leap-year bug is accounted for.
    let epoch = NaiveDate::from_ymd_opt(1899, 12, 30)?.and_hms_opt(0, 0, 0)?;
    let millis = (serial * 86_400_000.0).round();
    if !(0.0..=1e15).contains(&millis) {
        return None;
    }
    epoch.checked_add_signed(TimeDelta::milliseconds(millis as i64))
}

/// Whether a SAS numeric holds days or seconds since 1960-01-01.
#[derive(Debug, Clone, Copy, PartialEq)]
enum SasDateKind {
    Date,
    DateTime,
}

fn sas_number_to_datetime(value: f64, kind: Option<SasDateKind>) -> Option<NaiveDateTime> {
    // Without a format, assume days if it is a plausible day count (< year 2233),
    // otherwise seconds; any datetime after 1960-01-02 exceeds this threshold.
    let kind = kind.unwrap_or(if value.abs() < 100_000.0 {
        SasDateKind::Date
    } else {
        SasDateKind::DateTime
    });
    let epoch = NaiveDate::from_ymd_opt(1960, 1, 1)?.and_hms_opt(0, 0, 0)?;
    let millis = match kind {
        SasDateKind::Date => (value * 86_400_000.0).round(),
        SasDateKind::DateTime => (value * 1000.0).round(),
    };
    if !millis.is_finite() || millis.abs() > 1e15 {
        return None;
    }
    epoch.checked_add_signed(TimeDelta::milliseconds(millis as i64))
}

// ---------------------------------------------------------------------------
// Excel
// ---------------------------------------------------------------------------

fn read_excel(path: &Path) -> Result<RawTable, DataLoaderError> {
    let mut workbook = open_workbook_auto(path)?;
    let range = workbook
        .worksheet_range_at(0)
        .ok_or_else(|| DataLoaderError::EmptyFile(path.to_path_buf()))??;

    let mut rows = range.rows();
    let headers: Vec<String> = rows
        .next()
        .ok_or_else(|| DataLoaderError::EmptyFile(path.to_path_buf()))?
        .iter()
        .map(|c| c.to_string().trim().to_string())
        .collect();

    let rows = rows
        .map(|row| {
            row.iter()
                .map(|c| match c {
                    Data::Empty | Data::Error(_) => Cell::Empty,
                    Data::Int(i) => Cell::Number(*i as f64),
                    Data::Float(f) => Cell::Number(*f),
                    Data::Bool(b) => Cell::Text(b.to_string()),
                    Data::String(s) | Data::DateTimeIso(s) | Data::DurationIso(s) => {
                        Cell::Text(s.clone())
                    }
                    Data::DateTime(dt) if dt.is_datetime() => dt
                        .as_datetime()
                        .map(Cell::DateTime)
                        .unwrap_or(Cell::Number(dt.as_f64())),
                    Data::DateTime(dt) => Cell::Number(dt.as_f64()),
                })
                .collect()
        })
        .collect();

    Ok(RawTable { headers, rows })
}

// ---------------------------------------------------------------------------
// CSV
// ---------------------------------------------------------------------------

fn read_csv(path: &Path) -> Result<RawTable, DataLoaderError> {
    let bytes = std::fs::read(path)?;
    let bytes = bytes.strip_prefix(b"\xEF\xBB\xBF").unwrap_or(&bytes);
    let delimiter = detect_delimiter(bytes);

    let mut reader = csv::ReaderBuilder::new()
        .delimiter(delimiter)
        .flexible(true)
        .has_headers(true)
        .from_reader(bytes);

    let headers = reader
        .byte_headers()?
        .iter()
        .map(|h| decode_text(h).trim().to_string())
        .collect::<Vec<_>>();
    if headers.iter().all(String::is_empty) {
        return Err(DataLoaderError::EmptyFile(path.to_path_buf()));
    }

    let mut rows = Vec::new();
    for record in reader.byte_records() {
        let record = record?;
        rows.push(record.iter().map(|f| Cell::Text(decode_text(f))).collect());
    }
    Ok(RawTable { headers, rows })
}

/// Picks the most frequent of `;`, `,` and tab in the header line (outside quotes).
fn detect_delimiter(bytes: &[u8]) -> u8 {
    let mut counts = [(b';', 0usize), (b',', 0), (b'\t', 0)];
    let mut in_quotes = false;
    for &b in bytes.iter().take_while(|&&b| b != b'\n') {
        if b == b'"' {
            in_quotes = !in_quotes;
        } else if let Some(c) = counts.iter_mut().find(|(d, _)| *d == b && !in_quotes) {
            c.1 += 1;
        }
    }
    counts
        .iter()
        .max_by_key(|(_, n)| *n)
        .filter(|(_, n)| *n > 0)
        .map_or(b',', |(d, _)| *d)
}

/// UTF-8 if valid, otherwise Latin-1 / Windows-1252 style (one byte = one char).
fn decode_text(bytes: &[u8]) -> String {
    match std::str::from_utf8(bytes) {
        Ok(s) => s.to_string(),
        Err(_) => bytes.iter().map(|&b| b as char).collect(),
    }
}

mod xpt {
    use super::{Cell, RawTable, SasDateKind, decode_text, sas_number_to_datetime};
    use crate::data_loader::error::DataLoaderError;

    const RECORD: usize = 80;
    const HEADER_PREFIX: &[u8] = b"HEADER RECORD*******";

    struct Variable {
        is_numeric: bool,
        length: usize,
        name: String,
        format: String,
        position: usize,
    }

    fn err(msg: impl Into<String>) -> DataLoaderError {
        DataLoaderError::Xpt(msg.into())
    }

    /// Returns true if the 80-byte record at `offset` is a header record of one of `kinds`.
    fn is_header(data: &[u8], offset: usize, kinds: &[&[u8]]) -> bool {
        let Some(rec) = data.get(offset..offset + RECORD) else {
            return false;
        };
        rec.starts_with(HEADER_PREFIX)
            && kinds
                .iter()
                .any(|k| rec[HEADER_PREFIX.len()..].starts_with(k))
    }

    fn find_header(data: &[u8], from: usize, kinds: &[&[u8]]) -> Option<usize> {
        (from..data.len())
            .step_by(RECORD)
            .find(|&o| is_header(data, o, kinds))
    }

    fn be_i16(b: &[u8]) -> i16 {
        i16::from_be_bytes([b[0], b[1]])
    }

    fn be_i32(b: &[u8]) -> i32 {
        i32::from_be_bytes([b[0], b[1], b[2], b[3]])
    }

    fn trimmed(b: &[u8]) -> String {
        let end = b
            .iter()
            .rposition(|&c| c != b' ' && c != 0)
            .map_or(0, |p| p + 1);
        decode_text(&b[..end]).trim_start().to_string()
    }

    pub(super) fn read(data: &[u8]) -> Result<RawTable, DataLoaderError> {
        let is_v8 = if is_header(data, 0, &[b"LIBRARY HEADER RECORD"]) {
            false
        } else if is_header(data, 0, &[b"LIBV8   HEADER RECORD", b"LIBV8 HEADER RECORD"]) {
            true
        } else {
            return Err(err(
                "missing library header; not a SAS transport (XPT) file",
            ));
        };

        let member = find_header(data, RECORD, &[b"MEMBER", b"MEMBV8"])
            .ok_or_else(|| err("no dataset (member header) found"))?;
        // Bytes 74..78 of the member header hold the namestr length (140, or 136 on VAX/VMS).
        let namestr_len: usize = std::str::from_utf8(&data[member + 74..member + 78])
            .ok()
            .and_then(|s| s.trim().parse().ok())
            .filter(|&n| n == 140 || n == 136)
            .unwrap_or(140);

        let namestr_hdr = find_header(data, member, &[b"NAMESTR", b"NAMSTV8"])
            .ok_or_else(|| err("no NAMESTR header found"))?;
        let namestr_start = namestr_hdr + RECORD;
        let after_namestrs = find_header(data, namestr_start, &[b"OBS", b"LABELV8", b"LABELV9"])
            .ok_or_else(|| err("no OBS header found"))?;
        let obs_hdr = find_header(data, after_namestrs, &[b"OBS"])
            .ok_or_else(|| err("no OBS header found"))?;

        // Namestrs are padded with blanks to a multiple of 80 bytes; the padding is always
        // shorter than one namestr, so integer division yields the variable count.
        let n_vars = (after_namestrs - namestr_start) / namestr_len;
        let mut vars = Vec::with_capacity(n_vars);
        for i in 0..n_vars {
            let ns = &data[namestr_start + i * namestr_len..namestr_start + (i + 1) * namestr_len];
            let short_name = trimmed(&ns[8..16]);
            let long_name = if is_v8 && namestr_len >= 120 {
                trimmed(&ns[88..120])
            } else {
                String::new()
            };
            vars.push(Variable {
                is_numeric: be_i16(&ns[0..2]) == 1,
                length: usize::try_from(be_i16(&ns[4..6]))
                    .map_err(|_| err("negative variable length"))?,
                name: if long_name.is_empty() {
                    short_name
                } else {
                    long_name
                },
                format: trimmed(&ns[56..64]).to_ascii_uppercase(),
                position: usize::try_from(be_i32(&ns[84..88]))
                    .map_err(|_| err("negative variable position"))?,
            });
        }

        let row_len = vars
            .iter()
            .map(|v| v.position + v.length)
            .max()
            .unwrap_or(0);
        if row_len == 0 {
            return Ok(RawTable {
                headers: vars.into_iter().map(|v| v.name).collect(),
                rows: vec![],
            });
        }

        let obs_start = obs_hdr + RECORD;
        let obs_end = find_header(data, obs_start, &[b"MEMBER", b"MEMBV8"]).unwrap_or(data.len());
        let obs = &data[obs_start..obs_end];

        // The final record is blank-padded to 80 bytes, which can look like extra rows
        // when a row is shorter than 80 bytes. Drop trailing all-blank rows in that tail.
        let mut n_rows = obs.len() / row_len;
        while n_rows > 0 {
            let start = (n_rows - 1) * row_len;
            let in_padding_tail = obs.len() - start < RECORD + row_len;
            if in_padding_tail && obs[start..start + row_len].iter().all(|&b| b == b' ') {
                n_rows -= 1;
            } else {
                break;
            }
        }

        let date_kinds: Vec<Option<SasDateKind>> =
            vars.iter().map(|v| date_kind(&v.format)).collect();
        let rows = (0..n_rows)
            .map(|r| {
                let row = &obs[r * row_len..(r + 1) * row_len];
                vars.iter()
                    .zip(&date_kinds)
                    .map(|(v, kind)| {
                        let bytes = &row[v.position..v.position + v.length];
                        if !v.is_numeric {
                            let s = trimmed(bytes);
                            return if s.is_empty() {
                                Cell::Empty
                            } else {
                                Cell::Text(s)
                            };
                        }
                        match (ibm_to_f64(bytes), kind) {
                            (None, _) => Cell::Empty,
                            (Some(n), Some(k)) => sas_number_to_datetime(n, Some(*k))
                                .map(Cell::DateTime)
                                .unwrap_or(Cell::Number(n)),
                            (Some(n), None) => Cell::Number(n),
                        }
                    })
                    .collect()
            })
            .collect();

        Ok(RawTable {
            headers: vars.into_iter().map(|v| v.name).collect(),
            rows,
        })
    }

    /// Classifies a SAS display format as a date (days) or datetime (seconds) format.
    fn date_kind(format: &str) -> Option<SasDateKind> {
        let base = format.trim_end_matches(|c: char| c.is_ascii_digit() || c == '.');
        const DATETIME: [&str; 6] = [
            "DATETIME", "DATEAMPM", "E8601DT", "IS8601DT", "B8601DT", "MDYAMPM",
        ];
        const DATE: [&str; 16] = [
            "DATE", "YYMMDD", "MMDDYY", "DDMMYY", "E8601DA", "IS8601DA", "B8601DA", "YYMMDDN",
            "MMDDYYN", "DDMMYYN", "JULIAN", "WEEKDATE", "WORDDATE", "EURDFDE", "NLDATE", "MINGUO",
        ];
        if DATETIME.contains(&base) {
            Some(SasDateKind::DateTime)
        } else if DATE.contains(&base) {
            Some(SasDateKind::Date)
        } else {
            None
        }
    }

    /// Converts a (possibly truncated, 2..=8 byte) IBM System/370 double to f64.
    /// Returns `None` for SAS missing values (`.`, `._`, `.A`–`.Z`).
    fn ibm_to_f64(bytes: &[u8]) -> Option<f64> {
        let mut b = [0u8; 8];
        let n = bytes.len().min(8);
        b[..n].copy_from_slice(&bytes[..n]);

        if b[1..].iter().all(|&x| x == 0) {
            return match b[0] {
                0 => Some(0.0),
                b'.' | b'_' | b'A'..=b'Z' => None,
                _ => Some(0.0),
            };
        }
        let sign = if b[0] & 0x80 != 0 { -1.0 } else { 1.0 };
        let exponent = i32::from(b[0] & 0x7F) - 64;
        let mantissa = b[1..]
            .iter()
            .fold(0u64, |acc, &x| (acc << 8) | u64::from(x));
        Some(sign * (mantissa as f64 / 2f64.powi(56)) * 16f64.powi(exponent))
    }

    #[cfg(test)]
    mod tests {
        use super::ibm_to_f64;

        #[test]
        fn ibm_values() {
            assert_eq!(ibm_to_f64(&[0x41, 0x10, 0, 0, 0, 0, 0, 0]), Some(1.0));
            assert_eq!(ibm_to_f64(&[0xC1, 0x10, 0, 0, 0, 0, 0, 0]), Some(-1.0));
            assert_eq!(ibm_to_f64(&[0x40, 0x80, 0, 0, 0, 0, 0, 0]), Some(0.5));
            assert_eq!(ibm_to_f64(&[0; 8]), Some(0.0));
            assert_eq!(ibm_to_f64(&[b'.', 0, 0, 0, 0, 0, 0, 0]), None);
            assert_eq!(ibm_to_f64(&[b'A', 0, 0, 0, 0, 0, 0, 0]), None);
            assert_eq!(ibm_to_f64(&[0x41, 0x30, 0, 0]), Some(3.0)); // truncated length 4
        }
    }
}
