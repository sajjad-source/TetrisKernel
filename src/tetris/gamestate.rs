use crate::tetris::game::{HEIGHT, WIDTH};
use crate::tetris::tetlib::{init, new_piece};
use crate::tetris::{gamescore::GameScore, tetrominoe::Tetrominoe};

#[derive(Clone, PartialEq)]
pub struct GameState {
    pub display: [[Tetrominoe; WIDTH]; HEIGHT],
    pub active_piece: Tetrominoe,
    pub gamescore: GameScore,
    pub hold_piece: Option<Tetrominoe>,
    pub next_piece: Tetrominoe,
    pub counter: usize,
    pub is_game_over: bool,
}

impl GameState {
    pub fn new() -> Self {
        let mut gs = GameState {
            display: init(),
            active_piece: Tetrominoe::new(None, None),
            gamescore: GameScore::new(),
            hold_piece: None,
            next_piece: Tetrominoe::random(),
            counter: 0,
            is_game_over: false,
        };
        init();
        new_piece(
            &mut gs.display,
            &mut gs.active_piece,
            None,
            &mut gs.next_piece,
        );
        gs
    }
}
