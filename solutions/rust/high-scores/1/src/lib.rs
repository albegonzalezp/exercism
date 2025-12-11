#[derive(Debug)]
pub struct HighScores<'a>{
    scores: &'a [u32]
}

impl <'a>HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        if scores.len() > 0 {
            return HighScores { scores }
        }
        HighScores { scores: &[] }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        match self.scores.last() {
            Some(x) => return Some(*x),
            None => return None
        }
    }

    pub fn personal_best(&self) -> Option<u32> {
        if self.scores.len() == 1 {
            return Some(self.scores[0]);
        }
        let mut max: u32 = 0;
        self.scores.iter().for_each(|x: &u32| {if *x > max {
            max = *x
        }});

        if max > 0 {
            return Some(max)
        } else {
            return None
        }
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut result: Vec<u32> = self.scores().to_vec();
        result.sort();

        if result.len() < 3 {
            result.reverse();
            return result
        }
        

       let mut new_result: Vec<u32> = result[(result.len()-3)..].to_vec();
       new_result.reverse();
       new_result
    }
}