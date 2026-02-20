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
        }
    }
}

impl From<PhenotypeTerms> for Term {
    fn from(value: PhenotypeTerms) -> Self {
        value.as_term()
    }
}

pub enum SeverityTerms {
    Borderline,
    Mild,
    Moderate,
    Profound,
    Severe,
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
