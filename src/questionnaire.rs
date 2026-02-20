use crate::answer::Answer;
use crate::diagnosis::Diagnosis;
use crate::error::RorschachError;
use crate::questionnaire_item::QuestionnaireItem;
use crate::questionnaire_result::QuestionnaireResult;
use crate::traits::CalculateScore;
use std::collections::BTreeMap;

#[derive(Debug, PartialEq)]
pub struct Questionnaire {
    name: String,
    items: Vec<QuestionnaireItem>,
    interpretation: BTreeMap<usize, Option<Diagnosis>>,
    score_calculator: Box<dyn CalculateScore>,
}

impl Questionnaire {
    pub fn new(
        name: String,
        items: Vec<QuestionnaireItem>,
        interpretation: BTreeMap<usize, Option<Diagnosis>>,
        score_calculator: impl CalculateScore,
    ) -> Self {
        Self {
            name,
            items,
            interpretation,
            score_calculator: Box::new(score_calculator),
        }
    }

    pub fn evaluate(&self, answers: &[Answer]) -> Result<QuestionnaireResult, RorschachError> {
        if answers.len() != self.items.len() {
            return Err(RorschachError::AnswerQuestionMismatch {
                test_name: self.name.to_string(),
                expected: self.items.len(),
                found: answers.len(),
            });
        }

        let total_score: usize = answers.iter().map(|a| a.score()).sum();

        let max_score = self.max_score();

        if total_score > max_score {
            return Err(RorschachError::TotalScoreTooHigh {
                test_name: self.name.to_string(),
                max_score,
                found_score: total_score,
            });
        }

        let (_, diagnosis) = self
            .interpretation
            .range(..=total_score)
            .next_back()
            .ok_or_else(|| RorschachError::NoMatchingDiagnosis {
                found_score: total_score,
            })?;

        let mut result = QuestionnaireResult::from_diagnosis(diagnosis.clone());

        for (answer, question) in answers.into_iter().zip(self.items.iter()) {
            let phenotypes = question.answer(answer.score());
            result.push_phenotypes(phenotypes.to_vec());
        }

        Ok(result)
    }

    pub fn max_score(&self) -> usize {
        self.items.iter().map(|question| question.max_score()).sum()
    }
}
