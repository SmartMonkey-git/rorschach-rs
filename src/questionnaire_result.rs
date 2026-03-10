use crate::condition::Condition;
use crate::error::ToCsvError;
use crate::traits::ToCsv;
use crate::utils::{escape_csv_field, format_optional_datetime};
use chrono::{DateTime, Utc};
use std::collections::HashSet;
use std::error::Error;
use std::fmt;
use std::io::Write;

#[derive(Debug)]
pub struct QuestionnaireResult {
    id: String,
    diagnosis: Option<Condition>,
    phenotypes: HashSet<Condition>,
    taken_at: Option<DateTime<Utc>>,
}

impl QuestionnaireResult {
    pub fn new(
        id: impl Into<String>,
        diagnosis: Option<Condition>,
        phenotypes: HashSet<Condition>,
        taken_at: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            id: id.into(),
            diagnosis: diagnosis.clone(),
            phenotypes,
            taken_at,
        }
    }
}
impl ToCsv<Vec<QuestionnaireResult>> for Vec<QuestionnaireResult> {
    fn to_csv<W: Write>(&self, writer: &mut W) -> Result<(), ToCsvError> {
        let deduplicated: Vec<Vec<_>> = self
            .iter()
            .map(|r| -> Result<Vec<_>, ToCsvError> {
                let mut phenotype_map: std::collections::HashMap<&str, _> =
                    std::collections::HashMap::new();
                for p in r.phenotypes.iter() {
                    let term_id = p.term().id();
                    let entry = phenotype_map.entry(term_id).or_insert(p);

                    let current_severity = entry
                        .severity()
                        .as_ref()
                        .and_then(|s| s.as_severity())
                        .ok_or_else(|| ToCsvError::CantParseSeverity {
                            value: entry.severity().cloned(),
                        })?;

                    let new_severity = p
                        .severity()
                        .as_ref()
                        .and_then(|s| s.as_severity())
                        .ok_or_else(|| ToCsvError::CantParseSeverity {
                            value: p.severity().cloned(),
                        })?;

                    if new_severity > current_severity {
                        *entry = p;
                    }
                }
                let mut sorted: Vec<_> = phenotype_map.into_values().collect();
                sorted.sort_by_key(|p| p.term().id());
                Ok(sorted)
            })
            .collect::<Result<Vec<_>, ToCsvError>>()?;

        let max_phenotypes = deduplicated.iter().map(|p| p.len()).max().unwrap_or(0);

        // Write header
        writer.write_all(b"id,\
        diagnosis_term_id,diagnosis_term_label,diagnosis_severity_id,diagnosis_severity_label,diagnosis_observed_start,diagnosis_observed_end")?;

        for i in 1..=max_phenotypes {
            write!(
                writer,
                ",phenotype_{i}_term_id,phenotype_{i}_term_label,\
                phenotype_{i}_severity_id,phenotype_{i}_severity_label,\
                phenotype_{i}_observed_start,phenotype_{i}_observed_end"
            )?;
        }
        writer.write_all(b"\n")?;

        // Second pass: write data rows using already-deduplicated phenotypes
        for (result, sorted_phenotypes) in self.iter().zip(deduplicated.iter()) {
            writer.write_all(escape_csv_field(&result.id).as_bytes())?;
            writer.write_all(b",")?;

            if let Some(diag) = &result.diagnosis {
                writer.write_all(escape_csv_field(diag.term().id()).as_bytes())?;
                writer.write_all(b",")?;
                writer.write_all(escape_csv_field(diag.term().label()).as_bytes())?;
                writer.write_all(b",")?;
                writer.write_all(
                    escape_csv_field(diag.severity().as_ref().map(|s| s.id()).unwrap_or_default())
                        .as_bytes(),
                )?;
                writer.write_all(b",")?;
                writer.write_all(
                    escape_csv_field(
                        diag.severity()
                            .as_ref()
                            .map(|s| s.label())
                            .unwrap_or_default(),
                    )
                    .as_bytes(),
                )?;
                writer.write_all(b",")?;
                writer.write_all(format_optional_datetime(diag.observed_start()).as_bytes())?;
                writer.write_all(b",")?;
                writer.write_all(format_optional_datetime(diag.observed_end()).as_bytes())?;
            } else {
                writer.write_all(b",,,,,,")?;
            }

            for i in 0..max_phenotypes {
                writer.write_all(b",")?;
                if let Some(p) = sorted_phenotypes.get(i) {
                    writer.write_all(escape_csv_field(p.term().id()).as_bytes())?;
                    writer.write_all(b",")?;
                    writer.write_all(escape_csv_field(p.term().label()).as_bytes())?;
                    writer.write_all(b",")?;
                    writer.write_all(
                        escape_csv_field(p.severity().as_ref().map(|s| s.id()).unwrap_or_default())
                            .as_bytes(),
                    )?;
                    writer.write_all(b",")?;
                    writer.write_all(
                        escape_csv_field(
                            p.severity().as_ref().map(|s| s.label()).unwrap_or_default(),
                        )
                        .as_bytes(),
                    )?;
                    writer.write_all(b",")?;
                    writer.write_all(format_optional_datetime(p.observed_start()).as_bytes())?;
                    writer.write_all(b",")?;
                    writer.write_all(format_optional_datetime(p.observed_end()).as_bytes())?;
                } else {
                    writer.write_all(b",,,,,")?;
                }
            }

            writer.write_all(b"\n")?;
        }
        Ok(())
    }
}

