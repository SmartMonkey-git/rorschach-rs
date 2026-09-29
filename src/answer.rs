use chrono::{DateTime, Utc};

pub struct QuestionnaireResponse {
    answers: Vec<Answer>,
    taken_at: Option<DateTime<Utc>>,
}

impl QuestionnaireResponse {
    pub fn new(answers: Vec<Answer>, taken_at: Option<DateTime<Utc>>) -> QuestionnaireResponse {
        QuestionnaireResponse { answers, taken_at }
    }

    pub fn answers(&self) -> &[Answer] {
        &self.answers
    }
    pub fn taken_at(&self) -> Option<&DateTime<Utc>> {
        self.taken_at.as_ref()
    }
}

#[derive(Debug)]
pub struct Answer {
    idx: usize,
    score: Option<f32>,
}

impl Answer {
    pub fn new(idx: usize, score: Option<f32>) -> Self {
        Answer { idx, score }
    }
    pub fn idx(&self) -> usize {
        self.idx
    }

    pub fn score(&self) -> Option<f32> {
        self.score
    }
}
