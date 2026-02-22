use crate::answer::Answer;
use crate::diagnosis::Diagnosis;
use crate::error::RorschachError;
use crate::phenotype::Phenotype;
use crate::questionnaire_item::QuestionnaireItem;
use crate::questionnaire_result::QuestionnaireResult;
use crate::traits::{CalculateScore, EvaluateQuestionnaire};
use chrono::{DateTime, Duration, Utc};
use std::collections::{BTreeMap, HashSet};

#[derive(Debug)]
pub struct Questionnaire {
    name: String,
    items: Vec<QuestionnaireItem>,
    interpretation: BTreeMap<i32, Option<Diagnosis>>,
    score_calculator: Box<dyn CalculateScore>,
    recall_period: Option<Duration>,
}

impl Questionnaire {
    pub fn new(
        name: String,
        items: Vec<QuestionnaireItem>,
        interpretation: BTreeMap<i32, Option<Diagnosis>>,
        score_calculator: impl CalculateScore + 'static,
        recall_period: Option<Duration>,
    ) -> Self {
        Self {
            name,
            items,
            interpretation,
            score_calculator: Box::new(score_calculator),
            recall_period,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn max_score(&self) -> f32 {
        let max_item_scores = self
            .items
            .iter()
            .map(|question| question.max_score())
            .collect::<Vec<f32>>();

        self.score_calculator
            .calculate_score(max_item_scores.as_slice())
    }
}
impl EvaluateQuestionnaire for Questionnaire {
    fn evaluate(
        &self,
        questionnaire_id: &str,
        answers: &[Answer],
        taken_at: Option<DateTime<Utc>>,
    ) -> Result<QuestionnaireResult, RorschachError> {
        if answers.len() != self.items.len() {
            return Err(RorschachError::AnswerQuestionMismatch {
                test_name: self.name.to_string(),
                expected: self.items.len(),
                found: answers.len(),
            });
        }

        let item_scores: Vec<f32> = answers.iter().map(|a| a.score()).collect();
        let total_score = self.score_calculator.calculate_score(&item_scores);

        let max_score = self.max_score();

        if total_score > max_score {
            return Err(RorschachError::TotalScoreTooHigh {
                test_name: self.name.to_string(),
                max_score,
                found_score: total_score,
            });
        }

        let diagnosis = self.get_diagnosis(total_score)?;
        let mut result =
            QuestionnaireResult::new(questionnaire_id, diagnosis, HashSet::new(), taken_at);

        for (answer, question) in answers.into_iter().zip(self.items.iter()) {
            let mut phenotypes: Vec<Phenotype> = question
                .answer(answer.idx())
                .iter()
                .map(|pt| pt.clone())
                .collect();

            if let Some(taken) = taken_at
                && let Some(recall) = self.recall_period
            {
                for pt in phenotypes.iter_mut() {
                    pt.set_observed_start(Some(taken - recall));
                    pt.set_observed_end(Some(taken));
                }
            }

            result.push_phenotypes(phenotypes.to_vec());
        }

        Ok(result)
    }
}

impl Questionnaire {
    fn get_diagnosis(&self, total_score: f32) -> Result<Option<Diagnosis>, RorschachError> {
        let (_, diagnosis) = self
            .interpretation
            .range(..=total_score.ceil() as i32)
            .next_back()
            .ok_or_else(|| RorschachError::NoMatchingDiagnosis {
                found_score: total_score,
            })?;

        Ok(diagnosis.clone())
    }
}

impl PartialEq for Questionnaire {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.items == other.items
            && self.interpretation == other.interpretation
    }
}
