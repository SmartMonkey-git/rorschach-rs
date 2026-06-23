use crate::condition::Condition;
use crate::error::ToCsvError;
use crate::traits::ToCsv;
use crate::utils::{escape_csv_field, format_optional_datetime};
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::fmt;
use std::io::Write;

#[derive(Debug)]
pub struct QuestionnaireResult {
    id: String,
    proband_id: String,
    name: String,
    diagnosis: Option<Condition>,
    phenotypes: Vec<Option<Condition>>, // TODO: This should be Option<HashSet<Condition>>, because people can just not answer the question.
    taken_at: Option<DateTime<Utc>>,
}

impl QuestionnaireResult {
    pub fn new(
        id: Option<impl Into<String>>,
        proband_id: impl Into<String>,
        name: impl Into<String>,
        diagnosis: Option<Condition>,
        phenotypes: Vec<Option<Condition>>,
        taken_at: Option<&DateTime<Utc>>,
    ) -> Self {
        Self {
            id: id
                .map(Into::into)
                .unwrap_or_else(|| uuid::Uuid::new_v4().to_string()),
            proband_id: proband_id.into(),
            name: name.into(),
            diagnosis: diagnosis.clone(),
            phenotypes,
            taken_at: taken_at.copied(),
        }
    }

    pub fn get_unique_phenotypes(&self) -> Vec<(usize, Condition)> {
        let mut temp = HashMap::new();
        for (idx, pt_opt) in self.phenotypes.iter().enumerate() {
            if let Some(pt) = pt_opt {
                match temp.get(pt.term().id()) {
                    None => {
                        temp.insert(pt.term().id(), (idx, pt));
                    }
                    Some((_, existing_pt)) => match (pt.severity(), existing_pt.severity()) {
                        (Some(s), Some(e_s)) => {
                            if s.as_severity() > e_s.as_severity() {
                                temp.insert(pt.term().id(), (idx, pt));
                            }
                        }
                        (Some(_), None) => {
                            temp.insert(pt.term().id(), (idx, pt));
                        }
                        _ => {}
                    },
                }
            }
        }
        let mut output: Vec<(usize, Condition)> = temp
            .into_values()
            .map(|(idx, pt)| (idx, pt.clone()))
            .collect();
        output.sort_by_key(|&(idx, _)| idx);
        output
    }
}
impl ToCsv for Vec<QuestionnaireResult> {
    fn to_csv<W: Write>(&self, writer: &mut W, filter_duplicates: bool) -> Result<(), ToCsvError> {
        fn write_row<W: Write>(
            proband_id: &str,
            instrument_id: &str,
            instrument_name: &str,
            instrument_taken_at: Option<&DateTime<Utc>>,
            q_idx: usize,
            condition: &Condition,
            writer: &mut W,
        ) -> Result<(), ToCsvError> {
            writer.write_all(escape_csv_field(proband_id).as_bytes())?;
            writer.write_all(b",")?;

            writer.write_all(escape_csv_field(instrument_id).as_bytes())?;
            writer.write_all(b",")?;

            writer.write_all(q_idx.to_string().as_bytes())?;
            writer.write_all(b",")?;

            writer.write_all(escape_csv_field(instrument_name).as_bytes())?;
            writer.write_all(b",")?;

            if let Some(taken_at) = instrument_taken_at {
                writer.write_all(escape_csv_field(&taken_at.to_string()).as_bytes())?;
            }
            writer.write_all(b",")?;

            writer.write_all(escape_csv_field("phenotype").as_bytes())?;
            writer.write_all(b",")?;

            writer.write_all(escape_csv_field(condition.term().id()).as_bytes())?;
            writer.write_all(b",")?;

            writer.write_all(escape_csv_field(condition.term().label()).as_bytes())?;
            writer.write_all(b",")?;

            match condition.severity() {
                None => {
                    writer.write_all(b",")?;
                    writer.write_all(b",")?;
                }
                Some(s) => {
                    writer.write_all(escape_csv_field(s.id()).as_bytes())?;
                    writer.write_all(b",")?;

                    writer.write_all(escape_csv_field(s.label()).as_bytes())?;
                    writer.write_all(b",")?;
                }
            }

            writer.write_all(condition.excluded().to_string().as_bytes())?;
            writer.write_all(b",")?;

            writer.write_all(format_optional_datetime(condition.observed_start()).as_bytes())?;
            writer.write_all(b",")?;
            writer.write_all(format_optional_datetime(condition.observed_end()).as_bytes())?;
            writer.write_all(b"\n")?;

            Ok(())
        }

        writer.write_all(b"proband_id,instrument_id,question_idx,instrument_name,taken_at,term_type,term_id,term_label,severity_id,severity_label,excluded,observed_start,observed_end\n")?;
        for result in self.iter() {
            if filter_duplicates {
                for (q_idx, pt) in result.get_unique_phenotypes() {
                    write_row(
                        &result.proband_id,
                        &result.id,
                        &result.name,
                        result.taken_at.as_ref(),
                        q_idx,
                        &pt,
                        writer,
                    )?
                }
            } else {
                for (q_idx, pt_opt) in result.phenotypes.iter().enumerate() {
                    if let Some(pt) = pt_opt {
                        write_row(
                            &result.proband_id,
                            &result.id,
                            &result.name,
                            result.taken_at.as_ref(),
                            q_idx,
                            pt,
                            writer,
                        )?
                    }
                }
            }
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

        writeln!(f, "  ID:        {}", self.proband_id)?;

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
                writeln!(f, "       •  {:?}", phenotype)?;
            }
        }

        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::condition::Condition;
    use crate::term::{PhenotypeTerms, SeverityTerms};
    use chrono::TimeZone;

