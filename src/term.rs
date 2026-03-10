use crate::utils::escape_csv_field;
use std::fmt;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Term {
    id: String,
    label: String,
}

impl Term {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn label(&self) -> &str {
        &self.label
    }
    pub fn as_severity(&self) -> Option<SeverityTerms> {
        if self.id == SeverityTerms::Borderline.as_term().id {
            Some(SeverityTerms::Borderline)
        } else if self.id == SeverityTerms::Mild.as_term().id {
            Some(SeverityTerms::Mild)
        } else if self.id == SeverityTerms::Moderate.as_term().id {
            Some(SeverityTerms::Moderate)
        } else if self.id == SeverityTerms::Severe.as_term().id {
            Some(SeverityTerms::Severe)
        } else if self.id == SeverityTerms::Profound.as_term().id {
            Some(SeverityTerms::Profound)
        } else {
            None
        }
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} [{}]", self.label, self.id)
    }
}

pub enum DiagnosisTerms {
    DepressiveDisorder,
}

impl DiagnosisTerms {
    pub fn as_term(&self) -> Term {
        match self {
            DiagnosisTerms::DepressiveDisorder => Term::new("MONDO:0002050", "depressive disorder"),
        }
    }
}
impl From<DiagnosisTerms> for Term {
    fn from(value: DiagnosisTerms) -> Self {
        value.as_term()
    }
}
pub enum PhenotypeTerms {
    Anhedonia,
    Depression,
    SleepDisturbance,
    Fatigue,
    AbnormalEatingBehavior,
    LowSelfEsteem,
    Guilt,
    AbnormalVolitionalState,
    SuicidalIdeation,
    DiminishedAbilityToConcentrate,
}

impl PhenotypeTerms {
    pub fn as_term(&self) -> Term {
        match self {
            PhenotypeTerms::Anhedonia => Term::new("HP:0012154", "Anhedonia"),
            PhenotypeTerms::Depression => Term::new("HP:0000716", "Depression"),
            PhenotypeTerms::SleepDisturbance => Term::new("HP:0002360", "Sleep disturbance"),
            PhenotypeTerms::Fatigue => Term::new("HP:0012378", "Fatigue"),
            PhenotypeTerms::AbnormalEatingBehavior => {
                Term::new("HP:0100738", "Abnormal eating behavior")
            }
            PhenotypeTerms::LowSelfEsteem => Term::new("HP:0031469", "Low self-esteem"),
            PhenotypeTerms::Guilt => Term::new("HP:6000011", "Guilt"),
            PhenotypeTerms::AbnormalVolitionalState => {
                Term::new("HP:0025780", "Abnormal Volitional state")
            }
            PhenotypeTerms::SuicidalIdeation => Term::new(
                "HP:0031589",
                "Suicidal ideation
",
            ),
            PhenotypeTerms::DiminishedAbilityToConcentrate => {
                Term::new("HP:0031987", "Diminished ability to concentrate")
            }
        }
    }
}

impl From<PhenotypeTerms> for Term {
    fn from(value: PhenotypeTerms) -> Self {
        value.as_term()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum SeverityTerms {
    Borderline,
    Mild,
    Moderate,
    Severe,
    Profound,
}

impl SeverityTerms {
    pub fn as_term(&self) -> Term {
        match self {
            SeverityTerms::Borderline => Term::new("HP:0012827", "Borderline"),
            SeverityTerms::Mild => Term::new("HP:0012825", "Mild"),
            SeverityTerms::Moderate => Term::new("HP:0012826", "Moderate"),
            SeverityTerms::Profound => Term::new("HP:0012829", "Profound"),
            SeverityTerms::Severe => Term::new("HP:0012828", "Severe"),
        }
    }
}

impl From<SeverityTerms> for Term {
    fn from(value: SeverityTerms) -> Self {
        value.as_term()
    }
}
