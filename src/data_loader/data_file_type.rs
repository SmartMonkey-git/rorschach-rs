use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataFileType {
    Excel,
    CSV,
    XPT,
}

impl DataFileType {
    /// Detects the file type from the file extension (case-insensitive).
    pub fn from_path(path: &Path) -> Option<Self> {
        let ext = path.extension()?.to_str()?.to_ascii_lowercase();
        match ext.as_str() {
            "xlsx" | "xlsm" | "xlsb" | "xls" | "ods" => Some(Self::Excel),
            "csv" | "tsv" | "txt" => Some(Self::CSV),
            "xpt" => Some(Self::XPT),
            _ => None,
        }
    }
}
