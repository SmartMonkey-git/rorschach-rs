use chrono::{DateTime, Utc};

/// Wraps fields containing commas, quotes, or newlines in double-quotes,
/// and escapes any internal double-quotes by doubling them.
pub(crate) fn escape_csv_field(field: &str) -> String {
    if field.contains(',') || field.contains('"') || field.contains('\n') {
        format!("\"{}\"", field.replace('"', "\"\""))
    } else {
        field.to_string()
    }
}

pub(crate) fn format_optional_datetime(dt: Option<&DateTime<Utc>>) -> String {
    dt.map(|d| d.to_rfc3339()).unwrap_or_default()
}
