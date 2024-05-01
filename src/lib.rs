#[derive(Debug)]
pub struct HighScores {
    scores: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        let sorted_scores = scores.to_vec(); // Copy scores into a vector
        HighScores{scores: sorted_scores}
        //unimplemented!("Construct a HighScores struct, given the scores: {scores:?}")
    }
    pub fn scores(&self) -> &[u32] {
        &self.scores

        //unimplemented!("Return all the scores as a slice")
    }
    pub fn latest(&self) -> Option<u32> {

        self.scores.last().copied()


        //unimplemented!("Return the latest (last) score")
    }
    pub fn personal_best(&self) -> Option<u32> {
        let mut sorted_scores = self.scores.clone();
        sorted_scores.sort_unstable_by(|a, b| b.cmp(a));

        sorted_scores.first().copied()

        //unimplemented!("Return the highest score")
    }
    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut sorted_scores = self.scores.clone();
        sorted_scores.sort_unstable_by(|a, b| b.cmp(a));

        sorted_scores.iter().take(3).copied().collect()

        //unimplemented!("Return 3 highest scores")
    }
}
