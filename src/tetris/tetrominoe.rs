use crate::tetris::tetlib::EMP;
use crate::tetris::game::{WIDTH, HEIGHT};
use crate::random::rand;

#[derive(Clone)]
pub struct Tetrominoe {
    pub shape: [[char; 4]; 4],
    pub row: usize,
    pub col: usize,
    pub ptype: char,
    state: usize,
}

impl Tetrominoe {
    pub fn new() -> Tetrominoe {
        Tetrominoe {
            shape: [[EMP; 4]; 4],
            row: 0,
            col: 0,
            ptype: ' ',
            state: 0,
        }
    }

    pub fn set(&mut self, shape: char) -> &mut Self {
        self.ptype = shape;
        let shape = match shape {
            'I' => [
                [EMP, 'a', EMP, EMP],
                [EMP, 'a', EMP, EMP],
                [EMP, 'a', EMP, EMP],
                [EMP, 'a', EMP, EMP],
            ],

            'J' => [
                [EMP, 'a', EMP, EMP],
                [EMP, 'a', EMP, EMP],
                ['a', 'a', EMP, EMP],
                [EMP, EMP, EMP, EMP],
            ],

            'L' => [
                [EMP, 'a', EMP, EMP],
                [EMP, 'a', EMP, EMP],
                [EMP, 'a', 'a', EMP],
                [EMP, EMP, EMP, EMP],
            ],

            'O' => [
                [EMP, EMP, EMP, EMP],
                [EMP, 'a', 'a', EMP],
                [EMP, 'a', 'a', EMP],
                [EMP, EMP, EMP, EMP],
            ],

            'Z' => [
                [EMP, EMP, EMP, EMP],
                ['a', 'a', EMP, EMP],
                [EMP, 'a', 'a', EMP],
                [EMP, EMP, EMP, EMP],
            ],

            'T' => [
                [EMP, EMP, EMP, EMP],
                [EMP, 'a', EMP, EMP],
                ['a', 'a', 'a', EMP],
                [EMP, EMP, EMP, EMP],
            ],

            'S' => [
                [EMP, EMP, EMP, EMP],
                [EMP, 'a', 'a', EMP],
                ['a', 'a', EMP, EMP],
                [EMP, EMP, EMP, EMP],
            ],

            _ => panic!("Unknown shape: {}", shape),
        };
        self.shape = shape;
        self.state = 0;
        self
    }

    pub fn set_pos(&mut self, row: usize, col: usize) {
        self.row = row;
        self.col = col;
    }

    pub fn rotate(&mut self) {
        match self.ptype {
            'O' => (),
            'I' | 'J' | 'L' | 'T' => {
                // transpose or swap rows and columns
                let n = self.shape.len();
                for i in 0..n {
                    for j in i..n {
                        let temp = self.shape[i][j];
                        self.shape[i][j] = self.shape[j][i];
                        self.shape[j][i] = temp;
                    }
                }

                // reverse each row to rotate
                for i in 0..n {
                    self.shape[i].reverse();
                }
            }

            'Z' => {
                if self.state == 0 {
                    self.shape = [
                        [EMP, EMP, EMP, EMP],
                        [EMP, EMP, 'a', EMP],
                        [EMP, 'a', 'a', EMP],
                        [EMP, 'a', EMP, EMP],
                    ];
                    self.state = 1;
                } else {
                    self.shape = [
                        [EMP, EMP, EMP, EMP],
                        ['a', 'a', EMP, EMP],
                        [EMP, 'a', 'a', EMP],
                        [EMP, EMP, EMP, EMP],
                    ];
                    self.state = 0;
                }
            }

            'S' => {
                if self.state == 0 {
                    self.shape = [
                        [EMP, EMP, EMP, EMP],
                        [EMP, 'a', EMP, EMP],
                        [EMP, 'a', 'a', EMP],
                        [EMP, EMP, 'a', EMP],
                    ];

                    self.state = 1;
                } else {
                    self.shape = [
                        [EMP, EMP, EMP, EMP],
                        [EMP, 'a', 'a', EMP],
                        ['a', 'a', EMP, EMP],
                        [EMP, EMP, EMP, EMP],
                    ];
                    self.state = 0;
                }
            }

            _ => panic!("Unknown shape: {}", self.ptype),
        }
    }

    pub fn from(ptype: char) -> Tetrominoe {
        Tetrominoe::new().set(ptype).clone()
    }

    pub fn random(seed: usize) -> Tetrominoe {
        let ptype = match rand() % 7 {
            0 => 'I',
            1 => 'J',
            2 => 'L',
            3 => 'O',
            4 => 'Z',
            5 => 'T',
            6 => 'S',
            _ => panic!("Invalid random number"),
        };
        Tetrominoe::from(ptype)
    }
}
