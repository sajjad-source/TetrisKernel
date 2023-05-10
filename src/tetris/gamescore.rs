use crate::cmos::get_time;

#[derive(Clone, PartialEq)]
pub struct GameScore {
    pub score: usize,
    pub level: usize,
    elapsed_time: usize,
}

impl GameScore {
    pub fn new() -> GameScore {
        GameScore { score: 0, level: 0, elapsed_time: get_time() }
    }

    pub fn get_time(&mut self) -> usize {
        get_time() - self.elapsed_time
    }
}