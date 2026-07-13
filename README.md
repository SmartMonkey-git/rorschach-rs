# rorschach

A Rust library for defining, scoring, and interpreting standardized clinical
psychiatric rating scales — the kind used to screen for and track conditions
like depression, anxiety, and mania.

It comes with six built-in instruments (PHQ‑9, GAD‑7, ASRM, YMRS, MADRS,
HAM‑A) and a builder API for defining your own. Every scored answer maps to a
clinical term from a standard ontology (HPO for phenotypes, MONDO for
diagnoses), so results carry stable, interoperable IDs — not just free-text
labels — and can be exported to CSV for downstream analysis.

## How it fits together

| Type                  | Role                                                                                                                                                                 |
|-----------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `Questionnaire`       | A full instrument: its items, a score → `Condition` interpretation table, a scoring strategy, and an optional recall period (e.g. "past 2 weeks").                   |
| `QuestionnaireItem`   | A single question, mapping each possible answer score to a `Condition`.                                                                                              |
| `Answer`              | One respondent's answer to one item: an index plus an optional score.                                                                                                |
| `Condition`           | A clinical finding — a `Term`, an optional severity `Term`, whether it's explicitly excluded, and an observed time window.                                           |
| `Term`                | An ontology entry (id + label). `PhenotypeTerms` and `DiagnosisTerms` map to HPO/MONDO codes; `SeverityTerms` gives a shared severity scale (Borderline → Profound). |
| `QuestionnaireResult` | The output of scoring a filled-out questionnaire: an overall diagnosis, the per-item phenotype conditions, and metadata (proband id, timestamp).                     |

Scoring itself is pluggable via the `CalculateScore` trait:

- **`SumScore`** — plain sum of item scores (used by all six presets).
- **`ProratedScore`** — scales a partial set of answers up to what the full
  item count would predict, for handling incomplete questionnaires.

## Built-in instruments

| Preset  | Full name                                 | Measures   | Recall period |
|---------|-------------------------------------------|------------|---------------|
| `PHQ9`  | Patient Health Questionnaire‑9            | Depression | 2 weeks       |
| `GAD7`  | Generalized Anxiety Disorder‑7            | Anxiety    | 2 weeks       |
| `ASRM`  | Altman Self‑Rating Mania Scale            | Mania      | 2 weeks       |
| `YMRS`  | Young Mania Rating Scale                  | Mania      | 2 weeks       |
| `MADRS` | Montgomery‑Åsberg Depression Rating Scale | Depression | 2 weeks       |
| `HAMA`  | Hamilton Anxiety Rating Scale             | Anxiety    | Point-in-time |

Each preset hard-codes its official items, scoring thresholds, and phenotype
mappings, so `QuestionnairePresets::PHQ9.build()` always succeeds.

## Quick start

```rust
use rorschach_rs::answer::Answer;
use rorschach_rs::questionnaire_presets::QuestionnairePresets;

fn main() {
    let phq9 = QuestionnairePresets::PHQ9.build();

    // One answer per item, in item order. `None` means "not answered".
    let answers = vec![
        Answer::new(0, Some(2.0)),
        Answer::new(1, Some(1.0)),
        Answer::new(2, Some(3.0)),
        Answer::new(3, Some(2.0)),
        Answer::new(4, Some(0.0)),
        Answer::new(5, Some(1.0)),
        Answer::new(6, Some(0.0)),
        Answer::new(7, Some(0.0)),
        Answer::new(8, Some(0.0)),
    ];

    let result = phq9
        .evaluate("phq9-2026-07-13", &answers, None)
        .expect("answers should match the instrument");

    println!("{result}"); // pretty-printed summary
}
```

## Exporting results to CSV

```rust
use rorschach_rs::traits::ToCsv;

let mut buf = Vec::new();
// `true` deduplicates repeated phenotypes, keeping the highest severity seen.
vec![result].to_csv( & mut buf, true) ?;
```

## Defining a custom questionnaire

```rust
use rorschach_rs::condition::Condition;
use rorschach_rs::questionnaire::Questionnaire;
use rorschach_rs::questionnaire_item::QuestionnaireItem;
use rorschach_rs::score_calculations::sum_score::SumScore;
use rorschach_rs::term::{PhenotypeTerms, SeverityTerms};

// Builders are returned by `Questionnaire::builder(..)`, `QuestionnaireItem::builder(..)`,
// and `Condition::builder(..)` — chain directly off those, no separate import needed.
let question = QuestionnaireItem::builder(4)
.stem("Feeling nervous, anxious, or on edge")
.condition(0, Condition::builder(PhenotypeTerms::Anxiety).exclude().build())
.condition(3, Condition::builder(PhenotypeTerms::Anxiety).severity(SeverityTerms::Mild).build())
.build()
.expect("at least one condition was provided");

let questionnaire = Questionnaire::builder("My Scale", Box::new(SumScore))
.interpretation(0, Condition::builder(PhenotypeTerms::Anxiety).exclude().build())
.interpretation(5, Condition::builder(PhenotypeTerms::Anxiety).severity(SeverityTerms::Mild).build())
.item(question)
.build();
```

## Errors

- `RorschachError` — building or evaluating a questionnaire (mismatched
  answer counts, scores above the max, unmapped severities, missing
  interpretation for a score, etc.), via `thiserror`.
- `ToCsvError` — failures while writing results out as CSV.

## Project layout

```
src/
├── lib.rs                     # public module tree
├── answer.rs                  # Answer
├── condition.rs                # Condition
├── term.rs                     # Term, PhenotypeTerms, DiagnosisTerms, SeverityTerms
├── questionnaire.rs             # Questionnaire + evaluate()
├── questionnaire_item.rs        # QuestionnaireItem
├── questionnaire_result.rs      # QuestionnaireResult + CSV export
├── questionnaire_presets.rs      # QuestionnairePresets enum (public entry point)
├── error.rs                     # RorschachError, ToCsvError
├── traits.rs                    # CalculateScore, ToCsv, AsTerm
├── utils.rs                     # CSV escaping, datetime formatting
├── builders/                    # fluent builders for the above
│   ├── questionnaire_builder.rs
│   ├── questionnaire_item_builder.rs
│   └── condition_builder.rs
├── presets/                     # hard-coded instrument definitions (private)
│   ├── phq9.rs, gad7.rs, asrm.rs, ymrs.rs, madrs.rs, hama.rs
└── score_calculations/
    ├── sum_score.rs
    └── prorated_score.rs
```

## Dependencies

Inferred from the source (no `Cargo.toml` was included in the archive, so
double-check versions when you add it back): `chrono` (dates/durations),
`thiserror` (error types), `strum` (enum iteration/counting), `maplit`
(the `btreemap!` macro), and `uuid` (result IDs).

## Status

This crate has no `Cargo.toml`, `LICENSE`, or CI config in the current
checkout — worth adding before publishing. Test coverage is currently
concentrated in `questionnaire_result.rs`; the other modules are good
candidates for more unit tests, especially the per-instrument scoring
thresholds in `presets/`.