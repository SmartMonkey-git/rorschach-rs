use crate::answer::Answer;
use crate::error::RorschachError;
use crate::phenotype::Phenotype;
use crate::question::Question;
use crate::questionnaire_result::QuestionnaireResult;
use std::collections::BTreeMap;

struct Questionnaire {
    name: String,
    questions: Vec<Question>,
    max_score: usize,
    interpretation: BTreeMap<usize, Phenotype>,
    date: Option<String>,
}

impl Questionnaire {
    pub fn new(
        name: String,
        questions: Vec<Question>,
        max_score: usize,
        interpretation: BTreeMap<usize, Phenotype>,
        date: Option<String>,
    ) -> Questionnaire {
        Self {
            name,
            questions,
            max_score,
            interpretation,
            date,
        }
    }

    pub fn evaluate(&self, answers: Vec<Answer>) -> Result<QuestionnaireResult, RorschachError> {
        if answers.len() != self.questions.len() {
            return Err(RorschachError::AnswerQuestionMismatch(
                self.name.to_string(),
                self.questions.len(),
                answers.len(),
            ));
        }

        let total_score = answers.iter().fold(0, |acc, answer| acc + answer.score());

        if total_score > self.max_score {
            return Err(RorschachError::TotalScoreTooHigh(
                self.name.to_string(),
                self.max_score,
                total_score,
            ));
        }

        let (_, diagnosis) = self
            .interpretation
            .range(..=total_score)
            .next_back()
            .expect("Score should fit.");

        let mut result = QuestionnaireResult::new(diagnosis.clone());

        for (answer, question) in answers.into_iter().zip(self.questions.iter()) {
            let phenotypes = question.answer(answer.score());
            result.push_phenotypes(phenotypes);
        }

        Ok(result)
    }
}
