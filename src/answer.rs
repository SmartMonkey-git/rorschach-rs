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
