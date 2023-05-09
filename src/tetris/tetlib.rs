use crate::gamescore::GameScore;
use crate::tetrominoe::Tetrominoe;
use crate::WIDTH;
use crate::HEIGHT;

pub const EMP: char = '.';

pub fn render(gs: &mut GameState) {
    if !is_updated {
        return;
    }

    let width: u16 = gs.display[0].len() as u16;

    for (c, row) in gs.display.iter().enumerate() {
        for ch in row {
            match ch {
                &EMP => {
                    stdout.queue(Print(" .")).unwrap();
                }
                'a' => {
                    stdout.queue(Print("[]")).unwrap();
                }
                'l' => {
                    stdout.queue(Print("[]")).unwrap();
                }
                'g' => {
                    stdout
                        .queue(SetForegroundColor(Color::Rgb {
                            r: 50,
                            g: 50,
                            b: 50,
                        }))
                        .unwrap()
                        .queue(Print("//"))
                        .unwrap()
                        .queue(ResetColor)
                        .unwrap();
                }

                _ => panic!("unknown character: {}", ch),
            }
        }
        stdout.queue(MoveTo(width + 3, (c + 2) as u16)).unwrap();
    }
}

pub fn init(width: usize, height: usize) -> [[char; WIDTH]; HEIGHT] {
    let mut display: [[char; width]; height] = [[EMP; width]; height];
    display
}

pub fn gravity(gs: &mut GameScore) -> bool {
    let prev_display = display.clone();
    for row in (0..gs.display.len()).rev() {
        for col in 0..gs.display[row].len() {
            if gs.display[row][col] == 'a' {
                if row == gs.display.len() - 1 || gs.display[row + 1][col] == 'l' {
                    *gs.display = prev_display;
                    landed(gs);
                    let game_over = new_piece(gs, None);
                    return game_over;
                }

                gs.display[row][col] = EMP;
                gs.display[row + 1][col] = 'a';
            }
        }
    }
    gs.active_piece.row += 1;
    false
}

pub fn handle_input(gs: &mut GameScore, key: char) {
    let prev_display = gs.display.clone();
    match key {
        'l' => {
            for row in (0..gs.display.len()).rev() {
                for col in 0..gs.display[row].len() {
                    if gs.display[row][col] == 'a' {
                        if col == 0 || gs.display[row][col - 1] == 'l' {
                            *gs.display = prev_display;
                            return;
                        }
                        gs.display[row][col] = EMP;
                        gs.display[row][col - 1] = 'a';
                    }
                }
            }

            if gs.active_piece.col > 0 {
                gs.active_piece.col -= 1;
            }
        }

        'r' => {
            for row in (0..gs.display.len()).rev() {
                for col in (0..gs.display[row].len()).rev() {
                    if gs.display[row][col] == 'a' {
                        if col == gs.display[row].len() - 1 || gs.display[row][col + 1] == 'l' {
                            *gs.display = prev_display;
                            return;
                        }
                        gs.display[row][col] = EMP;
                        gs.display[row][col + 1] = 'a';
                    }
                }
            }
            gs.active_piece.col += 1;
        }

        's' => {
            // bring down piece until new piece is created
            while gs.display[0][display[0].len() / 2] == EMP {
                gravity(gs);
            }
        }

        'd' => {
            gravity(gs);
        }

        'u' => {
            // let prev_display = display.clone();
            let prev_piece = gs.active_piece.clone();

            // rotate piece
            gs.active_piece.rotate();
            if gs.active_piece.row + 4 > gs.display.len() {
                gs.active_piece.row = gs.display.len() - 4;
            }

            if gs.active_piece.col + 4 > gs.display[0].len() {
                gs.active_piece.col = gs.display[0].len() - 4;
            }

            // clear piece and replace with new rotated piece
            for row in 0..gs.display.len() {
                for col in 0..gs.display[row].len() {
                    if gs.display[row][col] == 'a' {
                        gs.display[row][col] = EMP;
                    }
                }
            }

            for row in gs.active_piece.row..active_piece.row + 4 {
                for col in gs.active_piece.col..active_piece.col + 4 {
                    if gs.display[row][col] == 'l' {
                        *gs.display = prev_display;
                        *gs.active_piece = prev_piece;
                        return;
                    }

                    if gs.active_piece.shape[row - gs.active_piece.row][col - gs.active_piece.col] == 'a' {
                        gs.display[row][col] =
                        gs.active_piece.shape[row - gs.active_piece.row][col - gs.active_piece.col];
                    }
                }
            }
        }

        _ => (),
    }
}