    fn to_csv_string(results: &Vec<QuestionnaireResult>) -> String {
        let mut buf = Vec::new();
        results
            .to_csv(&mut buf, true)
            .expect("to_csv should not fail");
        String::from_utf8(buf).expect("output should be valid UTF-8")
    }

    #[test]
    fn test_to_csv_empty() {
        let results: Vec<QuestionnaireResult> = vec![];
        let csv = to_csv_string(&results);

        assert_eq!(
            csv,
            "proband_id,instrument_id,question_idx,instrument_name,taken_at,term_type,term_id,term_label,severity_id,severity_label,excluded,observed_start,observed_end\n"
        );
    }

    #[test]
    fn test_to_csv_no_diagnosis_no_phenotypes() {
        let results = vec![QuestionnaireResult::new(
            Some("result-1"),
            "pp1",
            "PHQ-9",
            None,
            Vec::new(),
            None,
        )];
        let csv = to_csv_string(&results);
        let lines: Vec<&str> = csv.lines().collect();

        assert_eq!(lines.len(), 1);
    }

    #[test]
    fn test_to_csv_deduplicates_phenotypes_keeping_highest_severity() {
        let phenotype_mild = Condition::new(
            PhenotypeTerms::Guilt,
            SeverityTerms::Mild,
            false,
            Some(Utc.with_ymd_and_hms(2023, 3, 15, 0, 0, 0).unwrap()),
            None,
        );
        let phenotype_severe = Condition::new(
            PhenotypeTerms::Guilt,
            SeverityTerms::Severe,
            false,
            Some(Utc.with_ymd_and_hms(2023, 3, 15, 0, 0, 0).unwrap()),
            Some(Utc.with_ymd_and_hms(2023, 6, 1, 0, 0, 0).unwrap()),
        );

        let phenotypes = vec![Some(phenotype_mild), Some(phenotype_severe)];

        let results = vec![QuestionnaireResult::new(
            Some("result-dedup"),
            "pp1",
            "PHQ-9",
            None,
            phenotypes,
            None,
        )];
        let csv = to_csv_string(&results);
        let lines: Vec<&str> = csv.lines().collect();

        assert_eq!(lines.len(), 2);

        assert!(lines[1].contains("Severe"));
        assert!(!lines[1].contains("Mild"));
    }

    #[test]
    fn test_to_csv_multiple_results_same_column_width() {
        let low_self_esteem = Condition::new(
            PhenotypeTerms::LowSelfEsteem,
            SeverityTerms::Mild,
            false,
            None,
            None,
        );

        let guilt = Condition::new(
            PhenotypeTerms::Guilt,
            SeverityTerms::Mild,
            false,
            None,
            None,
        );

        let results = vec![
            QuestionnaireResult::new(
                Some("result-5"),
                "pp1",
                "PHQ-9",
                None,
                vec![Some(guilt)],
                None,
            ),
            QuestionnaireResult::new(
                Some("result-6"),
                "pp1",
                "PHQ-9",
                None,
                vec![Some(low_self_esteem)],
                None,
            ),
        ];
        let csv = to_csv_string(&results);
        let lines: Vec<&str> = csv.lines().collect();

        assert_eq!(lines.len(), 3);
        assert!(lines[1].contains(PhenotypeTerms::Guilt.as_term().label()));
        assert!(lines[2].contains(PhenotypeTerms::LowSelfEsteem.as_term().label()));

        let header_cols = lines[0].split(',').count();
        assert_eq!(lines[1].split(',').count(), header_cols);
        assert_eq!(lines[2].split(',').count(), header_cols);
    }
}
