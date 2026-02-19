use crate::term::Term;

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Phenotype {
    r#type: Term,
    severity: Term,
}
