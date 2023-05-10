use crate::cmos::read_cmos;

#[derive(Clone)]
pub struct GameScore {
    pub score: usize,
    pub level: usize,
    
}

impl GameScore {
    pub fn new() -> GameScore {
        GameScore { score: 0, level: 0 }
    }

    pub fn get_time(&mut self) -> usize {
        29
    }
}