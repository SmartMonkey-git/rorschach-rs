use crate::answer::Answer;
use crate::condition::Condition;
use crate::error::RorschachError;
use crate::questionnaire_item::QuestionnaireItem;
use crate::questionnaire_result::QuestionnaireResult;
use crate::term::{SeverityTerms, Term};
use crate::traits::CalculateScore;
use chrono::{DateTime, Duration, Utc};
use std::collections::{BTreeMap, HashSet};

#[derive(Debug)]
pub struct Questionnaire {
    name: String,
    items: Vec<QuestionnaireItem>,
    interpretation: BTreeMap<i32, Option<Condition>>,
    score_calculator: Box<dyn CalculateScore>,
    recall_period: Option<Duration>,
}

impl Questionnaire {
    pub fn new(
        name: impl Into<String>,
        items: Vec<QuestionnaireItem>,
        interpretation: BTreeMap<i32, Option<Condition>>,
        score_calculator: impl CalculateScore + 'static,
        recall_period: Option<Duration>,
    ) -> Self {
        Self {
            name: name.into(),
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
            .map(|question| question.n_answers())
            .collect::<Vec<f32>>();

        self.score_calculator
            .calculate_score(max_item_scores.as_slice())
    }
}

impl Questionnaire {
    pub fn evaluate(
        &self,
        questionnaire_id: &str,
        answers: &[Answer],
        taken_at: Option<&DateTime<Utc>>,
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

        let mut phenotypes_set: HashSet<Condition> = HashSet::new();
        for (answer, question) in answers.iter().zip(self.items.iter()) {
            let mut phenotype: Condition = question.phenotype().clone();
            self.set_time(&mut phenotype, taken_at);

            let severity = Self::calculate_severity(answer, &question)?;
            phenotype.set_severity(&severity);
            phenotypes_set.insert(phenotype);
        }

        let diagnosis = self.get_diagnosis(total_score, taken_at)?;
        let result = QuestionnaireResult::new(
            questionnaire_id,
            &self.name,
            diagnosis,
            phenotypes_set,
            taken_at,
        );

        Ok(result)
    }

    fn calculate_severity(
        answer: &Answer,
        question: &QuestionnaireItem,
    ) -> Result<Term, RorschachError> {
        // TODO: this need to take into account answers that exclude the phenotype, instead of assigning a severity to it. Currently, this is solved with the -1.
        let normalized_severity =
            ((answer.score() / (question.n_answers() - 1.0)) * 10.0).trunc() / 10.0;
        let severity = SeverityTerms::from_numerical_severity(normalized_severity)?.as_term();
        Ok(severity)
    }

    fn get_diagnosis(
        &self,
        total_score: f32,
        taken_at: Option<&DateTime<Utc>>,
    ) -> Result<Option<Condition>, RorschachError> {
        let (_, diagnosis) = self
            .interpretation
            .range(..=total_score.ceil() as i32)
            .next_back()
            .ok_or(RorschachError::NoMatchingDiagnosis {
                found_score: total_score,
            })?;

        let mut diagnosis = diagnosis.clone();
        if let Some(diagnosis) = diagnosis.as_mut() {
            self.set_time(diagnosis, taken_at);
        }

        Ok(diagnosis)
    }

    fn set_time(&self, condition: &mut Condition, taken_at: Option<&DateTime<Utc>>) {
        if let (Some(taken), Some(recall)) = (taken_at, self.recall_period) {
            condition.set_time(&(*taken - recall), taken);
        }
    }
}

impl PartialEq for Questionnaire {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.items == other.items
            && self.interpretation == other.interpretation
    }
}

#[cfg(test)]
mod tests {
    use crate::answer::Answer;
    use crate::questionnaire::Questionnaire;
    use crate::questionnaire_item::QuestionnaireItem;
    use crate::score_calculations::sum_score::SumScore;
    use crate::term::{PhenotypeTerms, SeverityTerms, Term};

    #[test]
    fn test_new() {
        let questionnaire =
            Questionnaire::new("".to_string(), vec![], Default::default(), SumScore, None);

        assert!(questionnaire.recall_period.is_none());
    }

    #[test]
    fn test_calculate_severity() {
        let answer = Answer::new(0, 3.0);
        let q = QuestionnaireItem::new(None, PhenotypeTerms::AbnormalEatingBehavior, 4.0);
        let severity = Questionnaire::calculate_severity(&answer, &q).unwrap();

        assert_eq!(severity, Term::from(SeverityTerms::Profound));
    }
}
