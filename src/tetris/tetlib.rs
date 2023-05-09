use crate::tetris::game::{HEIGHT, WIDTH};
use crate::tetris::gamestate::GameState;
use crate::tetris::tetrominoe::Tetrominoe;
use crate::vga_buffer::{change_color, clear_screen, Color};
use crate::{print, println};
use crate::vga_buffer::WRITER;

use crate::keyboard::getch;

pub const EMP: char = '.';

pub fn render(gs: &mut GameState, is_updated: bool) {
    if !is_updated {
        return;
    }

    clear_screen();
    let _width: u16 = gs.display[0].len() as u16;

    for (_c, row) in gs.display.iter().enumerate() {
        for ch in row {
            match ch {
                &EMP => {
                    print!(" .");
                }
                'a' => {
                    print!("[]");
                }
                'l' => {
                    print!("[]");
                }
                'g' => {
                    change_color(Color::DarkGray);
                    print!("//");
                    change_color(Color::White);
                }

                _ => panic!("unknown character: {}", ch),
            }
        }
        println!();
    }
    WRITER.lock().flush();
}

pub fn init() -> [[char; WIDTH]; HEIGHT] {
    let display: [[char; WIDTH]; HEIGHT] = [[EMP; WIDTH]; HEIGHT];
    display
}

pub fn gravity(gs: &mut GameState) -> bool {
    let prev_display = gs.display.clone();
    for row in (0..gs.display.len()).rev() {
        for col in 0..gs.display[row].len() {
            if gs.display[row][col] == 'a' {
                if row == gs.display.len() - 1 || gs.display[row + 1][col] == 'l' {
                    gs.display = prev_display;
                    landed(gs);
                    let game_over = new_piece(gs, Some(gs.next_piece.ptype));
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

pub fn handle_input(gs: &mut GameState, key: char) {
    let prev_display = gs.display.clone();
    match key {
        'l' => {
            for row in (0..gs.display.len()).rev() {
                for col in 0..gs.display[row].len() {
                    if gs.display[row][col] == 'a' {
                        if col == 0 || gs.display[row][col - 1] == 'l' {
                            gs.display = prev_display;
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
                            gs.display = prev_display;
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
            while gs.display[0][gs.display[0].len() / 2] == EMP {
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

            for row in gs.active_piece.row..gs.active_piece.row + 4 {
                for col in gs.active_piece.col..gs.active_piece.col + 4 {
                    if gs.display[row][col] == 'l' {
                        gs.display = prev_display;
                        gs.active_piece = prev_piece;
                        return;
                    }

                    if gs.active_piece.shape[row - gs.active_piece.row][col - gs.active_piece.col]
                        == 'a'
                    {
                        gs.display[row][col] = gs.active_piece.shape[row - gs.active_piece.row]
                            [col - gs.active_piece.col];
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

    let piece = desired_piece.unwrap_or_else(|| get_next_piece(&mut gs.next_piece));
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
    for row in &mut gs.display {
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
        gs.display.insert(0, [EMP; WIDTH]); // add new line at the top
    }

    match lines {
        1 => gs.gamescore.score += 40 * (gs.gamescore.level + 1),
        2 => gs.gamescore.score += 100 * (gs.gamescore.level + 1),
        3 => gs.gamescore.score += 300 * (gs.gamescore.level + 1),
        4 => gs.gamescore.score += 1200 * (gs.gamescore.level + 1),
        _ => (),
    }

    gs.gamescore.level = gs.gamescore.score / 1000;
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
    let mut active_piece = gs.active_piece.clone();

    gravity_until_new_piece(gs);

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
    gravity(gs);
    while gs.display[0][gs.display[0].len() / 2] == EMP {
        prev_display = gs.display.clone();
        gravity(gs);
    }
    gs.display = prev_display;
}

pub fn get_input() -> char {
    if let Some(key) = getch(&mut 0u8) {
        match key {
            'q' => return 'q', // quit
            ' ' => return 's', // hard drop
            'c' => return 'c', // hold
            'p' => return 'p', // pause
            'i' => return 'u', // rotate clockwise
            'k' => return 'd', // soft drop
            'j' => return 'l', // move left
            'l' => return 'r', // move right
            _ => return ' ',
        }
    } else {
        return ' ';
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
    if let Some(hold) = &gs.hold_piece {
        let prev_piece = gs.active_piece.clone();
        new_piece(gs, Some(hold.ptype));
        gs.hold_piece = Some(prev_piece);
    } else {
        gs.hold_piece = Some(gs.active_piece.clone());
        new_piece(gs, None);
    }
}

fn get_next_piece(next_piece: &mut Tetrominoe) -> char {
    let temp = next_piece.ptype;
    *next_piece = Tetrominoe::random(next_piece.col);
    temp
}

trait Remove {
    fn remove(&mut self, index: usize) -> Self;
}

trait Insert<T> {
    fn insert(&mut self, index: usize, item: T) -> Result<(), &'static str>;
}

impl<T: Clone + Copy, const N: usize> Remove for [T; N] {
    fn remove(&mut self, index: usize) -> Self {
        let mut temp = self.clone();
        temp[index] = temp[N - 1];
        temp[N - 1] = self[index];
        temp
    }
}

impl<T, const N: usize> Insert<T> for [T; N]
where
    T: Copy + Default,
{
    fn insert(&mut self, index: usize, item: T) -> Result<(), &'static str> {
        if index > N {
            return Err("Index out of bounds");
        }

        if index < N - 1 {
            for i in (index + 1..N).rev() {
                self[i] = self[i - 1];
            }
        }

        self[index] = item;
        Ok(())
    }
}
