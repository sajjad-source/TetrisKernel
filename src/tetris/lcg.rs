pub struct LCG {
    state: u32,
    a: u32,
    c: u32,
    m: u32,
}

impl LCG {
    pub fn new(seed: u32) -> LCG {
        LCG {
            state: seed,
            a: 1664525,
            c: 1013904223,
            m: 2u32.pow(32),
        }
    }

    pub fn next(&mut self) -> u32 {
        self.state = (self.a * self.state + self.c) % self.m;
        self.state
    }
}