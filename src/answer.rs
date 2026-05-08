#[derive(Debug)]
pub struct Answer {
    idx: usize,
    score: f32, //TODO: Should be option, because a proband can just skip questions.
}

impl Answer {
    pub fn new(idx: usize, score: f32) -> Self {
        Answer { idx, score }
    }
    pub fn idx(&self) -> usize {
        self.idx
    }

    pub fn score(&self) -> f32 {
        self.score
    }
}
