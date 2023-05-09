#[derive(Clone)]
pub struct GameState {
    pub display: [[char; 10]; 20],
    pub active_piece: Tetrominoe,
    pub gamescore: GameScore,
    pub hold_piece: Option<Tetrominoe>,
    pub next_piece: Tetrominoe,
    pub counter: usize,
    pub is_game_over: bool,
}

impl GameState {
    pub fn new(width: usize, height: usize) -> Self {
        let mut gs = GameState {
            display: init(width, height),
            active_piece: Tetrominoe::new(),
            gamescore: GameScore::new(),
            hold_piece: None,
            next_piece: Tetrominoe::random(),
            counter: 0,
            is_game_over: false,
        };
        init(width, height);
        new_piece(
            &mut gs.display,
            &mut gs.active_piece,
            None,
            &mut gs.next_piece,
        );
        gs
    }
}
