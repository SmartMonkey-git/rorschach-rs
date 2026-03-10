use crate::condition::Condition;
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
    fn to_csv<W: Write>(&self, writer: &mut W) -> Result<(), Box<dyn Error>> {
        writer.write_all(b"id,\
        diagnosis_term_id,diagnosis_term_label,diagnosis_severity_id,diagnosis_severity_label,diagnosis_observed_start,diagnosis_observed_end,\
        phenotypes\n")?;

        for result in self.iter() {
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

            writer.write_all(b",")?;

            let phenotypes: Vec<String> = result
                .phenotypes
                .iter()
                .map(|p| {
                    let mut parts = vec![
                        format!("id={}", p.term().id()),
                        format!("label={}", p.term().label()),
                    ];
                    if let Some(sev) = &p.severity() {
                        parts.push(format!("severity_id={}", sev.id()));
                        parts.push(format!("severity_label={}", sev.label()));
                    }
                    if let Some(start) = p.observed_start() {
                        parts.push(format!("start={}", start.to_rfc3339()));
                    }
                    if let Some(end) = p.observed_end() {
                        parts.push(format!("end={}", end.to_rfc3339()));
                    }
                    parts.join("|")
                })
                .collect();

            writer.write_all(escape_csv_field(&phenotypes.join(";")).as_bytes())?;
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
    use std::fs::File;
    use std::io::BufWriter;

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
            "id,diagnosis_term_id,diagnosis_term_label,diagnosis_severity_id,diagnosis_severity_label,diagnosis_observed_start,diagnosis_observed_end,phenotypes\n"
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
        // No diagnosis: expect 6 empty comma-separated fields, then empty phenotypes
        assert_eq!(lines[1], "result-1,,,,,,,,");
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
        let phenotype_field = lines[1].split(',').last().unwrap();
        assert!(phenotype_field.contains("id=HP:0031469"));
        assert!(phenotype_field.contains("label=Low self-esteem"));
        assert!(phenotype_field.contains("severity_id=HP:0012825"));
        assert!(phenotype_field.contains("severity_label=Mild"));
        assert!(phenotype_field.contains("start=2023-03-15T00:00:00+00:00"));
    }

    #[test]
    fn test_delete_me() {
        let phenotype = Condition::new(
            PhenotypeTerms::LowSelfEsteem,
            SeverityTerms::Mild,
            Some(Utc.with_ymd_and_hms(2023, 3, 15, 0, 0, 0).unwrap()),
            None,
        );

        let phenotype_2 = Condition::new(
            PhenotypeTerms::Guilt,
            SeverityTerms::Severe,
            Some(Utc.with_ymd_and_hms(2023, 3, 15, 0, 0, 0).unwrap()),
            Some(Utc.with_ymd_and_hms(2023, 3, 15, 0, 0, 0).unwrap()),
        );

        let diagnosis = Condition::new(
            DiagnosisTerms::DepressiveDisorder,
            SeverityTerms::Severe,
            Some(Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap()),
            Some(Utc.with_ymd_and_hms(2024, 6, 1, 0, 0, 0).unwrap()),
        );

        let mut phenotypes = HashSet::new();
        phenotypes.insert(phenotype);
        phenotypes.insert(phenotype_2);

        let mut results = vec![];
        for i in (0..10) {
            results.push(QuestionnaireResult::new(
                "result-4",
                Some(diagnosis.clone()),
                phenotypes.clone(),
                None,
            ));
        }

        let mut writer =
            BufWriter::new(File::create("/Users/rouvenreuter/Documents/test.csv").unwrap());
        results.to_csv(&mut writer).unwrap();
    }

    #[test]
    fn test_to_csv_multiple_results() {
        let results = vec![
            QuestionnaireResult::new("result-5", None, HashSet::new(), None),
            QuestionnaireResult::new("result-6", None, HashSet::new(), None),
        ];
        let csv = to_csv_string(&results);
        let lines: Vec<&str> = csv.lines().collect();

        // Header + 2 data rows
        assert_eq!(lines.len(), 3);
        assert!(lines[1].starts_with("result-5,"));
        assert!(lines[2].starts_with("result-6,"));
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