pub fn new_piece(gs: &mut GameState, desired_piece: Option<char>) -> bool {
    let half_width = gs.display[0].len() / 2;

    // game over
    if gs.display[0][half_width] != EMP {
        return true;
    }

    let piece = desired_piece.unwrap_or_else(|| get_next_piece(next_piece));
    match piece {
        'I' => {
            // I
            // I
            // I
            // I
            gs.display[0][half_width] = 'a';
            gs.display[1][half_width] = 'a';
            gs.display[2][half_width] = 'a';
            gs.display[3][half_width] = 'a';
        }
        'J' => {
            //  J
            //  J
            // JJ
            gs.display[0][half_width] = 'a';
            gs.display[1][half_width] = 'a';
            gs.display[2][half_width] = 'a';
            gs.display[2][half_width - 1] = 'a';
        }
        'L' => {
            // L
            // L
            // LL
            gs.display[0][half_width] = 'a';
            gs.display[1][half_width] = 'a';
            gs.display[2][half_width] = 'a';
            gs.display[2][half_width + 1] = 'a';
        }
        'O' => {
            // OO
            // OO
            gs.display[0][half_width] = 'a';
            gs.display[0][half_width + 1] = 'a';
            gs.display[1][half_width] = 'a';
            gs.display[1][half_width + 1] = 'a';
        }
        'S' => {
            // SS
            //  SS
            gs.display[0][half_width] = 'a';
            gs.display[0][half_width + 1] = 'a';
            gs.display[1][half_width - 1] = 'a';
            gs.display[1][half_width] = 'a';
        }
        'T' => {
            // T
            // TT
            // T
            gs.display[0][half_width] = 'a';
            gs.display[1][half_width - 1] = 'a';
            gs.display[1][half_width] = 'a';
            gs.display[1][half_width + 1] = 'a';
        }
        'Z' => {
            //  ZZ
            // ZZ
            gs.display[0][half_width - 1] = 'a';
            gs.display[0][half_width] = 'a';
            gs.display[1][half_width] = 'a';
            gs.display[1][half_width + 1] = 'a';
        }
        _ => panic!("unknown picece: {}", piece),
    }
    gs.active_piece.set(piece);
    gs.active_piece.set_pos(0, half_width - 1);
    false
}

pub fn landed(gs: &mut GameState) {
    for row in gs.display {
        for ch in row {
            if *ch == 'a' {
                *ch = 'l';
            }
        }
    }
}

pub fn full_line(gs: &mut GameState) {
    let mut lines: usize = 0;
    'outer: for row in (0..gs.display.len()).rev() {
        for ch in &gs.display[row] {
            if *ch != 'l' {
                continue 'outer;
            }
        }
        gs.display.remove(row);
        lines += 1;
    }

    for _ in 0..lines {
        gs.display.insert(0, vec![EMP; gs.display[0].len()]); // add new line at the top
    }

    match lines {
        1 => gs.score.score += 40 * (gs.score.level + 1),
        2 => gs.score.score += 100 * (gs.score.level + 1),
        3 => gs.score.score += 300 * (gs.score.level + 1),
        4 => gs.score.score += 1200 * (gs.score.level + 1),
        _ => (),
    }

    score.level = score.score / 1000;
}

pub fn ghost_piece(gs: &mut GameState) {
    for row in 0..gs.display.len() {
        for col in 0..gs.display[row].len() {
            if gs.display[row][col] == 'g' {
                gs.display[row][col] = EMP;
            }
        }
    }

    let mut ghost = gs.display.clone();
    let mut active_piece = active_piece.clone();

    gravity_until_new_piece(&mut ghost, &mut active_piece);

    for row in 0..ghost.len() {
        for col in 0..ghost[row].len() {
            if ghost[row][col] == 'a' && gs.display[row][col] == EMP {
                gs.display[row][col] = 'g';
            }
        }
    }
}

fn gravity_until_new_piece(gs: &mut GameState) {
    let mut prev_display = gs.display.clone();
    gravity(gs, &mut Tetrominoe::random());
    while gs.display[0][gs.display[0].len() / 2] == EMP {
        prev_display = gs.display.clone();
        gravity(gs, &mut Tetrominoe::random());
    }
    *gs.display = prev_display;
}

pub fn get_input() -> char {
    loop {
        if poll(Duration::from_millis(0)).unwrap() {
            let input = event::read().unwrap();
            match input {
                Event::Key(KeyEvent {
                    code: KeyCode::Char('q'),
                    kind: KeyEventKind::Press,
                    ..
                }) => return 'q', // quit
                Event::Key(KeyEvent {
                    code: KeyCode::Char(' '),
                    kind: KeyEventKind::Press,
                    ..
                }) => return 's', // hard drop
                Event::Key(KeyEvent {
                    code: KeyCode::Char('c'),
                    kind: KeyEventKind::Press,
                    ..
                }) => return 'c', // hold
                Event::Key(KeyEvent {
                    code: KeyCode::Char('p'),
                    kind: KeyEventKind::Press,
                    ..
                }) => return 'p', // pause
                Event::Key(KeyEvent {
                    code: KeyCode::Left,
                    kind: KeyEventKind::Press,
                    ..
                }) => return 'l', // move left
                Event::Key(KeyEvent {
                    code: KeyCode::Right,
                    kind: KeyEventKind::Press,
                    ..
                }) => return 'r', // move right
                Event::Key(KeyEvent {
                    code: KeyCode::Up,
                    kind: KeyEventKind::Press,
                    ..
                }) => return 'u', // rotate clockwise
                Event::Key(KeyEvent {
                    code: KeyCode::Down,
                    kind: KeyEventKind::Press,
                    ..
                }) => return 'd', // soft drop
                _ => (),
            }
        } else {
            return ' ';
        }
    }
}

pub fn hold(gs: &mut GameState) {
    // clear piece
    for row in gs.display.iter_mut() {
        for col in row.iter_mut() {
            if *col == 'a' {
                *col = EMP;
            }
        }
    }

    // hold piece
    if let Some(hold) = hold_piece {
        let prev_piece = active_piece.clone();
        new_piece(gs);
        *hold_piece = Some(prev_piece);
    } else {
        *hold_piece = Some(active_piece.clone());
        new_piece(gs);
    }
}

fn get_next_piece(next_piece: &mut Tetrominoe) -> char {
    let temp = next_piece.ptype;
    *next_piece = Tetrominoe::random();
    temp
}