impl fmt::Display for QuestionnaireResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "╭─────────────────────────────────────────╮")?;
        writeln!(f, "│       ✦  Questionnaire Result  ✦        │")?;
        writeln!(f, "╰─────────────────────────────────────────╯")?;
        writeln!(f)?;

        writeln!(f, "  ID:        {}", self.id)?;

        writeln!(f)?;

        match &self.taken_at {
            Some(date) => writeln!(f, "  Taken on: {}", date)?,
            None => writeln!(f, "  Taken on: Unknown")?,
        }

        writeln!(f)?;

        match &self.diagnosis {
            Some(diag) => writeln!(f, "  Diagnosis:  {}", diag)?,
            None => writeln!(f, "  Diagnosis:  No diagnosis recorded yet.")?,
        }

        writeln!(f)?;

        writeln!(f, "  Observed Phenotypes:")?;
        if self.phenotypes.is_empty() {
            writeln!(f, "       None recorded.")?;
        } else {
            for phenotype in &self.phenotypes {
                writeln!(f, "       •  {}", phenotype)?;
            }
        }

        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::condition::Condition;
    use crate::term::{DiagnosisTerms, PhenotypeTerms, SeverityTerms};
    use chrono::TimeZone;
    use std::collections::HashSet;

    fn to_csv_string(results: &Vec<QuestionnaireResult>) -> String {
        let mut buf = Vec::new();
        results.to_csv(&mut buf).expect("to_csv should not fail");
        String::from_utf8(buf).expect("output should be valid UTF-8")
    }

    #[test]
    fn test_to_csv_empty() {
        let results: Vec<QuestionnaireResult> = vec![];
        let csv = to_csv_string(&results);

        assert_eq!(
            csv,
            "id,diagnosis_term_id,diagnosis_term_label,diagnosis_severity_id,diagnosis_severity_label,diagnosis_observed_start,diagnosis_observed_end\n"
        );
    }

    #[test]
    fn test_to_csv_no_diagnosis_no_phenotypes() {
        let results = vec![QuestionnaireResult::new(
            "result-1",
            None,
            HashSet::new(),
            None,
        )];
        let csv = to_csv_string(&results);
        let lines: Vec<&str> = csv.lines().collect();

        assert_eq!(lines.len(), 2);
        assert_eq!(lines[1], "result-1,,,,,,,");
    }

    #[test]
    fn test_to_csv_with_full_diagnosis_no_phenotypes() {
        let diagnosis = Condition::new(
            DiagnosisTerms::DepressiveDisorder,
            SeverityTerms::Severe,
            Some(Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap()),
            Some(Utc.with_ymd_and_hms(2024, 6, 1, 0, 0, 0).unwrap()),
        );

        let results = vec![QuestionnaireResult::new(
            "result-2",
            Some(diagnosis),
            HashSet::new(),
            None,
        )];
        let csv = to_csv_string(&results);
        let lines: Vec<&str> = csv.lines().collect();

        assert_eq!(lines.len(), 2);
        assert!(
            lines[1].starts_with("result-2,MONDO:0002050,depressive disorder,HP:0012828,Severe,")
        );
        assert!(lines[1].contains("2024-01-01T00:00:00+00:00"));
        assert!(lines[1].contains("2024-06-01T00:00:00+00:00"));
    }

    #[test]
    fn test_to_csv_with_phenotypes() {
        let phenotype = Condition::new(
            PhenotypeTerms::LowSelfEsteem,
            SeverityTerms::Mild,
            Some(Utc.with_ymd_and_hms(2023, 3, 15, 0, 0, 0).unwrap()),
            None,
        );

        let mut phenotypes = HashSet::new();
        phenotypes.insert(phenotype);

        let results = vec![QuestionnaireResult::new("result-4", None, phenotypes, None)];
        let csv = to_csv_string(&results);
        let lines: Vec<&str> = csv.lines().collect();

        assert_eq!(lines.len(), 2);

        assert!(lines[0].contains("phenotype_1_term_id"));
        assert!(lines[0].contains("phenotype_1_term_label"));
        assert!(lines[0].contains("phenotype_1_severity_id"));
        assert!(lines[0].contains("phenotype_1_severity_label"));
        assert!(lines[0].contains("phenotype_1_observed_start"));
        assert!(lines[0].contains("phenotype_1_observed_end"));

        assert!(lines[1].contains("HP:0031469"));
        assert!(lines[1].contains("Low self-esteem"));
        assert!(lines[1].contains("HP:0012825"));
        assert!(lines[1].contains("Mild"));
        assert!(lines[1].contains("2023-03-15T00:00:00+00:00"));
    }

    #[test]
    fn test_to_csv_deduplicates_phenotypes_keeping_highest_severity() {
        let phenotype_mild = Condition::new(
            PhenotypeTerms::Guilt,
            SeverityTerms::Mild,
            Some(Utc.with_ymd_and_hms(2023, 3, 15, 0, 0, 0).unwrap()),
            None,
        );
        let phenotype_severe = Condition::new(
            PhenotypeTerms::Guilt,
            SeverityTerms::Severe,
            Some(Utc.with_ymd_and_hms(2023, 3, 15, 0, 0, 0).unwrap()),
            Some(Utc.with_ymd_and_hms(2023, 6, 1, 0, 0, 0).unwrap()),
        );

        let mut phenotypes = HashSet::new();
        phenotypes.insert(phenotype_mild);
        phenotypes.insert(phenotype_severe);

        let results = vec![QuestionnaireResult::new(
            "result-dedup",
            None,
            phenotypes,
            None,
        )];
        let csv = to_csv_string(&results);
        let lines: Vec<&str> = csv.lines().collect();

        assert_eq!(lines.len(), 2);

        assert!(lines[0].contains("phenotype_1_term_id"));
        assert!(!lines[0].contains("phenotype_2_term_id"));

        assert!(lines[1].contains("Severe"));
        assert!(!lines[1].contains("Mild"));
    }

    #[test]
    fn test_to_csv_multiple_results_same_column_width() {
        let phenotype = Condition::new(
            PhenotypeTerms::LowSelfEsteem,
            SeverityTerms::Mild,
            None,
            None,
        );
        let mut phenotypes = HashSet::new();
        phenotypes.insert(phenotype);

        let results = vec![
            QuestionnaireResult::new("result-5", None, HashSet::new(), None),
            QuestionnaireResult::new("result-6", None, phenotypes, None),
        ];
        let csv = to_csv_string(&results);
        let lines: Vec<&str> = csv.lines().collect();

        assert_eq!(lines.len(), 3);
        assert!(lines[1].starts_with("result-5,"));
        assert!(lines[2].starts_with("result-6,"));

        let header_cols = lines[0].split(',').count() + 1;
        assert_eq!(lines[1].split(',').count(), header_cols);
        assert_eq!(lines[2].split(',').count(), header_cols);
    }

    #[test]
    fn test_to_csv_field_with_comma_is_quoted() {
        let results = vec![QuestionnaireResult::new(
            "id,with,commas",
            None,
            HashSet::new(),
            None,
        )];
        let csv = to_csv_string(&results);
        let lines: Vec<&str> = csv.lines().collect();

        assert!(lines[1].starts_with("\"id,with,commas\""));
    }
}
