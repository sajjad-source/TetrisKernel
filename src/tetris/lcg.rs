pub struct LCG {
    state: usize,
    a: usize,
    c: usize,
    m: usize,
}

impl LCG {
    pub fn new(seed: usize) -> LCG {
        LCG {
            state: seed,
            a: 1664525,
            c: 1013904223,
            m: 2usize.pow(32),
        }
    }

    pub fn next(&mut self) -> usize {
        self.state = (self.a * self.state + self.c) % self.m;
        self.state
    }
}
