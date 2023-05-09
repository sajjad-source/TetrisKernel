use crate::tetris::tetlib::{new_piece,init};
use crate::tetris::{gamescore::GameScore, tetrominoe::Tetrominoe};
use crate::tetris::game::{WIDTH, HEIGHT};

#[derive(Clone)]
pub struct GameState {
    pub display: [[char; WIDTH]; HEIGHT],
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
            active_piece: Tetrominoe::new(),
            gamescore: GameScore::new(),
            hold_piece: None,
            next_piece: Tetrominoe::random(3),
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