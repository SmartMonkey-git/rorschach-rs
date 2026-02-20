pub struct Answer {
    idx: usize,
    score: usize,
}

impl Answer {
    pub fn new(idx: usize, score: usize) -> Self {
        Answer { idx, score }
    }
    pub fn idx(&self) -> usize {
        self.idx
    }

    pub fn score(&self) -> usize {
        self.score
    }
}
