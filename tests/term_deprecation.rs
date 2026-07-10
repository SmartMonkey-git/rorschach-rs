use ontolius::TermId;
use ontolius::io::OntologyLoaderBuilder;
use ontolius::ontology::OntologyTerms;
use ontolius::ontology::csr::FullCsrOntology;
use ontolius::term::MinimalTerm;
use ontology_registry::{
    BioRegistryMetadataProvider, FileSystemOntologyRegistry, FileType, OboLibraryProvider,
    OntologyRegistration, RegistryKey, SupportedOntology, Version,
};
use robinson_group_rust_template::term::PhenotypeTerms;
use robinson_group_rust_template::traits::AsTerm;
use std::path::PathBuf;
use std::str::FromStr;
use strum::IntoEnumIterator;

#[test]
pub fn term_deprecation() {
    let manifest_dir = PathBuf::from_str(env!("CARGO_MANIFEST_DIR")).unwrap();
    let integration_test_assets = manifest_dir.join("tests/assets");
    let registry = FileSystemOntologyRegistry::new(
        integration_test_assets,
        BioRegistryMetadataProvider::default(),
        OboLibraryProvider::default(),
    );

    let reg_key = RegistryKey::new(SupportedOntology::HP, Version::Latest, FileType::Json);

    let hpo = registry.register(reg_key).unwrap();

    let loader = OntologyLoaderBuilder::new().obographs_parser().build();

    let ontology: FullCsrOntology = loader.load_from_read(hpo).unwrap();

    let mut obsolete_terms = Vec::new();
    let mut label_wrong = Vec::new();
    for term in PhenotypeTerms::iter() {
        let t = term.as_term();

        let term_id =
            TermId::from_str(t.id()).unwrap_or_else(|_| panic!("Failed to get term {}", t.id()));

        let term_state = ontology
            .term_by_id(&term_id)
            .unwrap_or_else(|| panic!("Failed to get term {}", t.id()));

        if term_state.is_obsolete() {
            obsolete_terms.push(t.label().to_string());
        }
        if term_state.name() != t.label() {
            label_wrong.push(t.label().to_string());
        }
    }

    if !obsolete_terms.is_empty() || !label_wrong.is_empty() {
        panic!(
            "Got Obsolete terms: {:?}\nGot wrong labels: {:?}",
            obsolete_terms, label_wrong
        );
    }
}
